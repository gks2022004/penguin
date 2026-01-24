mod config;
mod exchange;
mod market;
mod infra;
mod ui;

#[tokio::main]
async fn main() {
let snapshot = exchange::binance::fetch_snapshot("BTCUSDT").await;
let book = exchange::binance::snapshot_to_orderbook(snapshot);


let (bid_p, bid_q) = book.best_bid().unwrap();
let (ask_p, ask_q) = book.best_ask().unwrap();


println!("BEST BID: {} @ {}", bid_p, bid_q);
println!("BEST ASK: {} @ {}", ask_p, ask_q);
println!("SPREAD: {}", ask_p - bid_p);
}