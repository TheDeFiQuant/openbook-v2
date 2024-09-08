use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config; // Correct import
use solana_client::rpc_config::RpcTransactionConfig; // Correct RpcTransactionConfig import
use solana_sdk::signature::Signature;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::option_serializer::OptionSerializer; // Import for OptionSerializer
use log::{info, warn, error, debug};
use std::str::FromStr;

pub async fn fetch_raw_transaction_logs(client: &RpcClient, market_pubkey: Pubkey) -> Result<()> {
    let mut before: Option<Signature> = None;
    let mut fetched_signatures = 0;

    loop {
        // Fetch signatures with pagination using the correct config
        let signatures = match client.get_signatures_for_address_with_config(
            &market_pubkey,
            GetConfirmedSignaturesForAddress2Config {
                before: before.clone(),
                limit: Some(1000), // Fetch 1000 signatures at a time
                ..Default::default()
            },
        ).await {
            Ok(sigs) => {
                info!("Fetched {} signatures", sigs.len());
                sigs
            },
            Err(e) => {
                error!("Failed to fetch signatures: {}", e);
                return Err(e.into());
            }
        };

        if signatures.is_empty() {
            break;
        }

        for (index, signature_info) in signatures.iter().enumerate() {
            debug!("Processing signature {}/{}", index + 1 + fetched_signatures, signatures.len() + fetched_signatures);

            let signature = match Signature::from_str(&signature_info.signature) {
                Ok(sig) => sig,
                Err(e) => {
                    error!("Invalid signature format: {}: {}", signature_info.signature, e);
                    continue;
                }
            };

            // Fetch the transaction details with maxSupportedTransactionVersion
            let transaction = match client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::JsonParsed),
                    max_supported_transaction_version: Some(0), // Add maxSupportedTransactionVersion
                    ..Default::default()
                },
            ).await {
                Ok(tx) => tx,
                Err(e) => {
                    error!("Failed to fetch transaction: {}", e);
                    continue;
                }
            };

            // Process transaction metadata
            if let Some(meta) = transaction.transaction.meta {
                // Handle OptionSerializer for log_messages
                match meta.log_messages {
                    OptionSerializer::Some(logs) => {
                        for log in logs {
                            info!("Log: {}", log);
                        }
                    }
                    OptionSerializer::None => {
                        warn!("No log messages found for transaction: {}", signature);
                    }
                    OptionSerializer::Skip => {
                        warn!("Log messages were skipped for transaction: {}", signature);
                    }
                }
            } else {
                warn!("No meta information found for transaction: {}", signature);
            }
        }

        // Update `before` to the last signature to fetch the next batch in pagination
        before = signatures.last().map(|sig_info| Signature::from_str(&sig_info.signature).unwrap());
        fetched_signatures += signatures.len();
    }

    info!("Finished processing all signatures.");
    Ok(())
}
