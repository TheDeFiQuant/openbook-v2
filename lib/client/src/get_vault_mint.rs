use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
use openbook_v2::state::Market;
use crate::market_fetch::fetch_all_markets; // Correct import

/// Fetch the base and quote token mints for a given `Market`
pub async fn get_vault_mints(market: &Market) -> Result<(Pubkey, Pubkey)> {
    let base_mint = market.base_mint;
    let quote_mint = market.quote_mint;
    Ok((base_mint, quote_mint))
}

/// Fetch the token mints for all OpenBook markets
pub async fn fetch_all_token_mints(client: &RpcClient) -> Result<Vec<(Pubkey, Pubkey)>> {
    let markets = fetch_all_markets(client).await?;

    let token_mints: Vec<(Pubkey, Pubkey)> = markets
        .into_iter()
        .map(|market| (market.base_mint, market.quote_mint))
        .collect();

    Ok(token_mints)
}
