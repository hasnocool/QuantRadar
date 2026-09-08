//! Full L2 order-book model: depth-level bids/asks, delta/rebuild, imbalance, liquidity.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct L2Depth {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderBookL2 {
    pub bids: Vec<L2Depth>,
    pub asks: Vec<L2Depth>,
    pub timestamp_ms: u64,
    pub sequence: u64,
}

impl OrderBookL2 {
    pub fn new(bids: Vec<L2Depth>, asks: Vec<L2Depth>, ts: u64, seq: u64) -> Self {
        Self { bids, asks, timestamp_ms: ts, sequence: seq }
    }
    pub fn spread(&self) -> f64 {
        let best_ask = self.asks.first().map(|a| a.price).unwrap_or(0.0);
        let best_bid = self.bids.first().map(|b| b.price).unwrap_or(0.0);
        if best_ask > best_bid { best_ask - best_bid } else { 0.0 }
    }
    pub fn mid(&self) -> f64 {
        let best_ask = self.asks.first().map(|a| a.price).unwrap_or(0.0);
        let best_bid = self.bids.first().map(|b| b.price).unwrap_or(0.0);
        if best_ask > 0.0 && best_bid > 0.0 { (best_bid + best_ask) / 2.0 } else { 0.0 }
    }
    pub fn depth_imbalance(&self) -> f64 {
        let bid_qty: f64 = self.bids.iter().map(|b| b.qty).sum();
        let ask_qty: f64 = self.asks.iter().map(|a| a.qty).sum();
        let total = bid_qty + ask_qty;
        if total > 0.0 { (bid_qty - ask_qty) / total } else { 0.0 }
    }
    pub fn apply_delta(&mut self, price: f64, qty: f64, side: OrderSide) {
        let levels = if side == OrderSide::Buy { &mut self.bids } else { &mut self.asks };
        if qty <= 0.0 {
            levels.retain(|l| l.price != price);
        } else {
            if let Some(l) = levels.iter_mut().find(|l| l.price == price) {
                l.qty = qty;
            } else {
                levels.push(L2Depth { price, qty });
                levels.sort_by(|a, b| if side == OrderSide::Buy { b.price.partial_cmp(&a.price).unwrap() } else { a.price.partial_cmp(&b.price).unwrap() });
            }
        }
    }
    pub fn rebuild(&self) -> Self { self.clone() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide { Buy, Sell }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn l2_spread_and_imbalance() {
        let book = OrderBookL2::new(
            vec![L2Depth { price: 100.0, qty: 1.0 }, L2Depth { price: 99.9, qty: 2.0 }],
            vec![L2Depth { price: 100.1, qty: 1.5 }], 1, 1,
        );
        assert_eq!(book.spread(), 0.1);
        assert!(book.depth_imbalance().abs() < 1.0);
    }
    #[test]
    fn delta_removes_level() {
        let mut b = OrderBookL2::new(vec![L2Depth { price: 100.0, qty: 1.0 }], vec![], 1, 1);
        b.apply_delta(100.0, 0.0, OrderSide::Buy);
        assert!(b.bids.is_empty());
    }
}
