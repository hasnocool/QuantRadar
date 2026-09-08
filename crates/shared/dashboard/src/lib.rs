//! Dashboard / UI stub.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct DashMetric { pub label: String, pub value: f64 }
impl DashMetric { pub fn new(label: &str, value: f64) -> Self { Self { label: label.into(), value } } }
#[cfg(test)] mod tests { use super::*; #[test] fn metric_ok() { assert!(DashMetric::new("pnl", 10.0).value > 0.0); } }
