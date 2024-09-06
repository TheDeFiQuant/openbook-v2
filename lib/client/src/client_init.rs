use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

/// Initializes the RPC client with the given URL and commitment level
pub fn initialize_client() -> RpcClient {
    let rpc_url = "YOUR_RPC_URL_HERE".to_string(); // add your rpc url here
    let commitment = CommitmentConfig::processed();
    RpcClient::new_with_commitment(rpc_url, commitment)
}
