//! Multi-exchange architecture stub.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct ExchangeRef { pub name: String }
impl ExchangeRef { pub fn list() -> Vec<Self> { vec![Self{name:"kraken".into()}, Self{name:"binance".into()}] } }
#[cfg(test)] mod tests { use super::*; #[test] fn exchanges_exist() { assert!(!ExchangeRef::list().is_empty()); } }
