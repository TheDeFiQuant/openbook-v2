use anyhow::Result;
use log::{info, warn, error};
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
    // Log the market pubkey
    info!("Fetching Market account data for: {}", market_pubkey);

    // Fetch the market account
    let market_account_data = match client.get_account_data(&market_pubkey).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch market account data: {}", e);
            return Err(e.into());
        }
    };

    let market: Market = match Market::try_deserialize(&mut &market_account_data[..]) {
        Ok(market) => market,
        Err(e) => {
            error!("Failed to deserialize market account data: {}", e);
            return Err(e.into());
        }
    };

    // Fetch the event heap for the given market
    let event_heap_pubkey = market.event_heap;
    info!("Fetching account data for event heap account: {}", event_heap_pubkey);

    let event_heap_account_data = match client.get_account_data(&event_heap_pubkey).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch event heap account data: {}", e);
            return Err(e.into());
        }
    };

    let event_heap: EventHeap = match EventHeap::try_deserialize(&mut &event_heap_account_data[..]) {
        Ok(event_heap) => event_heap,
        Err(e) => {
            error!("Failed to deserialize event heap data: {}", e);
            return Err(e.into());
        }
    };

    // Log the number of events in the event heap
    info!("EventHeap contains {} events", event_heap.len());

    let mut fill_events = Vec::new();

    // Iterate through the events in the event heap
    for (event, slot) in event_heap.iter() {
        info!("Processing event in slot {} with type: {}", slot, event.event_type);

        if let Some(fill_event) = event.as_fill_event() {
            info!("Found FillEvent: price = {}, quantity = {}", fill_event.price, fill_event.quantity);

            if fill_event.price != 0 && fill_event.quantity != 0 && fill_event.maker != Pubkey::default() && fill_event.taker != Pubkey::default() {
                let fill_info = FillEventInfo {
                    market_pubkey,
                    taker_side: fill_event.taker_side,  
                    maker_out: fill_event.maker_out as u8,  
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
        } else {
            warn!("Skipping non-FillEvent");
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
