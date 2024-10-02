// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {AaveLooper} from "../src/AaveLooper.sol";
import {ERC20} from "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import {console2} from "forge-std/console2.sol";
import {console} from "forge-std/console.sol";
import {DevOpsTools} from "../lib/foundry-devops/src/DevOpsTools.sol";
import {HelperConfig} from "./HelperConfig.s.sol";
import {Strings} from "../lib/openzeppelin-contracts/contracts/utils/Strings.sol";

contract GetPosition is Script {
    HelperConfig helperConfig = new HelperConfig();
    address aaveLooper = DevOpsTools.get_most_recent_deployment("AaveLooper", block.chainid);
    address supplyAsset;
    address borrowAsset;
    uint256 initialAmount;
    uint256 iterations;

    function formatFloat(uint256 value, uint256 decimals) internal pure returns (string memory) {
        uint256 factor = 10 ** decimals;
        uint256 integerPart = value / factor;
        uint256 fractionalPart = value % factor;

        string memory integerStr = Strings.toString(integerPart);
        string memory fractionalStr = Strings.toString(fractionalPart);

        // Pad the fractional part with leading zeros
        while (bytes(fractionalStr).length < decimals) {
            fractionalStr = string(abi.encodePacked("0", fractionalStr));
        }

        return string(abi.encodePacked(integerStr, ".", fractionalStr));
    }

    function run() external {
        AaveLooper looper = AaveLooper(aaveLooper);
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        (
            uint256 totalCollateral,
            uint256 totalDebt,
            uint256 availableBorrows,
            uint256 currentLiquidationThreshold,
            uint256 ltv,
            uint256 healthFactor
        ) = looper.getPositionData();

        supplyAsset = vm.envOr("SUPPLY_ASSET", networkConfig.usdc);
        borrowAsset = vm.envOr("BORROW_ASSET", networkConfig.weth);

        uint256 borrowAssetBalance = ERC20(borrowAsset).balanceOf(address(looper));
        uint256 supplyAssetBalance = ERC20(supplyAsset).balanceOf(address(looper));
        string memory borrowAssetBalanceFormatted = formatFloat(borrowAssetBalance, 18);
        string memory supplyAssetBalanceFormatted = formatFloat(supplyAssetBalance, 6);
        uint256 userBorrowAssetBalance = ERC20(borrowAsset).balanceOf(address(msg.sender));
        uint256 userSupplyAssetBalance = ERC20(supplyAsset).balanceOf(address(msg.sender));
        string memory userBorrowAssetBalanceFormatted = formatFloat(userBorrowAssetBalance, 18);
        string memory userSupplyAssetBalanceFormatted = formatFloat(userSupplyAssetBalance, 6);
        string memory totalCollateralUSD = formatFloat(totalCollateral, 8);
        string memory totalDebtUSD = formatFloat(totalDebt, 8);
        string memory availableBorrowsUSD = formatFloat(availableBorrows, 8);

        console2.log("Contract address:", aaveLooper);
        console2.log("Borrow asset contract balance (WETH):", borrowAssetBalanceFormatted);
        console2.log("Supply asset contract balance (USDC):", supplyAssetBalanceFormatted);
        console2.log("Borrow asset user balance (WETH):", userBorrowAssetBalanceFormatted);
        console2.log("Supply asset user balance (USDC):", userSupplyAssetBalanceFormatted);
        console2.log("Total collateral:", totalCollateralUSD);
        console2.log("Total debt:", totalDebtUSD);
        console2.log("Available borrows:", availableBorrowsUSD);
        console2.log("Current liquidation threshold:", currentLiquidationThreshold);
        console2.log("LTV:", ltv);
        console2.log("Health factor:", healthFactor);
    }
}
