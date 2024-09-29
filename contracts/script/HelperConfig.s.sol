// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {AaveLooper} from "../src/AaveLooper.sol";
import {console2} from "forge-std/console2.sol";

contract HelperConfig is Script {
    NetworkConfig public activeNetworkConfig;

    struct NetworkConfig {
        uint256 deployerKey;
        address deployerAddress;
        address aaveLendingPool;
        address aaveIncentives;
        address usdc;
        address usdt;
        address dai;
        address weth;
    }

    constructor() {
        if (block.chainid == 42161) {
            uint256 deployerKey = vm.envUint("PRIVATE_KEY");
            activeNetworkConfig = NetworkConfig({
                deployerKey: deployerKey,
                deployerAddress: vm.addr(deployerKey),
                aaveLendingPool: 0x794a61358D6845594F94dc1DB02A252b5b4814aD,
                aaveIncentives: 0x929EC64c34a17401F460460D4B9390518E5B473e,
                usdc: 0xaf88d065e77c8cC2239327C5EDb3A432268e5831,
                usdt: 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9,
                dai: 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1,
                weth: 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1
            });
        } else if (block.chainid == 1) {
            uint256 deployerKey = vm.envUint("PRIVATE_KEY");
            activeNetworkConfig = NetworkConfig({
                deployerKey: deployerKey,
                deployerAddress: vm.addr(deployerKey),
                aaveLendingPool: 0x7d2768dE32b0b80b7a3454c06BdAc94A69DDc7A9,
                aaveIncentives: 0x8164Cc65827dcFe994AB23944CBC90e0aa80bFcb,
                usdc: 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48,
                usdt: 0xdAC17F958D2ee523a2206206994597C13D831ec7,
                dai: 0x6B175474E89094C44Da98b954EedeAC495271d0F,
                weth: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
            });
            // sepolia
        } else if (block.chainid == 11155111) {
            uint256 deployerKey = vm.envUint("PRIVATE_KEY");
            activeNetworkConfig = NetworkConfig({
                deployerKey: deployerKey,
                deployerAddress: vm.addr(deployerKey),
                aaveLendingPool: 0x6Ae43d3271ff6888e7Fc43Fd7321a503ff738951,
                aaveIncentives: 0x4DA5c4da71C5a167171cC839487536d86e083483,
                usdc: 0x94a9D9AC8a22534E3FaCa9F4e7F2E2cf85d5E4C8,
                usdt: 0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0,
                dai: 0xFF34B3d4Aee8ddCd6F9AFFFB6Fe49bD371b8a357,
                weth: 0xC558DBdd856501FCd9aaF1E62eae57A9F0629a3c
            });
        } else {
            revert("Unsupported network");
        }
    }

    function getActiveNetworkConfig() public view returns (NetworkConfig memory) {
        return activeNetworkConfig;
    }
}
