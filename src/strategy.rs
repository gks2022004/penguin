#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub trait Strategy {
    fn on_mid(&mut self, mid: f64) -> Signal;
}

pub struct SimpleMidStrategy {
    last_mid: Option<f64>,
    threshold: f64,
}

impl SimpleMidStrategy {
    pub fn new(threshold: f64) -> Self {
        Self {
            last_mid: None,
            threshold,
        }
    }
}

impl Strategy for SimpleMidStrategy {
    fn on_mid(&mut self, mid: f64) -> Signal {
        let signal = match self.last_mid {
            None => Signal::Hold,
            Some(prev) => {
                let diff = mid - prev;
                if diff >= self.threshold {
                    Signal::Buy
                } else if diff <= -self.threshold {
                    Signal::Sell
                } else {
                    Signal::Hold
                }
            }
        };

        self.last_mid = Some(mid);
        signal
    }
}
