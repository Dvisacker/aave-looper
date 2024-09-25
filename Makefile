# Makefile for AAVE Looper project

# Variables
CARGO := cargo
FORGE := forge

# Directories
BINDINGS_DIR := bindings
CONTRACTS_DIR := contracts

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

sol-bind:
	@echo "Generating Rust bindings for Solidity contracts..."
	@cd $(CONTRACTS_DIR) && $(FORGE) bind --alloy --bindings-path ../$(BINDINGS_DIR) --crate-name bindings

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