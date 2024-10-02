// SPDX-License-Identifier: MIT

pragma solidity ^0.8.26;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Address.sol";
import {IPool} from "../lib/aave-v3-core/contracts/interfaces/IPool.sol";
import {DataTypes} from "../lib/aave-v3-core/contracts/protocol/libraries/types/DataTypes.sol";
import {IAaveOracle} from "../lib/aave-v3-core/contracts/interfaces/IAaveOracle.sol";
import {IPoolAddressesProvider} from "../lib/aave-v3-core/contracts/interfaces/IPoolAddressesProvider.sol";
import {IUniswapV3Pool} from "../lib/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import {IUniswapV3Factory} from "../lib/v3-core/contracts/interfaces/IUniswapV3Factory.sol";
import "./ImmutableOwnable.sol";
import {console2} from "forge-std/console2.sol";

interface ISwapRouter {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    struct ExactOutputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountOut;
        uint256 amountInMaximum;
        uint160 sqrtPriceLimitX96;
    }

    function exactInputSingle(ExactInputSingleParams calldata params) external payable returns (uint256 amountOut);
    function exactOutputSingle(ExactOutputSingleParams calldata params) external payable returns (uint256 amountIn);
}

/**
 * Single asset leveraged reborrowing strategy on AAVE, chain agnostic.
 * Position managed by this contract, with full ownership and control by Owner.
 * Monitor position health to avoid liquidation.
 */
contract AaveLooper is ImmutableOwnable {
    using SafeERC20 for ERC20;

    uint256 public constant USE_VARIABLE_DEBT = 2;
    uint256 public constant SAFE_BUFFER = 100; // wei

    IPool public immutable LENDING_POOL; // solhint-disable-line
    address public immutable UNISWAP_V3_ROUTER;
    address public immutable UNISWAP_V3_FACTORY;
    address public constant AGGREGATION_ROUTER_V5 = 0x1111111254EEB25477B68fb85Ed929f73A960582;

    uint24[] public FEE_TIERS = [100, 500, 3000, 10000];

    address public immutable WETH_ADDRESS;

    struct SwapParams {
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint256 minReturnAmount;
        address payable recipient;
        uint256 flags;
        bytes permit;
        bytes data;
    }

    /**
     * @param owner The contract owner, has complete ownership, immutable
     * @param lendingPool The deployed AAVE ILendingPool
     */
    constructor(address owner, address lendingPool, address wethAddress, address uniswapV3Router)
        ImmutableOwnable(owner)
    {
        require(lendingPool != address(0), "address 0");
        require(uniswapV3Router != address(0), "address 0");
        LENDING_POOL = IPool(lendingPool);
        WETH_ADDRESS = wethAddress;
        UNISWAP_V3_ROUTER = uniswapV3Router;
    }

    // ---- views ----

    function getDerivedAssets(address asset) public view returns (address[] memory assets) {
        DataTypes.ReserveData memory data = LENDING_POOL.getReserveData(asset);
        assets = new address[](2);
        assets[0] = data.aTokenAddress;
        assets[1] = data.variableDebtTokenAddress;
    }

    /**
     * @return The price in ETH according to Aave PriceOracle
     */
    function getAssetPrice(address asset) public view returns (uint256) {
        IPoolAddressesProvider provider = IPoolAddressesProvider(LENDING_POOL.ADDRESSES_PROVIDER());
        return IAaveOracle(provider.getPriceOracle()).getAssetPrice(asset);
    }

    /**
     * @return total supply balance
     */
    function getSupplyBalance(address asset) public view returns (uint256) {
        (uint256 totalCollateralETH,,,,,) = getPositionData();
        uint256 decimals = ERC20(asset).decimals();
        return (totalCollateralETH * (10 ** decimals)) / getAssetPrice(asset);
    }

    /**
     * @return total borrow balance
     */
    function getBorrowBalance(address asset) public view returns (uint256) {
        (, uint256 totalDebtETH,,,,) = getPositionData();
        uint256 decimals = ERC20(asset).decimals();
        return (totalDebtETH * (10 ** decimals)) / getAssetPrice(asset);
    }

    /**
     * @return available liquidity
     */
    function getLiquidity(address asset) public view returns (uint256) {
        (,, uint256 availableBorrowsUSD,,,) = getPositionData();
        uint256 assetPriceUSD = getAssetPrice(asset);
        uint8 assetDecimals = ERC20(asset).decimals();
        return (availableBorrowsUSD * (10 ** assetDecimals) / assetPriceUSD);
    }

    function getAvailableBorrowAmount(address asset) public view returns (uint256) {
        (,, uint256 availableBorrowsUSD,,,) = getPositionData();
        uint256 assetPriceUSD = getAssetPrice(asset);
        uint8 assetDecimals = ERC20(asset).decimals();
        uint256 availableBorrowAmount = (availableBorrowsUSD * (10 ** assetDecimals) / assetPriceUSD);
        return availableBorrowAmount;
    }

    /**
     * @return ASSET balanceOf(this)
     */
    function getAssetBalance(address asset) public view returns (uint256) {
        return ERC20(asset).balanceOf(address(this));
    }

    /**
     * Position data from Aave
     */
    function getPositionData()
        public
        view
        returns (
            uint256 totalCollateral,
            uint256 totalDebt,
            uint256 availableBorrows,
            uint256 currentLiquidationThreshold,
            uint256 ltv,
            uint256 healthFactor
        )
    {
        return LENDING_POOL.getUserAccountData(address(this));
    }

    /**
     * @return LTV of ASSET in 4 decimals ex. 82.5% == 8250
     */
    function getLTV(address asset) public view returns (uint256) {
        DataTypes.ReserveConfigurationMap memory config = LENDING_POOL.getConfiguration(asset);
        return config.data & 0xffff; // bits 0-15 in BE
    }

    // ---- main ----

    /**
     * @param iterations - Loop count
     * @return Liquidity at end of the loop
     */
    function enterPositionFully(address supplyAsset, address borrowAsset, uint256 iterations, uint24 feeTier)
        external
        onlyOwner
        returns (uint256)
    {
        return enterPosition(supplyAsset, borrowAsset, ERC20(supplyAsset).balanceOf(msg.sender), iterations, feeTier);
    }

    function getBestFeeTier(address factory, address tokenIn, address tokenOut, uint256 amountIn)
        public
        view
        returns (uint24 bestFeeTier, uint256 amountOut)
    {
        for (uint256 i = 0; i < FEE_TIERS.length; i++) {
            address pool = IUniswapV3Factory(factory).getPool(tokenIn, tokenOut, FEE_TIERS[i]);
            if (pool == address(0)) continue;

            (uint160 sqrtPriceX96,,,,,,) = IUniswapV3Pool(pool).slot0();
            uint256 price = uint256(sqrtPriceX96) * uint256(sqrtPriceX96) * 1e18 >> (96 * 2);
            uint256 currentAmountOut = (amountIn * price) / 1e18;

            if (currentAmountOut > amountOut) {
                bestFeeTier = FEE_TIERS[i];
                amountOut = currentAmountOut;
            }
        }
    }

    /**
     * @param principal - ASSET transferFrom sender amount, can be 0
     * @param iterations - Loop count
     * @param feeTier - Fee tier for the swap
     * @return Liquidity at end of the loop
     */
    function enterPosition(
        address supplyAsset,
        address borrowAsset,
        uint256 principal,
        uint256 iterations,
        uint24 feeTier
    ) public onlyOwner returns (uint256) {
        if (principal > 0) {
            ERC20(supplyAsset).transferFrom(msg.sender, address(this), principal);
        }

        if (getAssetBalance(supplyAsset) > 0) {
            _supply(supplyAsset, getAssetBalance(supplyAsset));
        }

        for (uint256 i = 0; i < iterations; i++) {
            _borrow(borrowAsset, getAvailableBorrowAmount(borrowAsset) - SAFE_BUFFER);
            uint256 amountToSwap = getAssetBalance(borrowAsset);
            swapUniswapV3(borrowAsset, supplyAsset, amountToSwap, feeTier);
            _supply(supplyAsset, getAssetBalance(supplyAsset));
        }

        return getLiquidity(supplyAsset);
    }

    function swapUniswapV3(address tokenIn, address tokenOut, uint256 amountIn, uint24 feeTier)
        public
        onlyOwner
        returns (uint256)
    {
        ERC20(tokenIn).approve(UNISWAP_V3_ROUTER, amountIn);

        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter.ExactInputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: feeTier,
            recipient: address(this),
            deadline: block.timestamp + 300, // 5 minutes deadline
            amountIn: amountIn,
            amountOutMinimum: 1,
            sqrtPriceLimitX96: 0
        });

        uint256 amountOut = ISwapRouter(UNISWAP_V3_ROUTER).exactInputSingle(params);
        return amountOut;
    }

    function exitPositionWithFlashLoan(address supplyAsset, address borrowAsset) external onlyOwner returns (uint256) {
        uint256 borrowBalance = getBorrowBalance(borrowAsset);

        // Request a flash loan for the borrow balance
        LENDING_POOL.flashLoanSimple(address(this), borrowAsset, borrowBalance, abi.encode(supplyAsset, borrowAsset), 0);

        // After the flash loan, withdraw any remaining supply and send to owner
        _redeemSupply(supplyAsset, type(uint256).max);
        return _withdrawToOwner(supplyAsset);
    }

    // flash loan callback
    function executeOperation(address asset, uint256 amount, uint256 premium, address initiator, bytes calldata params)
        external
        returns (bool)
    {
        require(msg.sender == address(LENDING_POOL), "Caller must be lending pool");
        require(initiator == address(this), "Initiator must be this contract");

        (address supplyAsset, address borrowAsset) = abi.decode(params, (address, address));

        // Repay the borrowed amount
        _repayBorrow(borrowAsset, amount);

        // Withdraw the supply
        _redeemSupply(supplyAsset, type(uint256).max);

        // Approve the flash loan repayment
        uint256 amountOwed = amount + premium;
        ERC20(asset).approve(address(LENDING_POOL), amountOwed);

        return true;
    }

    /**
     * @param supplyAsset - ASSET to withdraw
     * @param borrowAsset - ASSET to repay
     * @param maxIterations - MAX loop count
     * @return Withdrawn amount of ASSET to OWNER
     */
    function exitPosition(address supplyAsset, address borrowAsset, uint256 maxIterations, uint24 feeTier)
        external
        onlyOwner
        returns (uint256)
    {
        (, uint256 totalDebt,,, uint256 ltv,) = getPositionData(); // 4 decimals
        uint256 iterations = 0;

        console2.log("Total debt:", totalDebt);
        console2.log("LTV:", ltv);

        while (totalDebt > 10000 && iterations < maxIterations) {
            _redeemSupply(supplyAsset, ((getLiquidity(supplyAsset) * 1e4) / ltv) - SAFE_BUFFER);
            swapUniswapV3(supplyAsset, borrowAsset, getAssetBalance(supplyAsset), feeTier);
            _repayBorrow(borrowAsset, getAssetBalance(borrowAsset));
            totalDebt = getBorrowBalance(borrowAsset);
            iterations++;
        }

        if (getBorrowBalance(borrowAsset) == 0) {
            _redeemSupply(supplyAsset, type(uint256).max);
        }

        return _withdrawToOwner(supplyAsset);
    }

    // ---- internals, public onlyOwner in case of emergency ----

    /**
     * amount
     */
    function _supply(address asset, uint256 amount) public onlyOwner {
        ERC20(asset).safeIncreaseAllowance(address(LENDING_POOL), amount);
        LENDING_POOL.deposit(asset, amount, address(this), 0);
    }

    /**
     * amount
     */
    function _borrow(address asset, uint256 amount) public onlyOwner {
        LENDING_POOL.borrow(asset, amount, USE_VARIABLE_DEBT, 0, address(this));
    }

    /**
     * amount
     */
    function _redeemSupply(address asset, uint256 amount) public onlyOwner {
        LENDING_POOL.withdraw(asset, amount, address(this));
    }

    /**
     * amount
     */
    function _repayBorrow(address asset, uint256 amount) public onlyOwner {
        ERC20(asset).safeIncreaseAllowance(address(LENDING_POOL), amount);
        LENDING_POOL.repay(asset, amount, USE_VARIABLE_DEBT, address(this));
    }

    function _withdrawToOwner(address asset) public onlyOwner returns (uint256) {
        uint256 balance = ERC20(asset).balanceOf(address(this));
        ERC20(asset).safeTransfer(OWNER, balance);
        return balance;
    }

    // ---- emergency ----

    function emergencyFunctionCall(address target, bytes memory data) external onlyOwner {
        Address.functionCall(target, data);
    }

    function emergencyFunctionDelegateCall(address target, bytes memory data) external onlyOwner {
        Address.functionDelegateCall(target, data);
    }
}
