use std::sync::Arc;
use std::str::FromStr;
use anchor_client::Cluster;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use anchor_lang::AccountDeserialize;
use openbook_v2::state::Market;

#[derive(Clone, Debug)]
pub struct Client {
    pub cluster: Cluster,
    pub fee_payer: Arc<Keypair>,
    pub commitment: CommitmentConfig,
    pub timeout: Option<std::time::Duration>,
}

impl Client {
    pub fn new(
        cluster: Cluster,
        commitment: CommitmentConfig,
        fee_payer: Arc<Keypair>,
        timeout: Option<std::time::Duration>,
    ) -> Self {
        Self {
            cluster,
            fee_payer,
            commitment,
            timeout,
        }
    }

    pub fn rpc_async(&self) -> RpcClient {
        let url = self.cluster.url().to_string();
        RpcClient::new_with_commitment(url, self.commitment)
    }
}

async fn get_vault_balances(client: &RpcClient, market: Market) -> Result<(Market, u64, u64)> {
    let base_vault_balance = client.get_token_account_balance(&market.market_base_vault).await?;
    let quote_vault_balance = client.get_token_account_balance(&market.market_quote_vault).await?;
    Ok((market, base_vault_balance.amount.parse()?, quote_vault_balance.amount.parse()?))
}

async fn fetch_all_markets(client: &RpcClient) -> Result<Vec<(Market, u64, u64)>> {
    let program_id = Pubkey::from_str("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb").expect("Invalid Pubkey");

    // Fetch all accounts associated with the OpenBook program
    let accounts = client.get_program_accounts(&program_id).await?;

    let market_futures = accounts.into_iter().filter_map(|(_pubkey, account_data)| {
        if let Ok(market) = Market::try_deserialize(&mut &account_data.data[..]) {
            // Clone client to avoid referencing temporary value
            let rpc_client = client.clone();
            Some(get_vault_balances(rpc_client, market))
        } else {
            None
        }
    });

    let results: Vec<Result<(Market, u64, u64)>> = futures::future::join_all(market_futures).await;

    results.into_iter().collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the RpcClient with a hardcoded RPC URL
    let rpc_url = "YOUR_RPC_URL".to_string(); // Replace with your desired RPC URL
    let commitment = CommitmentConfig::processed();
    let client = RpcClient::new_with_commitment(rpc_url, commitment);

    // Fetch all markets and their vault balances
    let markets = fetch_all_markets(&client).await?;
    for (market, base_balance, quote_balance) in markets {
        println!("Market Pubkey: {}", market.market_authority);
        println!("Base Vault Balance: {}", base_balance);
        println!("Quote Vault Balance: {}", quote_balance);
    }

    Ok(())
}