use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::market_fetch::fetch_all_markets;
use anyhow::Result;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Initialize the client using the helper function from client_init.rs
    let client = initialize_client();

    // Fetch all markets
    let markets = fetch_all_markets(&client).await?;

    // Log table header for market pubkey
    info!("{:<45}", "Market Pubkey");

    // Log separator
    info!("{}", "-".repeat(45));

    // Log only the market pubkey for each market
    for market in markets {
        info!("{:<45}", market.market_pubkey);
    }

    Ok(())
}
