//! microstructure crate documentation.
// QuantRadar microstructure analytics: order-book/trade-flow models and executable-liquidity scoring.
use quantaradar_core::{OrderBookSnapshot, OrderSide, TradeTick};

#[derive(Clone, Debug, Default)]
pub struct MicrostructureFeatures {
    pub spread_bps: f64,
    pub bid_depth_usd: f64,
    pub ask_depth_usd: f64,
    pub depth_imbalance: f64,
    pub trade_buy_ratio: f64,
    pub trade_volume_usd: f64,
    pub avg_trade_usd: f64,
    pub impact_bps_1k: f64,
    pub impact_bps_10k: f64,
    pub impact_bps_100k: f64,
    pub liquidity_score: f64,
}

pub fn analyze(book: &OrderBookSnapshot, trades: &[TradeTick]) -> MicrostructureFeatures {
    let mid = if book.bid > 0.0 && book.ask > 0.0 { (book.bid + book.ask) / 2.0 } else { 0.0 };
    let spread_bps = if mid > 0.0 { (book.ask - book.bid) / mid * 10_000.0 } else { f64::INFINITY };
    let bid_depth_usd: f64 = book.bid_depth.iter().map(|(p,q)| p*q).sum();
    let ask_depth_usd: f64 = book.ask_depth.iter().map(|(p,q)| p*q).sum();
    let total_depth = bid_depth_usd + ask_depth_usd;
    let depth_imbalance = if total_depth > 0.0 { (bid_depth_usd - ask_depth_usd) / total_depth } else { 0.0 };
    let trade_volume_usd: f64 = trades.iter().map(|t| t.price*t.quantity).sum::<f64>();
    let buy_usd: f64 = trades.iter().filter(|t| t.side == OrderSide::Buy).map(|t| t.price*t.quantity).sum::<f64>();
    let trade_buy_ratio = if trade_volume_usd > 0.0 { buy_usd / trade_volume_usd } else { 0.5 };
    let avg_trade_usd = if !trades.is_empty() { trade_volume_usd / trades.len() as f64 } else { 0.0 };
    let impact = |size: f64, asks: bool| -> f64 {
        let levels = if asks { &book.ask_depth } else { &book.bid_depth };
        let mut remaining = size;
        let mut notional = 0.0;
        let mut qty = 0.0;
        for (price, level_qty) in levels {
            if remaining <= 0.0 { break; }
            let take = remaining.min(*level_qty);
            qty += take;
            notional += take * price;
            remaining -= take;
        }
        if qty <= 0.0 || mid <= 0.0 { return f64::INFINITY; }
        ((notional / qty) / mid - 1.0).abs() * 10_000.0
    };
    let i1 = impact(1_000.0 / mid.max(1e-12), true);
    let i10 = impact(10_000.0 / mid.max(1e-12), true);
    let i100 = impact(100_000.0 / mid.max(1e-12), true);
    let liquidity_score = (100.0 / (1.0 + spread_bps.max(0.0_f64))) * (1.0 + (total_depth.max(0.0_f64).log10()/8.0).clamp(0.0, 1.0)) / (1.0 + i10 / 10.0);
    MicrostructureFeatures { spread_bps, bid_depth_usd, ask_depth_usd, depth_imbalance, trade_buy_ratio, trade_volume_usd, avg_trade_usd, impact_bps_1k:i1, impact_bps_10k:i10, impact_bps_100k:i100, liquidity_score }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn feature_defaults_are_finite_except_unpriced_impact(){ let b=OrderBookSnapshot{ts:1,bid:100.0,ask:100.1,bid_depth:vec![(100.0,20.0)],ask_depth:vec![(100.1,20.0)]}; let t=vec![TradeTick{ts:1,price:100.1,quantity:2.0,side:OrderSide::Buy}]; let f=analyze(&b,&t); assert!(f.spread_bps > 0.0); assert!(f.trade_buy_ratio > 0.99); assert!(f.liquidity_score > 0.0); } }
