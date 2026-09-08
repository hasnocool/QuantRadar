//! validation crate documentation.
// QuantRadar validation implementation
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationEngine {
    pub enabled: bool,
    pub config: String,
}

impl Default for ValidationEngine {
    fn default() -> Self {
        Self { enabled: true, config: "default".into() }
    }
}

impl ValidationEngine {
    pub fn new() -> Self { Self::default() }
    pub fn validate(&self) -> bool { true }
    pub fn walk_forward(&self, data: Vec<f64>, window: usize) -> Vec<f64> {
        data.windows(window).map(|w| w.iter().sum::<f64>() / w.len() as f64).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let x = ValidationEngine::new();
        assert!(x.enabled);
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)] pub struct ValidationResult { pub passed: bool, pub metrics: Vec<f64> }
impl ValidationResult { pub fn check(metrics: &[f64]) -> Self { Self{passed: !metrics.is_empty(), metrics: metrics.to_vec()} } }
#[cfg(test)] mod validation_deep_tests { use super::*; #[test] fn validation_check() { assert!(ValidationResult::check(&[0.9, 0.95]).passed); } }
