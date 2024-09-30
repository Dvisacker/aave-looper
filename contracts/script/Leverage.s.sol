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

    function getCodeAt(address _addr) public view returns (bytes memory) {
        uint256 size;
        bytes memory code;

        assembly {
            // retrieve the size of the code, this needs assembly
            size := extcodesize(_addr)
            // allocate output byte array - this could also be done without assembly
            // by using code = new bytes(size)
            code := mload(0x40)
            // new "memory end" including padding
            mstore(0x40, add(code, and(add(add(size, 0x20), 0x1f), not(0x1f))))
            // store length in memory
            mstore(code, size)
            // actually retrieve the code, this needs assembly
            extcodecopy(_addr, add(code, 0x20), 0, size)
        }

        return code;
    }

    function run() external {
        console2.log("Chainid:", block.chainid);
        console2.log("Block timestamp:", block.timestamp);
        console2.log("Block number:", block.number);
        console2.log("AaveLooper address:", aaveLooper);
        HelperConfig.NetworkConfig memory networkConfig = helperConfig.getActiveNetworkConfig();

        supplyAsset = vm.envOr("SUPPLY_ASSET", networkConfig.usdc);
        borrowAsset = vm.envOr("BORROW_ASSET", networkConfig.weth);
        initialAmount = vm.envOr("INITIAL_AMOUNT", uint256(1000000));
        iterations = vm.envOr("ITERATIONS", uint256(3));

        address uniswapV3Factory = networkConfig.uniswapV3Factory;

        AaveLooper looper = AaveLooper(aaveLooper);

        console2.log("Supply asset:", supplyAsset);
        console2.log("Borrow asset:", borrowAsset);
        console2.log("Iterations:", iterations);

        // (uint24 bestFeeTier, uint256 amountOut) =
        // looper.getBestFeeTier(uniswapV3Factory, supplyAsset, borrowAsset, initialAmount);
        // console2.log("Best fee tier:", bestFeeTier);
        // console2.log("Amount out:", amountOut);

        vm.startBroadcast();
        ERC20(supplyAsset).approve(aaveLooper, initialAmount);
        uint256 liquidity = looper.enterPosition(supplyAsset, borrowAsset, initialAmount, iterations, 500);
        vm.stopBroadcast();

        console2.log("Entered position. Final liquidity:", liquidity);
    }
}
