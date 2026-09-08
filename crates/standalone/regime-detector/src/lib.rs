//! Regime-detector: classify market regime (bull/bear/neutral) from price returns.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Regime { Bull, Bear, Neutral, Unknown }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegimeDetector {
    pub enabled: bool,
    pub config: String,
    window: usize,
}

impl RegimeDetector {
    pub fn new() -> Self { Self { enabled: true, config: "default".into(), window: 20 } }
    pub fn detect(&self, closes: &[f64]) -> Regime {
        if closes.len() < 2 { return Regime::Unknown; }
        let first = closes[0];
        let last = closes[closes.len()-1];
        let ret = (last - first) / first.abs().max(1e-9);
        if ret > 0.05 { Regime::Bull }
        else if ret < -0.05 { Regime::Bear }
        else { Regime::Neutral }
    }
    pub fn process(&self) -> anyhow::Result<String> { Ok("regime-detected".into()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detect_bull() { assert_eq!(RegimeDetector::new().detect(&[100.0, 110.0]), Regime::Bull); }
}
#[cfg(test)] mod regime_load_tests { use super::*; #[test] fn regime_load_stable() { assert_eq!(RegimeDetector::new().detect(&[100.0, 110.0]), Regime::Bull); } }
