//! Derivatives data layer stub with basic contract.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct DerivativeContract { pub symbol: String, pub expiry: u64 }
impl DerivativeContract { pub fn new(s: &str, e: u64) -> Self { Self{symbol:s.into(), expiry:e} } }
#[cfg(test)] mod tests { use super::*; #[test] fn contract_create() { assert!(!DerivativeContract::new("BTC", 100).symbol.is_empty()); } }
