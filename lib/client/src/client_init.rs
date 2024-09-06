use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

/// Initializes the RPC client with the URL from an environment variable
pub fn initialize_client() -> RpcClient {
    // Get the RPC URL from the environment variable or panic if it's not set
    let rpc_url = env::var("RPC_URL")
        .expect("Environment variable RPC_URL must be set");

    let commitment = CommitmentConfig::processed();
    RpcClient::new_with_commitment(rpc_url, commitment)
}
