use crate::execution::{Order, Side};
use crate::strategy::Signal;

pub struct RiskEngine {
    max_position: f64,
    max_order_size: f64,
}

impl RiskEngine {
    pub fn new(max_position: f64, max_order_size: f64) -> Self {
        Self {
            max_position,
            max_order_size,
        }
    }

    pub fn evaluate(&self, signal: Signal, position: f64) -> Option<Order> {
        let qty = self.max_order_size;
        match signal {
            Signal::Buy => {
                if position + qty <= self.max_position {
                    Some(Order {
                        side: Side::Buy,
                        qty,
                    })
                } else {
                    None
                }
            }
            Signal::Sell => {
                if position - qty >= -self.max_position {
                    Some(Order {
                        side: Side::Sell,
                        qty,
                    })
                } else {
                    None
                }
            }
            Signal::Hold => None,
        }
    }
}
