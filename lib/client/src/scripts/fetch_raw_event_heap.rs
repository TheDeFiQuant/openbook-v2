use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::event_heap_reader::fetch_event_heap_raw;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the RPC client using the helper function
    println!("Initializing client...");
    let client = initialize_client();
    println!("Client initialized");

    // Fetch the MARKET_PUBKEY from the environment
    let market_pubkey_str = std::env::var("MARKET_PUBKEY")
        .expect("Environment variable MARKET_PUBKEY must be set");
    println!("Market Pubkey String: {}", market_pubkey_str);

    // Validate the Pubkey format
    let market_pubkey = Pubkey::from_str(&market_pubkey_str)
        .expect("Invalid MARKET_PUBKEY format");
    println!("Market Pubkey: {:?}", market_pubkey);

    // Fetch and print the raw event heap data for the given market
    println!("Fetching raw event heap data...");
    fetch_event_heap_raw(&client, market_pubkey).await?;
    println!("Raw event heap data fetched");

    Ok(())
}
