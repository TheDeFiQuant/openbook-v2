use openbook_v2_client::client_init::initialize_client;
use openbook_v2_client::market_fetch::fetch_all_markets;
use anyhow::Result;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Initialize the client using the helper function from client_init.rs
    let client = initialize_client();

    // Fetch all markets
    let markets = fetch_all_markets(&client).await?;

    // Log table header
    info!(
        "{:<45} {:<5} {:<5} {:<45} {:<10} {:<45} {:<45} {:<45} {:<10} {:<5} {:<5} {:<10} {:<10} {:<15} {:<15} {:<15} {:<10} {:<10} {:<10} {:<20} {:<20} {:<45} {:<10} {:<45} {:<10} {:<10}",
        "Market Pubkey", "Bump", "Base Dec.", "Market Authority", "Expiry", "Fee Admin",
        "Bids", "Asks", "Conf Filter", "Staleness", "Quote Lot", "Base Lot", "Seq Num",
        "Reg Time", "Maker Fee", "Taker Fee", "Accrued Fees", "Ref Fees", "Rebates Accr.",
        "Fees Avail.", "Maker Vol.", "Taker Vol.", "Base Mint", "Base Dep.", "Quote Mint", "Quote Dep."
    );

    // Log separator
    info!("{}", "-".repeat(650));

    // Log all the relevant fields for each market
    for market in markets {
        info!(
            "{:<45} {:<5} {:<5} {:<45} {:<10} {:<45} {:<45} {:<45} {:.6} {:<5} {:<5} {:<10} {:<10} {:<15} {:<15} {:<15} {:<10} {:<10} {:<10} {:<20} {:<20} {:<45} {:<10} {:<45} {:<10} {:<10}",
            market.market_pubkey,
            market.market_data.bump,
            market.market_data.base_decimals,
            market.market_data.market_authority,
            market.market_data.time_expiry,
            market.market_data.collect_fee_admin,
            market.market_data.bids,
            market.market_data.asks,
            market.market_data.oracle_config.conf_filter,
            market.market_data.oracle_config.max_staleness_slots,
            market.market_data.quote_lot_size,
            market.market_data.base_lot_size,
            market.market_data.seq_num,
            market.market_data.registration_time,
            market.market_data.maker_fee,
            market.market_data.taker_fee,
            market.market_data.fees_accrued,
            market.market_data.fees_to_referrers,
            market.market_data.referrer_rebates_accrued,
            market.market_data.fees_available,
            market.market_data.maker_volume,
            market.market_data.taker_volume_wo_oo,
            market.market_data.base_mint,
            market.market_data.base_deposit_total,
            market.market_data.quote_mint,
            market.market_data.quote_deposit_total
        );
    }

    Ok(())
}
