// SPDX-License-Identifier: MIT

pragma solidity ^0.8.26;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Address.sol";
import "./IAaveInterfaces.sol";
import "./IAggregatorContractV5.sol";
import "./ImmutableOwnable.sol";

/**
 * Single asset leveraged reborrowing strategy on AAVE, chain agnostic.
 * Position managed by this contract, with full ownership and control by Owner.
 * Monitor position health to avoid liquidation.
 */
contract AaveLooper is ImmutableOwnable {
    using SafeERC20 for ERC20;

    uint256 public constant USE_VARIABLE_DEBT = 2;
    uint256 public constant SAFE_BUFFER = 10; // wei

    ERC20 public immutable ASSET; // solhint-disable-line
    ILendingPool public immutable LENDING_POOL; // solhint-disable-line
    IAaveIncentivesController public immutable INCENTIVES; // solhint-disable-line

    address public constant AGGREGATION_ROUTER_V5 = 0x1111111254EEB25477B68fb85Ed929f73A960582;

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
     * @param incentives The deployed AAVE IAaveIncentivesController
     */
    constructor(address owner, address lendingPool, address incentives) ImmutableOwnable(owner) {
        require(lendingPool != address(0) && incentives != address(0), "address 0");

        // ASSET = ERC20(asset);
        LENDING_POOL = ILendingPool(lendingPool);
        INCENTIVES = IAaveIncentivesController(incentives);
    }

    // ---- views ----

    function getSupplyAndBorrowAssets(address asset) public view returns (address[] memory assets) {
        DataTypes.ReserveData memory data = LENDING_POOL.getReserveData(asset);
        assets = new address[](2);
        assets[0] = data.aTokenAddress;
        assets[1] = data.variableDebtTokenAddress;
    }

    /**
     * @return The ASSET price in ETH according to Aave PriceOracle, used internally for all ASSET amounts calculations
     */
    function getAssetPrice(address asset) public view returns (uint256) {
        return IAavePriceOracle(LENDING_POOL.getAddressesProvider().getPriceOracle()).getAssetPrice(asset);
    }

    /**
     * @return total supply balance in ASSET
     */
    function getSupplyBalance(address asset) public view returns (uint256) {
        (uint256 totalCollateralETH,,,,,) = getPositionData();
        uint256 decimals = ERC20(asset).decimals();
        return (totalCollateralETH * (10 ** decimals)) / getAssetPrice(asset);
    }

    /**
     * @return total borrow balance in ASSET
     */
    function getBorrowBalance(address asset) public view returns (uint256) {
        (, uint256 totalDebtETH,,,,) = getPositionData();
        uint256 decimals = ERC20(asset).decimals();
        return (totalDebtETH * (10 ** decimals)) / getAssetPrice(asset);
    }

    /**
     * @return available liquidity in ASSET
     */
    function getLiquidity(address asset) public view returns (uint256) {
        (,, uint256 availableBorrowsETH,,,) = getPositionData();
        uint256 decimals = ERC20(asset).decimals();
        return (availableBorrowsETH * (10 ** decimals)) / getAssetPrice(asset);
    }

    /**
     * @return ASSET balanceOf(this)
     */
    function getAssetBalance(address asset) public view returns (uint256) {
        return ERC20(asset).balanceOf(address(this));
    }

    /**
     * @return Pending rewards
     */
    function getPendingRewards(address asset) public view returns (uint256) {
        return INCENTIVES.getRewardsBalance(getSupplyAndBorrowAssets(asset), address(this));
    }

    /**
     * Position data from Aave
     */
    function getPositionData()
        public
        view
        returns (
            uint256 totalCollateralETH,
            uint256 totalDebtETH,
            uint256 availableBorrowsETH,
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
    function getLTV() public view returns (uint256) {
        DataTypes.ReserveConfigurationMap memory config = LENDING_POOL.getConfiguration(address(ASSET));
        return config.data & 0xffff; // bits 0-15 in BE
    }

    // ---- unrestricted ----

    /**
     * Claims and transfers all pending rewards to OWNER
     */
    function claimRewardsToOwner(address asset) external {
        INCENTIVES.claimRewards(getSupplyAndBorrowAssets(asset), type(uint256).max, OWNER);
    }

    // ---- main ----

    /**
     * @param iterations - Loop count
     * @return Liquidity at end of the loop
     */
    function enterPositionFully(uint256 iterations) external onlyOwner returns (uint256) {
        return enterPosition(address(ASSET), ASSET.balanceOf(msg.sender), iterations);
    }

    /**
     * @param principal - ASSET transferFrom sender amount, can be 0
     * @param iterations - Loop count
     * @return Liquidity at end of the loop
     */
    function enterPosition(address asset, uint256 principal, uint256 iterations) public onlyOwner returns (uint256) {
        if (principal > 0) {
            ASSET.safeTransferFrom(msg.sender, address(this), principal);
        }

        if (getAssetBalance(asset) > 0) {
            _supply(asset, getAssetBalance(asset));
        }

        for (uint256 i = 0; i < iterations; i++) {
            _borrow(getLiquidity(asset) - SAFE_BUFFER);
            _supply(asset, getAssetBalance(asset));
        }

        return getLiquidity(asset);
    }

    /**
     * @param iterations - MAX loop count
     * @return Withdrawn amount of ASSET to OWNER
     */
    function exitPosition(address asset, uint256 iterations) external onlyOwner returns (uint256) {
        (,,,, uint256 ltv,) = getPositionData(); // 4 decimals

        for (uint256 i = 0; i < iterations && getBorrowBalance(asset) > 0; i++) {
            _redeemSupply(((getLiquidity(asset) * 1e4) / ltv) - SAFE_BUFFER);
            _repayBorrow(getAssetBalance(asset));
        }

        if (getBorrowBalance(asset) == 0) {
            _redeemSupply(type(uint256).max);
        }

        return _withdrawToOwner(address(ASSET));
    }

    // ---- internals, public onlyOwner in case of emergency ----

    /**
     * amount in ASSET
     */
    function _supply(address asset, uint256 amount) public onlyOwner {
        ASSET.safeIncreaseAllowance(address(LENDING_POOL), amount);
        LENDING_POOL.deposit(asset, amount, address(this), 0);
    }

    /**
     * amount in ASSET
     */
    function _borrow(uint256 amount) public onlyOwner {
        LENDING_POOL.borrow(address(ASSET), amount, USE_VARIABLE_DEBT, 0, address(this));
    }

    /**
     * amount in ASSET
     */
    function _redeemSupply(uint256 amount) public onlyOwner {
        LENDING_POOL.withdraw(address(ASSET), amount, address(this));
    }

    /**
     * amount in ASSET
     */
    function _repayBorrow(uint256 amount) public onlyOwner {
        ASSET.safeIncreaseAllowance(address(LENDING_POOL), amount);
        LENDING_POOL.repay(address(ASSET), amount, USE_VARIABLE_DEBT, address(this));
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

    // -- swap --

    function swapVia1inch(SwapParams memory params) external {
        IERC20(params.tokenIn).transferFrom(msg.sender, address(this), params.amountIn);
        IERC20(params.tokenIn).approve(AGGREGATION_ROUTER_V5, params.amountIn);

        IAggregationRouterV5.SwapDescription memory desc = IAggregationRouterV5.SwapDescription({
            srcToken: IERC20(params.tokenIn),
            dstToken: IERC20(params.tokenOut),
            srcReceiver: payable(address(this)),
            dstReceiver: params.recipient,
            amount: params.amountIn,
            minReturnAmount: params.minReturnAmount,
            flags: params.flags
        });

        IAggregationRouterV5(AGGREGATION_ROUTER_V5).swap(
            address(0), // executor (0 for default)
            desc,
            params.permit,
            params.data
        );

        // If there are any leftover tokens, send them back to the user
        uint256 leftover = IERC20(params.tokenIn).balanceOf(address(this));
        if (leftover > 0) {
            IERC20(params.tokenIn).transfer(msg.sender, leftover);
        }
    }
}
