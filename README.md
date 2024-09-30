# Aave Interaction CLI

This CLI tool allows you to interact with the Aave protocol on Ethereum and Arbitrum networks. You can supply assets, borrow assets, enter leveraged positions, and run an automated bot.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Foundry (for Solidity development)
- RPC URL

## Building the Project

1. Clone the repository:
   ```
   git clone https://github.com/your-username/aave-interaction-cli.git
   cd aave-interaction-cli
   ```

2. Build the project:
   ```
   make build
   ```

   This will build both the Rust bindings and Solidity contracts.

## Configuration

1. Create a `.env` file in the project root with the following content:
   ```
   PRIVATE_KEY=your_wallet_private_key
   MAINNET_RPC_URL=your_ethereum_mainnet_node_url
   ARBITRUM_RPC_URL=your_arbitrum_node_url
   SEPOLIA_RPC_URL=your_sepolia_testnet_node_url
   ETHERSCAN_API_KEY=your_etherscan_api_key
   TELEGRAM_BOT_TOKEN=your_telegram_bot_token (optional)
   CHAT_ID=your_telegram_chat_id (optional)
   ```

2. Ensure that the `src/contract_addresses.json` file contains the correct contract addresses for the networks and tokens you want to interact with.

## Using the CLI

The general syntax for using the CLI is:

```
./target/release/aave-interaction-cli [COMMAND] [OPTIONS]
```

## Available Commands

Here are some of the available commands from the Makefile:

### Deployment

Use the make commands to deploy the looper contractto the network of your choice.

- Deploy to local network:
  ```
  make deploy-local
  ```

- Deploy to Sepolia testnet:
  ```
  make deploy-sepolia
  ```

- Deploy to Arbitrum:
  ```
  make deploy-arbitrum
  ```

- Deploy to Ethereum mainnet:
  ```
  make deploy-mainnet
  ```

### Leveraging

  ```
  make leverage-arbitrum
  ```

### Exiting Positions

  ```
  make flashloan-exit-arbitrum
  ```

### Getting Position Information

  ```
  make get-position-arbitrum
  ```

## Development Commands

- Run tests:
  ```
  make test
  ```

- Clean build artifacts:
  ```
  make clean
  ```

- Generate Rust bindings for Solidity contracts:
  ```
  make forge-bind
  ```

For a full list of available commands, run:
```
make help
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.