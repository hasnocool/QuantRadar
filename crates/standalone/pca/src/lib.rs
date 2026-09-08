//! PCA clustering stub with basic variance.
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct PcaResult { pub variance: f64 }
impl PcaResult { pub fn compute(values: &[f64]) -> Self { let mean = values.iter().sum::<f64>()/values.len() as f64; let var = values.iter().map(|v|(v-mean).powi(2)).sum::<f64>()/values.len() as f64; Self{variance:var} } }
#[cfg(test)] mod tests { use super::*; #[test] fn pca_compute() { assert!(PcaResult::compute(&[1.0,2.0,3.0]).variance > 0.0); } }
