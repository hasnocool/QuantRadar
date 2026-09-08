//! Data-quality validation, metric computation, and anomaly flagging.
//!
//! Provides deterministic checks on market bars and derived features:
//! completeness (no missing bars), consistency (OHLC ordering),
//! and simple anomaly flags (zero volume, extreme range).

use serde::{Deserialize, Serialize};

/// Quality report for a stream segment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QualityReport {
    pub completeness: f64,
    pub consistency_violations: usize,
    pub anomaly_flags: usize,
}

/// Compute basic quality metrics over a bar slice (stub for full pass).
pub fn assess_quality() -> QualityReport {
    QualityReport::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quality_default() {
        let r = assess_quality();
        assert_eq!(r.completeness, 0.0);
    }
}
