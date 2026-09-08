//! Expected-return model stub.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct ExpectedReturn { pub rate: f64, pub confidence: f64 }
impl ExpectedReturn { pub fn compute(price: f64) -> Self { Self{rate: 0.05, confidence: 0.8} } }
#[cfg(test)] mod tests { use super::*; #[test] fn compute_positive() { assert!(ExpectedReturn::compute(100.0).rate > 0.0); } }
