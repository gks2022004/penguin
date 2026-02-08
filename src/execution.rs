use crate::portfolio::Portfolio;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub side: Side,
    pub qty: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Fill {
    pub side: Side,
    pub qty: f64,
    pub price: f64,
}

pub struct ExecutionEngine;

impl ExecutionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, order: Order, price: f64, portfolio: &mut Portfolio) -> Fill {
        match order.side {
            Side::Buy => {
                portfolio.position += order.qty;
                portfolio.cash -= order.qty * price;
            }
            Side::Sell => {
                portfolio.position -= order.qty;
                portfolio.cash += order.qty * price;
            }
        }

        Fill {
            side: order.side,
            qty: order.qty,
            price,
        }
    }
}
