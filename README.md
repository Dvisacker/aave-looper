# Aave Interaction CLI

This CLI tool allows you to interact with the Aave protocol on Ethereum and Arbitrum networks. You can supply assets, borrow assets, enter leveraged positions, and run an automated bot.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- RPC URL

## Building the Project

1. Clone the repository:
   ```
   git clone https://github.com/your-username/aave-interaction-cli.git
   cd aave-interaction-cli
   ```

2. Build the project:
   ```
   cargo build --release
   ```

   The built binary will be located at `target/release/aave-interaction-cli`.

## Configuration

1. Create a `.env` file in the project root with the following content:
   ```
   PRIVATE_KEY=your_wallet_private_key
   RPC_URL=your_ethereum_node_url
   TELEGRAM_BOT_TOKEN=your_telegram_bot_token (optional)
   CHAT_ID=your_telegram_chat_id (optional)
   ```

2. Ensure that the `src/contract_addresses.json` file contains the correct contract addresses for the networks and tokens you want to interact with.

## Using the CLI

The general syntax for using the CLI is:
