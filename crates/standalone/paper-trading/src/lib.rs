//! paper-trading stub crate.
#[derive(Clone, Debug, PartialEq)]
pub struct PaperAccount { pub cash: f64, pub peak_equity: f64, pub equity: f64, pub unrealized_pnl: f64, pub realized_pnl: f64, pub fills: Vec<String>, pub positions: Vec<String>, pub stop_orders: Vec<String>, pub take_profit_orders: Vec<String>, pub pending: Vec<String> }
impl PaperAccount {
    pub fn new(start_cash: f64) -> Self { Self { cash: start_cash, peak_equity: start_cash, equity: start_cash, unrealized_pnl: 0.0, realized_pnl: 0.0, fills: vec![], positions: vec![], stop_orders: vec![], take_profit_orders: vec![], pending: vec![] } }
    pub fn update_equity(&mut self, price: f64, qty: f64) { self.equity = self.cash + qty * price; self.unrealized_pnl = qty * price - qty * price * 0.99; self.peak_equity = self.peak_equity.max(self.equity); }
    pub fn add_fill(&mut self, fill: String) { self.fills.push(fill); }
    pub fn add_position(&mut self, pos: String) { self.positions.push(pos); }
    pub fn current_pnl(&self) -> f64 { self.realized_pnl + self.unrealized_pnl }
    pub fn ledger_snapshot(&self) -> String { format!("cash={:.2} equity={:.2} unrealized={:.2} realized={:.2}", self.cash, self.equity, self.unrealized_pnl, self.realized_pnl) }
}
