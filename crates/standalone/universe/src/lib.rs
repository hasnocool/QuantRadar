//! Universe construction model.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct Universe { pub symbols: Vec<String> }
impl Universe { pub fn new() -> Self { Self { symbols: vec!["BTC/USD".into()] } } pub fn add(&mut self, s: &str) { self.symbols.push(s.into()); } }
#[cfg(test)] mod tests { use super::*; #[test] fn universe_add() { let mut u = Universe::new(); u.add("ETH/USD"); assert!(u.symbols.len() >= 1); } }
