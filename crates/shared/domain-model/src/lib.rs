//! domain-model crate documentation.
use quantaradar_core::{Direction, OrderSide};

#[derive(Clone, Debug, PartialEq)]
pub struct Trade { pub symbol: String, pub size: f64, pub price: f64, pub direction: Direction, pub time: String }
pub struct Position { pub symbol: String, pub qty: f64, pub avg_price: f64, pub unrealized_pnl: f64 }
impl Trade { pub fn new(s: String, qty: f64, p: f64, dir: Direction) -> Self { Self { symbol: s, size: qty, price: p, direction: dir, time: "now".into() } } pub fn direction_str(&self) -> String { self.direction.to_string() } }
