use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;

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

#[derive(Debug, Deserialize)]
pub struct DepthEvent {
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<[String; 2]>,
    #[serde(rename = "a")]
    pub asks: Vec<[String; 2]>,
}

pub async fn stream_depth(symbol: &str, sender: tokio::sync::mpsc::Sender<DepthEvent>) {
    let url = format!(
        "wss://stream.binance.com:9443/ws/{}@depth@100ms",
        symbol.to_lowercase()
    );

    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(30);

    loop {
        let (ws, _) = match connect_async(&url).await {
            Ok(result) => {
                backoff = Duration::from_secs(1);
                result
            }
            Err(_) => {
                sleep(backoff).await;
                backoff = (backoff * 2).min(max_backoff);
                continue;
            }
        };

        let (_, mut read) = ws.split();

        while let Some(msg) = read.next().await {
            let text = match msg {
                Ok(message) => match message.into_text() {
                    Ok(text) => text,
                    Err(_) => continue,
                },
                Err(_) => break,
            };

            let value: Value = match serde_json::from_str(&text) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if value.get("b").is_none() || value.get("a").is_none() {
                continue;
            }

            let event: DepthEvent = match serde_json::from_value(value) {
                Ok(event) => event,
                Err(_) => continue,
            };

            if sender.send(event).await.is_err() {
                return;
            }
        }

        sleep(backoff).await;
        backoff = (backoff * 2).min(max_backoff);
    }
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