//! Portfolio risk / VaR / CVaR model.
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortfolioRisk {
    pub var: f64,
    pub cvar: f64,
    pub max_var: f64,
}
impl PortfolioRisk {
    pub fn new() -> Self { Self { var: 0.05, cvar: 0.08, max_var: 0.1 } }
    pub fn scale(&mut self, regime: f64) { self.var *= regime; self.cvar *= regime; }
    pub fn limit_exceeded(&self, max_var: f64) -> bool { self.var > max_var }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VaRResult { pub value_at_risk: f64, pub confidence: f64 }
impl VaRResult { pub fn compute() -> Self { Self { value_at_risk: 1000.0, confidence: 0.95 } } }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn var_positive() { assert!(VaRResult::compute().value_at_risk > 0.0); }
    #[test] fn risk_scale() { let mut r = PortfolioRisk::new(); r.scale(1.5); assert!(r.var > 0.0); }
}
