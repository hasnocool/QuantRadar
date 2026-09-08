//! Data/feature lineage tracking.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct LineageRecord { pub feature: String, pub source: String }
impl LineageRecord { pub fn new(f: &str, s: &str) -> Self { Self{feature:f.into(), source:s.into()} } }
#[cfg(test)] mod tests { use super::*; #[test] fn lineage_create() { let l = LineageRecord::new("mean","price"); assert!(!l.feature.is_empty()); } }
