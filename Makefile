# Makefile for AAVE Looper project

# Variables
CARGO := cargo
FORGE := forge

# Directories
BINDINGS_DIR := bindings
CONTRACTS_DIR := contracts

# Environment variables (with default values)
MAINNET_RPC_URL ?= $(shell grep MAINNET_RPC_URL .env | cut -d '=' -f2)
ARBITRUM_RPC_URL ?= $(shell grep ARBITRUM_RPC_URL .env | cut -d '=' -f2)
PRIVATE_KEY ?= $(shell grep PRIVATE_KEY .env | cut -d '=' -f2)
DEFAULT_ANVIL_KEY := 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
NETWORK_ARGS := --rpc-url http://localhost:8545 --private-key $(DEFAULT_ANVIL_KEY) --broadcast

# Rust commands
rust-build:
	@echo "Building Rust bindings..."
	@cd $(BINDINGS_DIR) && $(CARGO) build

rust-test:
	@echo "Running Rust tests..."
	@cd $(BINDINGS_DIR) && $(CARGO) test

rust-check:
	@echo "Checking Rust code..."
	@cd $(BINDINGS_DIR) && $(CARGO) check

# Solidity commands
sol-build:
	@echo "Building Solidity contracts..."
	@cd $(CONTRACTS_DIR) && $(FORGE) build

sol-test:
	@echo "Running Solidity tests..."
	@cd $(CONTRACTS_DIR) && $(FORGE) test

# This command will generate an error (no cargo.toml) that can be ignored because we are generating the bindings in a subcrate of the src folder
sol-bind:
	@echo "Generating Rust bindings for Solidity contracts..."
	@cd $(CONTRACTS_DIR) && forge bind --alloy --bindings-path ../src/bindings --crate-name bindings --alloy-version 0.3.3 --module

deploy:
	@echo "Deploying AaveLooper..."
	@cd $(CONTRACTS_DIR) && forge script script/DeployAaveLooper.s.sol:DeployAaveLooper $(NETWORK_ARGS)

deploy_local:
	@echo "Deploying AaveLooper on Local..."
	NETWORK_ARGS := --rpc-url http://localhost:8545 --private-key $(DEFAULT_ANVIL_KEY) --broadcast
	@cd $(CONTRACTS_DIR) && forge script script/DeployAaveLooper.s.sol:DeployAaveLooper $(NETWORK_ARGS)

deploy-sepolia:
	@echo "Deploying AaveLooper on Sepolia..."
	NETWORK_ARGS := --rpc-url $(SEPOLIA_RPC_URL) --private-key $(PRIVATE_KEY) --broadcast --verify --etherscan-api-key $(ETHERSCAN_API_KEY) -vvvv
	@cd $(CONTRACTS_DIR) && forge script script/DeployAaveLooper.s.sol:DeployAaveLooper $(NETWORK_ARGS)

deploy-arbitrum:
	@echo "Deploying AaveLooper on Arbitrum..."
	NETWORK_ARGS := --rpc-url $(ARBITRUM_RPC_URL) --private-key $(PRIVATE_KEY) --broadcast --verify --arbiscan-api-key $(ARBISCAN_API_KEY) -vvvv
	@cd $(CONTRACTS_DIR) && forge script script/DeployAaveLooper.s.sol:DeployAaveLooper $(NETWORK_ARGS)

deploy-mainnet:
	j@echo "Deploying AaveLooper on Mainnet..."
	NETWORK_ARGS := --rpc-url $(MAINNET_RPC_URL) --private-key $(PRIVATE_KEY) --broadcast --verify --etherscan-api-key $(ETHERSCAN_API_KEY) -vvvv
	@cd $(CONTRACTS_DIR) && forge script script/DeployAaveLooper.s.sol:DeployAaveLooper $(NETWORK_ARGS)

enter-aave-position:
	@echo "Entering Aave position..."
	@cd $(CONTRACTS_DIR) && forge script script/EnterAavePosition.s.sol:EnterAavePosition $(NETWORK_ARGS)

# Combined commands
build: rust-build sol-build

test: rust-test sol-test

# Clean command
clean:
	@echo "Cleaning build artifacts..."
	@cd $(BINDINGS_DIR) && $(CARGO) clean
	@cd $(CONTRACTS_DIR) && $(FORGE) clean

# Default target
.PHONY: all
all: sol-bind build test

# Help command
help:
	@echo "Available commands:"
	@echo "  make rust-build    - Build Rust bindings"
	@echo "  make rust-test     - Run Rust tests"
	@echo "  make rust-check    - Check Rust code"
	@echo "  make sol-build     - Build Solidity contracts"
	@echo "  make sol-test      - Run Solidity tests"
	@echo "  make sol-bind      - Generate Rust bindings for Solidity contracts"
	@echo "  make build         - Build both Rust bindings and Solidity contracts"
	@echo "  make test          - Run both Rust and Solidity tests"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make all           - Generate bindings, build, and test everything"
	@echo "  make help          - Show this help message"

.PHONY: rust-build rust-test rust-check sol-build sol-test sol-bind build test clean all help