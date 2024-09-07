use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use openbook_v2::state::{EventHeap, Market, EventHeapHeader, AnyEvent, FillEvent, OutEvent, EventType}; // Add missing imports
use anchor_lang::AccountDeserialize;

/// Fetch raw event heap data for a specific market
pub async fn fetch_event_heap_raw(client: &RpcClient, market_pubkey: Pubkey) -> Result<()> {
    // Fetch the market account
    println!("Fetching market account...");
    let market_account_data = client.get_account_data(&market_pubkey).await?;
    let market: Market = Market::try_deserialize(&mut &market_account_data[..])?;
    println!("Market account fetched");

    // Fetch the event heap account
    let event_heap_pubkey = market.event_heap;
    println!("Event heap pubkey: {:?}", event_heap_pubkey);
    let event_heap_account_data = client.get_account_data(&event_heap_pubkey).await?;
    let event_heap: EventHeap = EventHeap::try_deserialize(&mut &event_heap_account_data[..])?;
    println!("Event heap account fetched");

    // Print raw event heap data
    print_raw_event_heap(&event_heap);

    Ok(())
}

/// Print the raw event heap data
pub fn print_raw_event_heap(event_heap: &EventHeap) {
    println!("EventHeap Raw Data:");
    println!("Header: seq_num = {}, count = {}", event_heap.header.seq_num, event_heap.header.count());

    println!("Reserved: {:?}", event_heap.reserved);

    // Iterate over the raw nodes in the event heap
    for (i, node) in event_heap.iter().enumerate() {
        match node.0.event_type {
            0 => { // FillEvent
                if let Some(fill_event) = parse_fill_event(&node.0) {
                    println!("Node {}: Fill Event", i);
                    println!(
                        "  taker_side: {}, maker_out: {}, maker_slot: {}, timestamp: {}, market_seq_num: {},",
                        fill_event.taker_side, fill_event.maker_out, fill_event.maker_slot, fill_event.timestamp, fill_event.market_seq_num
                    );
                    println!(
                        "  maker: {:?}, maker_timestamp: {}, maker_client_order_id: {}, taker: {:?}, taker_client_order_id: {},",
                        fill_event.maker, fill_event.maker_timestamp, fill_event.maker_client_order_id, fill_event.taker, fill_event.taker_client_order_id
                    );
                    println!(
                        "  price: {}, peg_limit: {}, quantity: {}, reserved: {:?}",
                        fill_event.price, fill_event.peg_limit, fill_event.quantity, fill_event.reserved
                    );
                } else {
                    println!("Node {}: Invalid Fill Event Data", i);
                }
            }
            1 => { // OutEvent
                if let Some(out_event) = parse_out_event(&node.0) {
                    println!("Node {}: Out Event", i);
                    println!(
                        "  side: {}, owner_slot: {}, timestamp: {}, seq_num: {}, owner: {:?}, quantity: {},",
                        out_event.side, out_event.owner_slot, out_event.timestamp, out_event.seq_num, out_event.owner, out_event.quantity
                    );
                } else {
                    println!("Node {}: Invalid Out Event Data", i);
                }
            }
            _ => {
                println!("Node {}: Unknown Event Type: {}", i, node.0.event_type);
            }
        }
    }
}

/// Deserialize the raw `AnyEvent` data into a `FillEvent`
fn parse_fill_event(event: &AnyEvent) -> Option<FillEvent> {
    if event.event_type == EventType::Fill as u8 {
        let fill_event = unsafe { *(event as *const AnyEvent as *const FillEvent) };
        Some(fill_event)
    } else {
        None
    }
}

/// Deserialize the raw `AnyEvent` data into an `OutEvent`
fn parse_out_event(event: &AnyEvent) -> Option<OutEvent> {
    if event.event_type == EventType::Out as u8 {
        let out_event = unsafe { *(event as *const AnyEvent as *const OutEvent) };
        Some(out_event)
    } else {
        None
    }
}
