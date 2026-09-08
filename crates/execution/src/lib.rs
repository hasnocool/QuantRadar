// QuantRadar risk controls and paper-trading execution simulator. No live broker implementation here.
use chrono::{DateTime, Utc};
use quantaradar_core::{Direction, OrderBookSnapshot, Signal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimits { pub starting_equity: f64, pub max_position_pct: f64, pub max_portfolio_heat: f64, pub max_drawdown_pct: f64, pub max_concurrent_positions: usize, pub min_liquidity_score: f64 }
impl Default for RiskLimits { fn default()->Self{Self{starting_equity:100_000.0,max_position_pct:0.10,max_portfolio_heat:0.02,max_drawdown_pct:0.15,max_concurrent_positions:8,min_liquidity_score:0.65}} }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position { pub symbol:String, pub qty:f64, pub avg_price:f64, pub stop_price:f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderIntent { pub id:Uuid, pub ts:DateTime<Utc>, pub symbol:String, pub side:String, pub qty:f64, pub limit_price:Option<f64>, pub reason:String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill { pub order_id:Uuid, pub ts:DateTime<Utc>, pub symbol:String, pub side:String, pub qty:f64, pub price:f64, pub fee:f64 }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaperAccount { pub cash:f64, pub peak_equity:f64, pub equity:f64, pub realized_pnl:f64, pub fills:Vec<Fill>, pub positions:Vec<Position> }

pub fn size_position(equity:f64, entry:f64, stop:f64, risk_pct:f64, max_position_pct:f64) -> f64 {
    if entry<=0.0 || stop<=0.0 || entry<=stop { return 0.0; }
    let risk_dollars=equity*risk_pct; let risk_per_unit=entry-stop; let by_risk=risk_dollars/risk_per_unit; let by_notional=equity*max_position_pct/entry; by_risk.min(by_notional).max(0.0)
}

pub fn approve(signal:&Signal, liquidity_score:f64, equity:f64, active_positions:usize, limits:&RiskLimits, entry:f64, stop:f64)->Option<OrderIntent>{
    if signal.direction != Direction::Long || signal.score<=0.0 || liquidity_score<limits.min_liquidity_score || active_positions>=limits.max_concurrent_positions{return None;}
    let qty=size_position(equity,entry,stop,limits.max_portfolio_heat,limits.max_position_pct); if qty<=0.0{return None;}
    Some(OrderIntent{id:Uuid::new_v4(),ts:signal.ts,symbol:signal.symbol.clone(),side:"buy".into(),qty,limit_price:Some(entry),reason:signal.rationale.join("; ")})
}

pub fn paper_fill(order:&OrderIntent, book:&OrderBookSnapshot, fee_bps:f64)->Option<Fill>{
    let price=if order.side=="buy"{book.ask}else{book.bid}; if !price.is_finite()||price<=0.0{return None;} let fee=order.qty*price*fee_bps/10_000.0; Some(Fill{order_id:order.id,ts:order.ts,symbol:order.symbol.clone(),side:order.side.clone(),qty:order.qty,price,fee})
}

pub fn update_account(account:&mut PaperAccount, fill:&Fill){
    let signed=if fill.side=="buy"{1.0}else{-1.0}; let notional=fill.qty*fill.price; account.cash-=signed*notional+fill.fee; account.realized_pnl-=fill.fee; if let Some(p)=account.positions.iter_mut().find(|p|p.symbol==fill.symbol){ let old=p.qty; let new=(old+signed*fill.qty).max(0.0); if signed>0.0 && new>0.0 {p.avg_price=(p.avg_price*old+fill.price*fill.qty)/new;} p.qty=new; } else if signed>0.0 {account.positions.push(Position{symbol:fill.symbol.clone(),qty:fill.qty,avg_price:fill.price,stop_price:0.0});} account.fills.push(fill.clone());
}

#[cfg(test)]
mod tests { use super::*; #[test] fn sizing_respects_notional_cap(){assert!(size_position(100000.0,100.0,95.0,0.01,0.10)<=100.0);} }
