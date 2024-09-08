use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::market_fetch::fetch_all_markets;
use openbook_v2_client::vault_balances::get_vault_balances;
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

    // Fetch vault balances for each market
    let mut vault_balances = Vec::new();
    for market in markets {
        let balance = get_vault_balances(&client, market.market_data).await?;
        vault_balances.push((market.market_pubkey, balance));
    }

    // Log table header
    info!("{:<45} {:<20} {:<20}", "Market Pubkey", "Base Vault Balance", "Quote Vault Balance");
    info!("{}", "-".repeat(85));

    // Log the vault balances in a table format
    for (market_pubkey, (market, base_balance, quote_balance)) in vault_balances {
        info!(
            "{:<45} {:<20} {:<20}",
            market_pubkey, base_balance, quote_balance
        );
    }

    Ok(())
}
