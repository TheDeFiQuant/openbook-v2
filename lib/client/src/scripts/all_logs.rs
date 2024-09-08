use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use std::str::FromStr;
use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::get_event_log::fetch_raw_transaction_logs;
use log::info;
use flexi_logger::{Logger, Duplicate, FileSpec};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger (logs to file and console)
    Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().directory("./")) // Specify file spec and directory
        .duplicate_to_stdout(Duplicate::Info)  // Logs to both file and stdout
        .start()?;

    // Initialize the RPC client using your helper function
    let client = initialize_client();

    // Fetch the MARKET_PUBKEY from the environment
    let market_pubkey_str = std::env::var("MARKET_PUBKEY")
        .expect("Environment variable MARKET_PUBKEY must be set");
    let market_pubkey = Pubkey::from_str(&market_pubkey_str)
        .expect("Invalid MARKET_PUBKEY format");

    // Log start of fetching transaction logs
    info!("Starting to fetch transaction logs for market: {}", market_pubkey);

    // Fetch transaction logs (raw logs) for the specified market
    if let Err(e) = fetch_raw_transaction_logs(&client, market_pubkey).await {
        // Log an error if fetching transaction logs fails
        log::error!("Failed to fetch transaction logs: {}", e);
    }

    // Log completion of task
    info!("Completed fetching transaction logs for market: {}", market_pubkey);

    Ok(())
}
