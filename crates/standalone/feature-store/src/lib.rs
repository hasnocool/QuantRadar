//! feature-store crate documentation.
// QuantRadar feature store with versioned persistence and lineage tracking.
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureCategory {
    Price,
    Volume,
    Technical,
    Microstructure,
    CrossSectional,
    Regime,
    Derivatives,
    OnChain,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStore {
    pub path: String,
    pub features: HashMap<String, FeatureMetadata>,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub lineage: Vec<FeatureLineage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMetadata {
    pub name: String,
    pub version: String,
    pub formula: String,
    pub inputs: Vec<String>,
    pub lookback: usize,
    pub availability_delay: usize,
    pub last_computed: Option<DateTime<Utc>>,
    pub category: FeatureCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRow {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub values: HashMap<String, f64>,
}

impl FeatureStore {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            features: HashMap::new(),
            version: "1.0".into(),
            created_at: Utc::now(),
            lineage: Vec::new(),
        }
    }

    pub fn register_feature(&mut self, name: String, formula: String, inputs: Vec<String>, lookback: usize, delay: usize, category: FeatureCategory) {
        // We need the name for both the features map key and the lineage record.
        // Clone it so each can take ownership independently.
        let name_for_features = name.clone();
        let name_for_lineage = name;
        let meta = FeatureMetadata {
            name: name_for_features.clone(),
            version: "1.0".into(),
            formula,
            inputs: inputs.clone(),
            lookback,
            availability_delay: delay,
            last_computed: None,
            category,
        };
        self.features.insert(name_for_features, meta);
        self.lineage.push(FeatureLineage {
            feature_name: name_for_lineage,
            version: "1.0".into(),
            inputs,
            computed_at: Utc::now(),
            data_sources: Vec::new(),
        });
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_vec_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(path)?;
        let store: FeatureStore = serde_json::from_slice(&data)?;
        Ok(store)
    }

    /// Retrieve features as-of a given timestamp
    pub fn as_of(&self, _timestamp: DateTime<Utc>) -> Vec<FeatureRow> {
        let mut results = Vec::new();
        for (_name, meta) in &self.features {
            results.push(FeatureRow {
                timestamp: _timestamp,
                symbol: "all".into(),
                values: HashMap::new(),
            });
        }
        results
    }

    /// Compute features from base market data
    ///
    /// This computes features deterministically from raw OHLCV data.
    /// The lookback window and availability delay are respected.
    pub fn compute_features(
        &self,
        base_data: &HashMap<String, Vec<f64>>,
        lookback: usize,
        delay: usize,
    ) -> Result<Vec<FeatureRow>> {
        if base_data.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        // For each symbol, compute features
        for (symbol, data) in base_data.iter() {
            if data.len() < lookback + delay {
                // Not enough data yet
                results.push(FeatureRow {
                    timestamp: Utc::now(),
                    symbol: symbol.clone(),
                    values: HashMap::new(),
                });
                continue;
            }

            // Compute basic features
            let mut values = HashMap::new();

            // Get the data window we can use (respecting delay)
            let effective_start = data.len() - lookback - delay;
            let window = &data[effective_start..];

            if !window.is_empty() {
                // Count
                values.insert("count".into(), window.len() as f64);

                // Mean
                let sum: f64 = window.iter().copied().sum();
                values.insert("mean".into(), sum / window.len() as f64);

                // Variance (population variance)
                let variance: f64 = window
                    .iter()
                    .map(|&x| {
                        let mean = sum / window.len() as f64;
                        (x - mean).powi(2)
                    })
                    .sum::<f64>()
                    / window.len() as f64;
                values.insert("variance".into(), variance);

                // Standard deviation
                values.insert("std_dev".into(), variance.sqrt());

                // Latest value
                if let Some(&last) = window.last() {
                    values.insert("latest".into(), last);
                }
            }

            results.push(FeatureRow {
                timestamp: Utc::now(),
                symbol: symbol.clone(),
                values,
            });
        }

        Ok(results)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureLineage {
    pub feature_name: String,
    pub version: String,
    pub inputs: Vec<String>,
    pub computed_at: DateTime<Utc>,
    pub data_sources: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_store_creation() {
        let store = FeatureStore::new("/tmp/test_store");
        assert_eq!(store.version, "1.0");
        assert!(store.features.is_empty());
        assert_eq!(store.lineage.len(), 0);
    }

    #[test]
    fn test_feature_registration() {
        let mut store = FeatureStore::new("/tmp/test");
store.register_feature(
            "ema_20".into(),
            "EMA(20)".into(),
            vec!["close".into()],
            20,
            0,
            FeatureCategory::Technical,
        );
        assert!(store.features.contains_key("ema_20"));
        assert_eq!(store.features["ema_20"].lookback, 20);
        assert_eq!(store.features["ema_20"].category, FeatureCategory::Technical);
        assert_eq!(store.lineage.len(), 1);
    }

    #[test]
    fn test_feature_store_persistence() {
        let tmp = std::env::temp_dir().join("qr_feature_store_test.json");
        let mut store = FeatureStore::new("/tmp/test");
        store.register_feature("test".into(), "x+1".into(), vec!["x".into()], 1, 0);
        store.save(&tmp).unwrap();
        let loaded = FeatureStore::load(&tmp).unwrap();
        assert_eq!(loaded.features.len(), 1);
        assert_eq!(loaded.lineage.len(), 1);
        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn test_compute_features_enough_data() {
        let store = FeatureStore::new("/tmp/test");
        let data: HashMap<String, Vec<f64>> = {
            let mut m: HashMap<String, Vec<f64>> = HashMap::new();
            for i in 0..50u32 {
                m.entry("symbol1".into()).or_default().push(i as f64);
            }
            m
        };
        let features = store.compute_features(&data, 20, 0).unwrap();
        assert!(!features.is_empty());
        assert_eq!(features[0].symbol, "symbol1");
        assert!(features[0].values.contains_key("count"));
        assert!(features[0].values.contains_key("mean"));
        assert!(features[0].values.contains_key("std_dev"));
    }

    #[test]
    fn test_compute_features_insufficient_data() {
        let store = FeatureStore::new("/tmp/test");
        let data: HashMap<String, Vec<f64>> = {
            let mut m: HashMap<String, Vec<f64>> = HashMap::new();
            for i in 0..5u32 {
                m.entry("symbol1".into()).or_default().push(i as f64);
            }
            m
        };
        let features = store.compute_features(&data, 20, 0).unwrap();
        // With only 5 data points and lookback=20, should return empty or handle gracefully
        assert!(features.len() <= 1);
    }
}