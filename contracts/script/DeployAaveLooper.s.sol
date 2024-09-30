// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {AaveLooper} from "../src/AaveLooper.sol";
import {HelperConfig} from "./HelperConfig.s.sol";
import {console2} from "forge-std/console2.sol";

contract DeployAaveLooper is Script {
    address owner;
    address lendingPool;
    address incentives;
    address uniswapV3Router;

    function run() external {
        HelperConfig helperConfig = new HelperConfig();
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        owner = networkConfig.deployerAddress;
        lendingPool = networkConfig.aaveLendingPool;
        incentives = networkConfig.aaveIncentives;
        uniswapV3Router = networkConfig.uniswapV3Router;
        require(owner != address(0), "Owner address not set");
        require(lendingPool != address(0), "Lending Pool address not set");
        require(incentives != address(0), "Incentives Controller address not set");

        // Start broadcasting transactions
        vm.startBroadcast();

        // Deploy the AaveLooper contract
        AaveLooper looper = new AaveLooper(owner, lendingPool, incentives, uniswapV3Router);

        console2.log("AaveLooper deployed at:", address(looper));

        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
