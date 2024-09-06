use openbook_v2_client::client_init::initialize_client;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use openbook_v2::state::Market;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::str::FromStr;
use anchor_lang::AccountDeserialize;

/// Fetch all OpenBook markets and return them as a vector of `Market` structs
async fn fetch_all_markets(client: &RpcClient) -> Result<Vec<(Market, Pubkey)>> {
    let program_id = Pubkey::from_str("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb").expect("Invalid Pubkey");

    // Fetch all accounts associated with the OpenBook program
    let accounts = client.get_program_accounts(&program_id).await?;
    
    let markets: Vec<(Market, Pubkey)> = accounts.into_iter()
        .filter_map(|(pubkey, account_data)| {
            Market::try_deserialize(&mut &account_data.data[..])
                .ok()
                .map(|market| (market, pubkey))
        })
        .collect();

    Ok(markets)
}

/// Fetch the base and quote vault addresses for a given `Market`
async fn get_vault_addresses(market: &Market) -> Result<(Pubkey, Pubkey)> {
    let base_vault_address = market.market_base_vault;
    let quote_vault_address = market.market_quote_vault;
    Ok((base_vault_address, quote_vault_address))
}

/// Main function that initializes the client, fetches all markets, and retrieves vault addresses
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client using the helper function from client_init.rs
    let client = initialize_client();

    // Fetch all markets
    let markets = fetch_all_markets(&client).await?;

    // Print table header
    println!("{:<45} {:<45} {:<45}", "Market Pubkey", "Base Vault Address", "Quote Vault Address");
    println!("{}", "-".repeat(135));

    // Fetch vault addresses for each market and print them in a table format
    for (market, market_pubkey) in markets {
        let (base_vault, quote_vault) = get_vault_addresses(&market).await?;
        println!(
            "{:<45} {:<45} {:<45}",
            market_pubkey, base_vault, quote_vault
        );
    }

    Ok(())
}
