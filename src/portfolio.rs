pub struct Portfolio {
    pub position: f64,
    pub cash: f64,
}

impl Portfolio {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            cash: 0.0,
        }
    }

    pub fn unrealized_pnl(&self, mark_price: f64) -> f64 {
        self.cash + self.position * mark_price
    }
}
