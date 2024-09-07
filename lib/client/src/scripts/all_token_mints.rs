use openbook_v2_client::get_vault_mint::fetch_all_token_mints;
use openbook_v2_client::market_fetch::fetch_all_markets;
use openbook_v2_client::client_init::initialize_client;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client using the helper function from client_init.rs
    let client = initialize_client();

    // Fetch all markets
    let markets = fetch_all_markets(&client).await?;

    // Fetch all token mints (base and quote token addresses) for the markets
    let token_mints = fetch_all_token_mints(&client).await?;

    // Print table header
    println!(
        "{:<45} {:<45} {:<45}",
        "Market Pubkey", "Base Token Mint", "Quote Token Mint"
    );
    println!("{}", "-".repeat(135));

    // Print market pubkeys and corresponding token mints
    for (market, (base_mint, quote_mint)) in markets.iter().zip(token_mints.iter()) {
        println!(
            "{:<45} {:<45} {:<45}",
            market.market_authority, base_mint, quote_mint
        );
    }

    Ok(())
}
