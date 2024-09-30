// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {AaveLooper} from "../src/AaveLooper.sol";
import {ERC20} from "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import {console2} from "forge-std/console2.sol";
import {DevOpsTools} from "../lib/foundry-devops/src/DevOpsTools.sol";
import {HelperConfig} from "./HelperConfig.s.sol";

contract GetPosition is Script {
    HelperConfig helperConfig = new HelperConfig();
    address aaveLooper = DevOpsTools.get_most_recent_deployment("AaveLooper", block.chainid);
    address supplyAsset;
    address borrowAsset;
    uint256 initialAmount;
    uint256 iterations;

    function run() external {
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();
        AaveLooper looper = AaveLooper(aaveLooper);

        (
            uint256 totalCollateral,
            uint256 totalDebt,
            uint256 availableBorrows,
            uint256 currentLiquidationThreshold,
            uint256 ltv,
            uint256 healthFactor
        ) = looper.getPositionData();

        console2.log("Total collateral:", totalCollateral);
        console2.log("Total debt:", totalDebt);
        console2.log("Available borrows:", availableBorrows);
        console2.log("Current liquidation threshold:", currentLiquidationThreshold);
        console2.log("LTV:", ltv);
        console2.log("Health factor:", healthFactor);
    }
}
