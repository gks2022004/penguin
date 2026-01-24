use serde::Deserialize;
use reqwest::Client;
use crate::market::orderbook::OrderBook;

#[derive(Debug, Deserialize)]
pub struct DepthSnapshot {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}

pub async fn fetch_snapshot(symbol: &str) -> DepthSnapshot {
    let url = format!(
        "https://api.binance.com/api/v3/depth?symbol={}&limit=1000",
        symbol
    );

    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    res.json::<DepthSnapshot>().await.unwrap()
}

pub fn snapshot_to_orderbook(snapshot: DepthSnapshot) -> OrderBook {
    let mut book = OrderBook::new();
    book.last_update_id = snapshot.last_update_id;

    for [price, qty] in snapshot.bids {
        let p: f64 = price.parse().unwrap();
        let q: f64 = qty.parse().unwrap();
        book.bids.insert(p.into(), q);
    }

    for [price, qty] in snapshot.asks {
        let p: f64 = price.parse().unwrap();
        let q: f64 = qty.parse().unwrap();
        book.asks.insert(p.into(), q);
    }

    book
}