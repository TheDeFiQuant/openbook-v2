use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use openbook_v2::state::{EventHeap, Market, FillEvent, EventType, AnyEvent};
use anchor_lang::AccountDeserialize;

/// Struct to hold the information about a fetched fill event
pub struct FillEventInfo {
    pub market_pubkey: Pubkey,
    pub taker_side: u8,
    pub maker_out: u8,
    pub maker_slot: u8,
    pub timestamp: u64,
    pub market_seq_num: u64,
    pub maker: Pubkey,
    pub maker_timestamp: u64,
    pub taker: Pubkey,
    pub taker_client_order_id: u64,
    pub price: i64,
    pub peg_limit: i64,
    pub quantity: i64,
    pub maker_client_order_id: u64,
}

/// Fetch all fill events (matched trades) for a given market
pub async fn fetch_fill_events(client: &RpcClient, market_pubkey: Pubkey) -> Result<Vec<FillEventInfo>> {
    // Fetch the market account
    let market_account_data = client.get_account_data(&market_pubkey).await?;
    let market: Market = Market::try_deserialize(&mut &market_account_data[..])?;

    // Fetch the event heap for the given market
    let event_heap_pubkey = market.event_heap;
    let event_heap_account_data = client.get_account_data(&event_heap_pubkey).await?;
    let event_heap: EventHeap = EventHeap::try_deserialize(&mut &event_heap_account_data[..])?;

    let mut fill_events = Vec::new();

    // Iterate through the events in the event heap
    for fill_event in event_heap.iter_fill_events() {
        // Filter out invalid events
        if fill_event.price != 0 && fill_event.quantity != 0 && fill_event.maker != Pubkey::default() && fill_event.taker != Pubkey::default() {
            let fill_info = FillEventInfo {
                market_pubkey,
                taker_side: fill_event.taker_side,  
                maker_out: fill_event.maker_out,  
                maker_slot: fill_event.maker_slot,  
                timestamp: fill_event.timestamp,  
                market_seq_num: fill_event.market_seq_num,  
                maker: fill_event.maker,  
                maker_timestamp: fill_event.maker_timestamp,  
                taker: fill_event.taker,  
                taker_client_order_id: fill_event.taker_client_order_id,  
                price: fill_event.price,  
                peg_limit: fill_event.peg_limit,  
                quantity: fill_event.quantity,  
                maker_client_order_id: fill_event.maker_client_order_id,  
            };

            fill_events.push(fill_info);
        }
    }

    Ok(fill_events)
}

/// Trait to iterate through fill events in the EventHeap
pub trait EventHeapExt {
    fn iter_fill_events(&self) -> Box<dyn Iterator<Item = FillEvent> + '_>;
}

impl EventHeapExt for EventHeap {
    // Method to iterate only through fill events
    fn iter_fill_events(&self) -> Box<dyn Iterator<Item = FillEvent> + '_> {
        Box::new(self.nodes.iter().filter_map(|node| {
            node.event.as_fill_event()
        }))
    }
}

/// Trait to extend AnyEvent to provide a method for converting to FillEvent
pub trait AnyEventExt {
    fn as_fill_event(&self) -> Option<FillEvent>;
}

impl AnyEventExt for AnyEvent {
    fn as_fill_event(&self) -> Option<FillEvent> {
        if self.event_type == EventType::Fill as u8 {
            // Cast the event to FillEvent if it's a Fill event
            Some(unsafe { *(self as *const AnyEvent as *const FillEvent) })
        } else {
            None
        }
    }
}
