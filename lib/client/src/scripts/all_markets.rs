use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::market_fetch::fetch_all_markets;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client using the helper function from client_init.rs
    let client = initialize_client();

    // Fetch all markets
    let markets = fetch_all_markets(&client).await?;

    // Print table header
    println!("{:<45}", "Market Pubkey");
    println!("{}", "-".repeat(45));

    // Print all the market pubkeys
    for market in markets {
        println!("{:<45}", market.market_authority);
    }

    Ok(())
}
