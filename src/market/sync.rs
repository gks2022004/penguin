use crate::exchange::binance::DepthEvent;
use crate::market::orderbook::{OrderBook, OrderedFloat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
	Applied,
	Ignored,
	Desync,
}

pub fn apply_delta(event: DepthEvent, book: &mut OrderBook) -> SyncStatus {
	if event.final_update_id <= book.last_update_id {
		return SyncStatus::Ignored;
	}

	let expected_next = book.last_update_id + 1;
	let in_range = event.first_update_id <= expected_next && event.final_update_id >= expected_next;
	if !in_range {
		return SyncStatus::Desync;
	}

	for [price, qty] in event.bids {
		let p: f64 = price.parse().unwrap();
		let q: f64 = qty.parse().unwrap();
		let key: OrderedFloat = p.into();
		if q == 0.0 {
			book.bids.remove(&key);
		} else {
			book.bids.insert(key, q);
		}
	}

	for [price, qty] in event.asks {
		let p: f64 = price.parse().unwrap();
		let q: f64 = qty.parse().unwrap();
		let key: OrderedFloat = p.into();
		if q == 0.0 {
			book.asks.remove(&key);
		} else {
			book.asks.insert(key, q);
		}
	}

	book.last_update_id = event.final_update_id;
	SyncStatus::Applied
}
