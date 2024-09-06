use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use openbook_v2::state::Market;

/// Fetch vault balances for a given market
pub async fn get_vault_balances(client: &RpcClient, market: Market) -> Result<(Market, u64, u64)> {
    let base_vault_balance = client.get_token_account_balance(&market.market_base_vault).await?;
    let quote_vault_balance = client.get_token_account_balance(&market.market_quote_vault).await?;
    
    Ok((market, base_vault_balance.amount.parse()?, quote_vault_balance.amount.parse()?))
}
