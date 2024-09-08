use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
use anchor_lang::AccountDeserialize;
use openbook_v2::state::Market;
use std::str::FromStr;

/// Struct to hold both the market Pubkey and the Market struct
#[derive(Debug)] // Derive the Debug trait to allow raw data logging
pub struct MarketInfo {
    pub market_pubkey: Pubkey,
    pub market_data: Market,
}

/// Fetch all OpenBook markets and return them as a vector of `MarketInfo` structs
pub async fn fetch_all_markets(client: &RpcClient) -> Result<Vec<MarketInfo>> {
    let program_id = Pubkey::from_str("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb").expect("Invalid Pubkey");

    // Fetch all accounts associated with the OpenBook program
    let accounts = client.get_program_accounts(&program_id).await?;

    // Deserialize the account data and store both the Pubkey and the Market data
    let markets: Vec<MarketInfo> = accounts.into_iter()
        .filter_map(|(pubkey, account_data)| {
            Market::try_deserialize(&mut &account_data.data[..])
                .ok()
                .map(|market_data| MarketInfo {
                    market_pubkey: pubkey,
                    market_data,
                })
        })
        .collect();

    Ok(markets)
}
