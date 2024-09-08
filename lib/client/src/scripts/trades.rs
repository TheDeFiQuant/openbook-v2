use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use std::str::FromStr;
use openbook_v2_client::trade_fetcher::fetch_fill_events;
use openbook_v2_client::client_init::initialize_client;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Initialize the RPC client using your helper function
    let client = initialize_client();

    // Fetch the MARKET_PUBKEY from the environment
    let market_pubkey_str = std::env::var("MARKET_PUBKEY")
        .expect("Environment variable MARKET_PUBKEY must be set");
    let market_pubkey = match Pubkey::from_str(&market_pubkey_str) {
        Ok(pk) => pk,
        Err(_) => {
            error!("Invalid MARKET_PUBKEY format: {}", market_pubkey_str);
            return Err(anyhow::anyhow!("Invalid MARKET_PUBKEY format"));
        }
    };

    // Fetch fill events (matched trades) for the specified market
    info!("Fetching fill events for market: {}", market_pubkey);
    let fill_events = match fetch_fill_events(&client, market_pubkey).await {
        Ok(events) => {
            info!("Successfully fetched {} fill events", events.len());
            events
        }
        Err(e) => {
            error!("Failed to fetch fill events: {}", e);
            return Err(e);
        }
    };

    // Log the fill events in a readable format with extended information
    info!("{:<45} {:<15} {:<15} {:<45} {:<45} {:<20} {:<10} {:<10} {:<20} {:<20} {:<20} {:<20}", 
          "Market Pubkey", "Price", "Quantity", "Maker", "Taker", "Timestamp", "Taker Side", 
          "Maker Out", "Maker Slot", "Market Seq Num", "Peg Limit", "Maker Client Order ID");

    for event in fill_events {
        info!(
            "{:<45} {:<15} {:<15} {:<45} {:<45} {:<20} {:<10} {:<10} {:<20} {:<20} {:<20} {:<20}",
            event.market_pubkey, 
            event.price, 
            event.quantity, 
            event.maker, 
            event.taker, 
            event.timestamp, 
            event.taker_side,           // Display the taker_side
            event.maker_out,            // Display the maker_out (if the maker's order quantity is 0)
            event.maker_slot,           // Display the maker slot
            event.market_seq_num,       // Display the sequence number for the market
            event.peg_limit,            // Display the peg limit price (if any)
            event.maker_client_order_id // Display the client order ID for the maker
        );
    }

    Ok(())
}
