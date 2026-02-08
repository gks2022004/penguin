mod config;
mod exchange;
mod market;
mod infra;
mod ui;
mod strategy;
mod risk;
mod execution;
mod portfolio;

use crate::strategy::Strategy;

#[tokio::main]
async fn main() {
	let (tx, mut rx) = tokio::sync::mpsc::channel(1000);

	tokio::spawn(exchange::binance::stream_depth("BTCUSDT", tx));

	let mut buffered = Vec::new();
	while let Some(event) = rx.recv().await {
		buffered.push(event);
		if buffered.len() >= 50 {
			break;
		}
	}

	let snapshot = exchange::binance::fetch_snapshot("BTCUSDT").await;
	let mut book = exchange::binance::snapshot_to_orderbook(snapshot);
	let mut last_mid: Option<f64> = None;
	let mut strategy = strategy::SimpleMidStrategy::new(0.5);
	let risk = risk::RiskEngine::new(1.0, 0.1);
	let execution = execution::ExecutionEngine::new();
	let mut portfolio = portfolio::Portfolio::new();

	buffered.sort_by_key(|e| e.final_update_id);
	for event in buffered {
		match market::sync::apply_delta(event, &mut book) {
			market::sync::SyncStatus::Applied => {}
			market::sync::SyncStatus::Ignored => {}
			market::sync::SyncStatus::Desync => {
				println!("DESYNC — rebuilding order book");
				let snapshot = exchange::binance::fetch_snapshot("BTCUSDT").await;
				book = exchange::binance::snapshot_to_orderbook(snapshot);
				last_mid = None;
			}
		}
	}

	loop {
		if let Some(event) = rx.recv().await {
			match market::sync::apply_delta(event, &mut book) {
				market::sync::SyncStatus::Applied => {
					if let (Some((bp, _)), Some((ap, _))) = (book.best_bid(), book.best_ask()) {
						let mid = (bp + ap) / 2.0;
						let changed = last_mid.map_or(true, |prev| (mid - prev).abs() > 0.0);
						if changed {
							println!("MID: {:.2}", mid);
							last_mid = Some(mid);
							let signal = strategy.on_mid(mid);
							if let Some(order) = risk.evaluate(signal, portfolio.position) {
								let fill = execution.execute(order, mid, &mut portfolio);
								let pnl = portfolio.unrealized_pnl(mid);
								println!(
									"FILL: {:?} {:.4} @ {:.2} | POS: {:.4} | PNL: {:.2}",
									fill.side,
									fill.qty,
									fill.price,
									portfolio.position,
									pnl
								);
							}
						}
					}
				}
				market::sync::SyncStatus::Ignored => {}
				market::sync::SyncStatus::Desync => {
					println!("DESYNC — rebuilding order book");
					let snapshot = exchange::binance::fetch_snapshot("BTCUSDT").await;
					book = exchange::binance::snapshot_to_orderbook(snapshot);
					last_mid = None;
					strategy = strategy::SimpleMidStrategy::new(0.5);
				}
			}
		}
	}
}