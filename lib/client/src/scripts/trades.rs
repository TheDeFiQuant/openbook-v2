use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use std::str::FromStr;
use openbook_v2_client::trade_fetcher::fetch_fill_events;
use openbook_v2_client::client_init::initialize_client;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the RPC client using your helper function
    let client = initialize_client();

    // Fetch the MARKET_PUBKEY from the environment
    let market_pubkey_str = std::env::var("MARKET_PUBKEY")
        .expect("Environment variable MARKET_PUBKEY must be set");
    let market_pubkey = Pubkey::from_str(&market_pubkey_str)
        .expect("Invalid MARKET_PUBKEY format");

    // Fetch fill events (matched trades) for the specified market
    let fill_events = fetch_fill_events(&client, market_pubkey).await?;

    // Print the fill events in a readable format with extended information
    println!("{:<45} {:<15} {:<15} {:<45} {:<45} {:<20} {:<10} {:<10} {:<20} {:<20} {:<20} {:<20}", 
             "Market Pubkey", "Price", "Quantity", "Maker", "Taker", "Timestamp", "Taker Side", 
             "Maker Out", "Maker Slot", "Market Seq Num", "Peg Limit", "Maker Client Order ID");
    println!("{}", "-".repeat(300));
    
    for event in fill_events {
        println!(
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
