use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
use openbook_v2::state::Market;

/// Fetch the base and quote vault addresses for a given `Market`
pub async fn get_vault_addresses(market: &Market) -> Result<(Pubkey, Pubkey)> {
    let base_vault_address = market.market_base_vault;
    let quote_vault_address = market.market_quote_vault;
    Ok((base_vault_address, quote_vault_address))
}

/// Fetch the vault addresses for all OpenBook markets
pub async fn fetch_all_vault_addresses(client: &RpcClient) -> Result<Vec<(Pubkey, Pubkey)>> {
    let markets = super::fetch_all_markets(client).await?;
    
    let vault_addresses: Vec<(Pubkey, Pubkey)> = markets
        .into_iter()
        .map(|market| (market.market_base_vault, market.market_quote_vault))
        .collect();

    Ok(vault_addresses)
}
