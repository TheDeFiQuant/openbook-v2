use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::rpc_client::RpcClient;
use anchor_lang::AccountDeserialize;
use openbook_v2::state::Market;
use std::str::FromStr;

/// Fetch all OpenBook markets and return them as a vector of `Market` structs
pub async fn fetch_all_markets(client: &RpcClient) -> Result<Vec<Market>> {
    let program_id = Pubkey::from_str("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb").expect("Invalid Pubkey");

    // Fetch all accounts associated with the OpenBook program
    let accounts = client.get_program_accounts(&program_id).await?;
    
    let markets: Vec<Market> = accounts.into_iter()
        .filter_map(|(_, account_data)| {
            Market::try_deserialize(&mut &account_data.data[..]).ok()
        })
        .collect();

    Ok(markets)
}
