use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::market_fetch::fetch_all_markets;
use openbook_v2_client::vault_balances::get_vault_balances;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client using the helper function from client_init.rs
    let client = initialize_client();

    // Fetch all markets
    let markets = fetch_all_markets(&client).await?;

    // Fetch vault balances for each market
    let mut vault_balances = Vec::new();
    for market in markets {
        let balance = get_vault_balances(&client, market).await?;
        vault_balances.push(balance);
    }

    // Print the vault balances
    for (market, base_balance, quote_balance) in vault_balances {
        println!("Market Pubkey: {}", market.market_authority);
        println!("Base Vault Balance: {}", base_balance);
        println!("Quote Vault Balance: {}", quote_balance);
    }

    Ok(())
}
