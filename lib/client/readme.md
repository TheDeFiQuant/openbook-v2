# Openbook v2 Client

This project includes multiple Rust binary targets, each defined in the `Cargo.toml` under the `[bin]` section. These binaries are designed to execute specific tasks within the Openbook v2 client, focusing on interacting with Openbook markets, vaults, and token mints. Below is a brief description of each binary:

### Available Binaries:

- **all_deposits**:  
  Path: `src/scripts/all_market_deposits.rs`  
  Fetches and displays vault balances (base and quote) for all markets.

- **all_vaults**:  
  Path: `src/scripts/all_vault_addresses.rs`  
  Fetches and lists the addresses of all vaults in the Openbook program.

- **all_markets_data**:  
  Path: `src/scripts/all_markets_data.rs`  
  Retrieves detailed data for all markets available in Openbook.

- **all_markets**:  
  Path: `src/scripts/all_markets.rs`  
  Fetches and lists all active markets on the Openbook decentralized exchange.

- **trades**:  
  Path: `src/scripts/trades.rs`  
  Displays trade data for specific markets on Openbook.

- **token_mint**:  
  Path: `src/scripts/all_token_mints.rs`  
  Fetches and logs base and quote token mints for all markets.

- **event_heap**:  
  Path: `src/scripts/fetch_raw_event_heap.rs`  
  Retrieves raw event heap data for a given market on Openbook.

### Running a Binary

To run any of the binaries, use the following command in your terminal:

```bash
cargo run --bin <binary_name>
```

## Setting Environment Variables

Some binaries require environment variables to be set, such as `MARKET_PUBKEY` for specifying a market and `RPC_URL` to configure the Solana RPC URL.

- `MARKET_PUBKEY`: The public key of the market you want to interact with.
- `RPC_URL`: The URL of the Solana RPC node you want to connect to.

To set these environment variables, you can use the following commands before running a binary:

```bash
export MARKET_PUBKEY=<your_market_pubkey>
export RPC_URL=<your_rpc_url>
```

