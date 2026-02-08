#[derive(Debug, Clone)]
pub struct AppConfig {
	pub symbol: String,
	pub mid_threshold: f64,
	pub max_position: f64,
	pub max_order_size: f64,
}

impl AppConfig {
	pub fn from_env() -> Self {
		let symbol = std::env::var("PENGUIN_SYMBOL").unwrap_or_else(|_| "BTCUSDT".to_string());
		let mid_threshold = std::env::var("PENGUIN_MID_THRESHOLD")
			.ok()
			.and_then(|v| v.parse().ok())
			.unwrap_or(0.5);
		let max_position = std::env::var("PENGUIN_MAX_POSITION")
			.ok()
			.and_then(|v| v.parse().ok())
			.unwrap_or(1.0);
		let max_order_size = std::env::var("PENGUIN_MAX_ORDER_SIZE")
			.ok()
			.and_then(|v| v.parse().ok())
			.unwrap_or(0.1);

		Self {
			symbol,
			mid_threshold,
			max_position,
			max_order_size,
		}
	}
}
