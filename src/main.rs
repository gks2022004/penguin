mod config;
mod exchange;
mod market;
mod infra;
mod ui;

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

	buffered.sort_by_key(|e| e.final_update_id);
	for event in buffered {
		match market::sync::apply_delta(event, &mut book) {
			market::sync::ApplyDeltaOutcome::Applied => {}
			market::sync::ApplyDeltaOutcome::Skipped => {}
			market::sync::ApplyDeltaOutcome::Desync => {
				println!("DESYNC DETECTED – resyncing snapshot");
				let snapshot = exchange::binance::fetch_snapshot("BTCUSDT").await;
				book = exchange::binance::snapshot_to_orderbook(snapshot);
			}
		}
	}

	loop {
		if let Some(event) = rx.recv().await {
			match market::sync::apply_delta(event, &mut book) {
				market::sync::ApplyDeltaOutcome::Applied => {
					if let (Some((bp, _)), Some((ap, _))) = (book.best_bid(), book.best_ask()) {
						let mid = (bp + ap) / 2.0;
						let changed = last_mid.map_or(true, |prev| (mid - prev).abs() > 0.0);
						if changed {
							println!("MID: {:.2}", mid);
							last_mid = Some(mid);
						}
					}
				}
				market::sync::ApplyDeltaOutcome::Skipped => {}
				market::sync::ApplyDeltaOutcome::Desync => {
					println!("DESYNC DETECTED – resyncing snapshot");
					let snapshot = exchange::binance::fetch_snapshot("BTCUSDT").await;
					book = exchange::binance::snapshot_to_orderbook(snapshot);
					last_mid = None;
				}
			}
		}
	}
}