// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {AaveLooper} from "../src/AaveLooper.sol";
import {ERC20} from "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import {console2} from "forge-std/console2.sol";
import {DevOpsTools} from "../lib/foundry-devops/src/DevOpsTools.sol";
import {HelperConfig} from "./HelperConfig.s.sol";

contract Leverage is Script {
    HelperConfig helperConfig = new HelperConfig();
    address aaveLooper = DevOpsTools.get_most_recent_deployment("AaveLooper", block.chainid);
    address supplyAsset;
    address borrowAsset;
    uint256 initialAmount;
    uint256 iterations;

    function run() external {
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        supplyAsset = vm.envOr("SUPPLY_ASSET", networkConfig.usdc);
        borrowAsset = vm.envOr("BORROW_ASSET", networkConfig.weth);
        initialAmount = vm.envOr("INITIAL_AMOUNT", uint256(1000000));
        iterations = vm.envOr("ITERATIONS", uint256(3));

        AaveLooper looper = AaveLooper(aaveLooper);

        vm.startBroadcast();
        ERC20(supplyAsset).approve(aaveLooper, initialAmount);
        uint256 liquidity = looper.leverage(supplyAsset, borrowAsset, initialAmount, iterations, 500);
        vm.stopBroadcast();

        console2.log("Entered position. Final liquidity:", liquidity);
    }
}
