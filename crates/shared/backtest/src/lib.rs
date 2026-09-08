//! backtest crate documentation.
// Cost-aware deterministic event-driven backtester with realistic execution.
use chrono::{DateTime,Utc};
use quantaradar_core::Bar;
use quantaradar_microstructure::MicrostructureFeatures;
use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct BacktestConfig {
    pub initial_cash: f64,
    pub fee_bps: f64,
    pub slippage_bps: f64,
    pub fast: usize,
    pub slow: usize,
    pub min_liquidity_score: f64,
    pub max_position_pct: f64,
    pub partial_fill_probability: f64,
    pub latency_ms: u64,
}
impl Default for BacktestConfig {
    fn default()->Self{Self{
        initial_cash:10_000.0,
        fee_bps:8.0,
        slippage_bps:3.0,
        fast:20,
        slow:50,
        min_liquidity_score:0.65,
        max_position_pct:0.10,
        partial_fill_probability:0.1,
        latency_ms:100,
    }}
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct BacktestResult {
    pub initial_cash: f64,
    pub final_cash: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
    pub trades: usize,
    pub win_rate: f64,
    pub equity: Vec<(DateTime<Utc>,f64)>,
    pub sharpe: f64,
    pub sortino: f64,
    pub profit_factor: f64,
    pub avg_trade_pnl: f64,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TradeRecord {
    pub entry_time: DateTime<Utc>,
    pub exit_time: DateTime<Utc>,
    pub entry_price: f64,
    pub exit_price: f64,
    pub qty: f64,
    pub pnl: f64,
    pub fees: f64,
    pub slippage: f64,
    pub was_partial: bool,
}

fn sma(xs:&[f64],p:usize)->Option<f64>{if xs.len()<p{None}else{Some(xs[xs.len()-p..].iter().sum::<f64>()/p as f64)}}

fn liquidity_gate(liq: &MicrostructureFeatures, min_score: f64) -> bool {
    liq.liquidity_score >= min_score
}

fn simulate_partial_fill(qty: f64, prob: f64) -> (f64, bool) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    (qty as u64).hash(&mut h);
    let val = h.finish() as f64 / u64::MAX as f64;
    if val < prob && qty > 1.0 {
        let fill_ratio = 0.3 + (val / prob) * 0.7;
        (qty * fill_ratio, true)
    } else {
        (qty, false)
    }
}

pub fn run(bars:&[Bar], c:&BacktestConfig, liquidity: Option<&[MicrostructureFeatures]>) -> BacktestResult {
    let mut cash = c.initial_cash;
    let mut qty = 0.0;
    let mut peak = c.initial_cash;
    let mut max_dd: f64 = 0.0;
    let mut entry = 0.0;
    let (mut wins, mut trades) = (0usize, 0usize);
    let mut eq = Vec::with_capacity(bars.len());
    let mut trade_records = Vec::new();
    let mut returns = Vec::new();

    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();

    for i in 0..bars.len() {
        let fast = sma(&closes[..=i], c.fast);
        let slow = sma(&closes[..=i], c.slow);
        let price = bars[i].close;

        let liq_ok = liquidity.map_or(true, |l| {
            l.get(i).map_or(true, |liq| liquidity_gate(liq, c.min_liquidity_score))
        });

        let long = fast.zip(slow).map(|(a,b)| a > b).unwrap_or(false);

        if long && qty == 0.0 && liq_ok {
            let slippage = c.slippage_bps / 10_000.0;
            let fee = c.fee_bps / 10_000.0;
            let px = price * (1.0 + slippage);
            let max_qty = cash * c.max_position_pct / px;
            let (fill_qty, was_partial) = simulate_partial_fill(max_qty, c.partial_fill_probability);
            let cost = fill_qty * px * (1.0 + fee);
            cash -= cost;
            qty = fill_qty;
            entry = px;
            trades += 1;
            trade_records.push(TradeRecord {
                entry_time: bars[i].ts,
                exit_time: bars[i].ts, // placeholder
                entry_price: px,
                exit_price: 0.0,
                qty: fill_qty,
                pnl: 0.0,
                fees: fill_qty * px * fee,
                slippage: fill_qty * px * slippage,
                was_partial,
            });
        } else if (!long || !liq_ok) && qty > 0.0 {
            let slippage = c.slippage_bps / 10_000.0;
            let fee = c.fee_bps / 10_000.0;
            let px = price * (1.0 - slippage);
            let proceeds = qty * px * (1.0 - fee);
            cash += proceeds;
            let pnl = (px - entry) * qty;
            if px > entry { wins += 1; }
            returns.push(pnl);

            if let Some(last_trade) = trade_records.last_mut() {
                last_trade.exit_time = bars[i].ts;
                last_trade.exit_price = px;
                last_trade.pnl = pnl;
                last_trade.fees += qty * px * fee;
                last_trade.slippage += qty * px * slippage;
            }
            qty = 0.0;
            entry = 0.0;
        }

        let equity = cash + qty * price;
        peak = peak.max(equity);
        max_dd = max_dd.max((peak - equity) / peak.max(1.0));
        eq.push((bars[i].ts, equity));
    }

    if qty > 0.0 {
        let price = *closes.last().unwrap_or(&0.0);
        let fee = c.fee_bps / 10_000.0;
        let proceeds = qty * price * (1.0 - fee);
        cash += proceeds;
        let pnl = (price - entry) * qty;
        if price > entry { wins += 1; }
        returns.push(pnl);
        if let Some(last_trade) = trade_records.last_mut() {
            last_trade.exit_time = bars.last().map(|b| b.ts).unwrap_or_else(Utc::now);
            last_trade.exit_price = price;
            last_trade.pnl = pnl;
            last_trade.fees += qty * price * fee;
        }
    }

    let total_return = cash / c.initial_cash - 1.0;
    let win_rate = if trades == 0 { 0.0 } else { wins as f64 / trades as f64 };
    let avg_trade_pnl = if returns.is_empty() { 0.0 } else { returns.iter().sum::<f64>() / returns.len() as f64 };
    let gross_profit = returns.iter().filter(|&&x| x > 0.0).sum::<f64>();
    let gross_loss = returns.iter().filter(|&&x| x < 0.0).sum::<f64>().abs();
    let profit_factor = if gross_loss > 0.0 { gross_profit / gross_loss } else { f64::INFINITY };

    let returns_mean = returns.iter().sum::<f64>() / returns.len().max(1) as f64;
    let returns_std = (returns.iter().map(|x| (x - returns_mean).powi(2)).sum::<f64>() / returns.len().max(1) as f64).sqrt();
    let sharpe = if returns_std > 0.0 { returns_mean / returns_std * (252.0_f64).sqrt() } else { 0.0 };
    let downside_std = (returns.iter().filter(|&&x| x < 0.0).map(|x| x.powi(2)).sum::<f64>() / returns.len().max(1) as f64).sqrt();
    let sortino = if downside_std > 0.0 { returns_mean / downside_std * (252.0_f64).sqrt() } else { 0.0 };

    BacktestResult {
        initial_cash: c.initial_cash,
        final_cash: cash,
        total_return,
        max_drawdown: max_dd,
        trades,
        win_rate,
        equity: eq,
        sharpe,
        sortino,
        profit_factor,
        avg_trade_pnl,
    }
}
