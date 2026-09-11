//! Cross-sectional factor engine.
//! Implements winsorization, cross-sectional normalization, sector neutralization,
//! factor orthogonalization, rank transforms, score calibration, turnover-aware
//! and liquidity-aware ranking, exposure constraints, and uncertainty estimates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Errors for factor engine
#[derive(Debug, thiserror::Error)]
pub enum FactorEngineError {
    #[error("Insufficient observations: need at least {min}", min = 30)]
    InsufficientObservations,
    #[error("No sector map provided for neutralization")]
    NoSectorMap,
    #[error("Factor computation failed: {0}")]
    ComputationFailed(String),
}

/// Configuration for factor engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorEngineConfig {
    /// Winsorization percentile (e.g., 0.01 for 1% tails)
    pub winsorization_pct: f64,
    /// Whether to apply sector neutralization
    pub sector_neutralize: bool,
    /// Whether to orthogonalize factors (Gram-Schmidt)
    pub orthogonalize: bool,
    /// Minimum observations for cross-sectional stats
    pub min_obs: usize,
    /// Turnover penalty weight
    pub turnover_penalty: f64,
    /// Liquidity weight
    pub liquidity_weight: f64,
    /// Max position concentration
    pub max_concentration: f64,
    /// Uncertainty bootstrap samples
    pub bootstrap_samples: usize,
}

impl Default for FactorEngineConfig {
    fn default() -> Self {
        Self {
            winsorization_pct: 0.01,
            sector_neutralize: true,
            orthogonalize: true,
            min_obs: 30,
            turnover_penalty: 0.1,
            liquidity_weight: 0.2,
            max_concentration: 0.1,
            bootstrap_samples: 100,
        }
    }
}

/// Sector mapping for assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorMap {
    pub asset_to_sector: HashMap<String, String>,
    pub sector_to_assets: HashMap<String, Vec<String>>,
}

impl SectorMap {
    pub fn new(asset_to_sector: HashMap<String, String>) -> Self {
        let mut sector_to_assets: HashMap<String, Vec<String>> = HashMap::new();
        for (asset, sector) in &asset_to_sector {
            sector_to_assets.entry(sector.clone()).or_default().push(asset.clone());
        }
        Self { asset_to_sector, sector_to_assets }
    }

    pub fn get_sector(&self, asset: &str) -> Option<&String> {
        self.asset_to_sector.get(asset)
    }

    pub fn assets_in_sector(&self, sector: &str) -> Option<&Vec<String>> {
        self.sector_to_assets.get(sector)
    }
}

/// Raw factor values for an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorValues {
    pub asset: String,
    pub values: HashMap<String, f64>,
    pub sector: Option<String>,
    pub liquidity_score: f64,
    pub turnover: f64,
}

/// Cross-sectional ranking result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedAsset {
    pub asset: String,
    pub raw_score: f64,
    pub normalized_score: f64,
    pub rank: usize,
    pub percentile: f64,
    pub factor_contributions: HashMap<String, f64>,
    pub sector: Option<String>,
    pub uncertainty: f64,
    pub turnover_adjusted: f64,
    pub liquidity_adjusted: f64,
    pub exposure: f64,
    pub liquidity_score: f64,
    pub turnover: f64,
}

/// Intermediate scored asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredAsset {
    pub asset: String,
    pub raw_score: f64,
    pub sector: Option<String>,
    pub liquidity_score: f64,
    pub turnover: f64,
    pub factor_contributions: HashMap<String, f64>,
}

/// Cross-sectional factor engine
pub struct FactorEngine {
    config: FactorEngineConfig,
    sector_map: Option<SectorMap>,
}

impl FactorEngine {
    pub fn new(config: FactorEngineConfig) -> Self {
        Self {
            config,
            sector_map: None,
        }
    }

    pub fn with_sector_map(mut self, sector_map: SectorMap) -> Self {
        self.sector_map = Some(sector_map);
        self
    }

    pub fn with_config(mut self, config: FactorEngineConfig) -> Self {
        self.config = config;
        self
    }

    /// Winsorize factors cross-sectionally
    pub fn winsorize_factors(&self, factors: Vec<FactorValues>) -> Result<Vec<FactorValues>, FactorEngineError> {
        if factors.is_empty() || factors.len() < self.config.min_obs {
            return Err(FactorEngineError::InsufficientObservations);
        }

        let mut winsorized = factors.clone();
        for name in factors[0].values.keys() {
            // Collect values for this factor
            let mut values: Vec<f64> = factors.iter()
                .map(|f| f.values.get(name).copied().unwrap_or(0.0))
                .collect();

            // Sort values
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // Calculate bounds
            let n = values.len();
            let lower_idx = (n as f64 * self.config.winsorization_pct).floor() as usize;
            let upper_idx = (n as f64 * (1.0 - self.config.winsorization_pct)).ceil() as usize - 1;
            let lower_bound = values[lower_idx];
            let upper_bound = values[upper_idx];

            // Apply winsorization
            for f in &mut winsorized {
                if let Some(val) = f.values.get_mut(name) {
                    *val = val.clamp(lower_bound, upper_bound);
                }
            }
        }

        Ok(winsorized)
    }

    /// Cross-sectional normalization (z-score)
    pub fn normalize_cross_sectional(&self, factors: Vec<FactorValues>) -> Result<Vec<FactorValues>, FactorEngineError> {
        if factors.is_empty() || factors.len() < self.config.min_obs {
            return Err(FactorEngineError::InsufficientObservations);
        }

        let mut normalized = factors.clone();
        for name in factors[0].values.keys() {
            // Collect values for this factor
            let values: Vec<f64> = factors.iter()
                .map(|f| f.values.get(name).copied().unwrap_or(0.0))
                .collect();

            // Calculate mean and std
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance = values.iter()
                .map(|&x| (x - mean) * (x - mean))
                .sum::<f64>() / values.len() as f64;
            let std = variance.sqrt();

            // Apply normalization (avoid division by zero)
            if std > 1e-12 {
                for f in &mut normalized {
                    if let Some(val) = f.values.get_mut(name) {
                        *val = (*val - mean) / std;
                    }
                }
            } else {
                // If std is zero, set all values to 0
                for f in &mut normalized {
                    if let Some(val) = f.values.get_mut(name) {
                        *val = 0.0;
                    }
                }
            }
        }

        Ok(normalized)
    }

    /// Sector neutralization: subtract sector mean from each asset's factor
    pub fn neutralize_sectors(&self, mut factors: Vec<FactorValues>) -> Result<Vec<FactorValues>, FactorEngineError> {
        let sector_map = self.sector_map.as_ref()
            .ok_or(FactorEngineError::NoSectorMap)?;

        // Group by sector
        let mut sector_groups: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, f) in factors.iter().enumerate() {
            if let Some(sector) = sector_map.get_sector(&f.asset) {
                sector_groups.entry(sector.clone()).or_default().push(i);
            }
        }

        // For each factor, subtract sector mean
        let factor_names: Vec<String> = factors.first()
            .map(|f| f.values.keys().cloned().collect())
            .unwrap_or_default();

        for name in &factor_names {
            for (_sector, indices) in &sector_groups {
                let sector_mean: f64 = indices.iter()
                    .map(|&i| factors[i].values.get(name).copied().unwrap_or(0.0))
                    .sum::<f64>() / indices.len() as f64;

                for &i in indices {
                    if let Some(val) = factors[i].values.get_mut(name.as_str()) {
                        *val -= sector_mean;
                    }
                }
            }
        }

        Ok(factors)
    }

    /// Orthogonalize factors using Gram-Schmidt
    pub fn orthogonalize_factors(&self, factors: Vec<FactorValues>) -> Result<Vec<FactorValues>, FactorEngineError> {
        let factor_names: Vec<String> = factors.first()
            .map(|f| f.values.keys().cloned().collect())
            .unwrap_or_default();

        // Build correlation matrix
        let n = factor_names.len();
        if n <= 1 {
            return Ok(factors);
        }

        // Collect factor vectors
        let mut factor_vectors: Vec<Vec<f64>> = vec![vec![]; n];
        let mut asset_order: Vec<String> = Vec::new();

        for f in &factors {
            asset_order.push(f.asset.clone());
            for (i, name) in factor_names.iter().enumerate() {
                if let Some(&val) = f.values.get(name) {
                    factor_vectors[i].push(val);
                } else {
                    factor_vectors[i].push(0.0);
                }
            }
        }

        // Gram-Schmidt orthogonalization
        let mut ortho = factor_vectors.clone();
        for j in 0..n {
            for i in 0..j {
                let dot: f64 = ortho[i].iter().zip(ortho[j].iter()).map(|(a, b)| a * b).sum::<f64>();
                let norm_sq: f64 = ortho[i].iter().map(|x| x * x).sum();
                if norm_sq > 1e-12 {
                    let proj = dot / norm_sq;
                    for k in 0..ortho[j].len() {
                        ortho[j][k] -= proj * ortho[i][k];
                    }
                }
            }

            // Normalize
            let norm: f64 = ortho[j].iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 1e-12 {
                for k in 0..ortho[j].len() {
                    ortho[j][k] /= norm;
                }
            }
        }

        // Reconstruct FactorValues
        let mut result = Vec::new();
        for (idx, asset) in asset_order.iter().enumerate() {
            let mut values = HashMap::new();
            for (j, name) in factor_names.iter().enumerate() {
                values.insert(name.clone(), ortho[j][idx]);
            }
            result.push(FactorValues {
                asset: asset.clone(),
                values,
                sector: factors.iter().find(|f| f.asset == *asset).and_then(|f| f.sector.clone()),
                liquidity_score: factors.iter().find(|f| f.asset == *asset).map(|f| f.liquidity_score).unwrap_or(0.0),
                turnover: factors.iter().find(|f| f.asset == *asset).map(|f| f.turnover).unwrap_or(0.0),
            });
        }

        Ok(result)
    }

    /// Compute composite scores from factors
    pub fn compute_composite_scores(&self, factors: Vec<FactorValues>) -> Result<Vec<ScoredAsset>, FactorEngineError> {
        let factor_names: Vec<String> = factors.first()
            .map(|f| f.values.keys().cloned().collect())
            .unwrap_or_default();

        // Equal weight for now (could be configurable)
        let weight = 1.0 / factor_names.len().max(1) as f64;

        let mut result = Vec::new();
        for f in factors {
            let mut score = 0.0;
            let mut contributions = HashMap::new();
            for name in &factor_names {
                let val = f.values.get(name).copied().unwrap_or(0.0);
                score += val * weight;
                contributions.insert(name.clone(), val * weight);
            }

            result.push(ScoredAsset {
                asset: f.asset,
                raw_score: score,
                sector: f.sector.clone(),
                liquidity_score: f.liquidity_score,
                turnover: f.turnover,
                factor_contributions: contributions,
            });
        }

        Ok(result)
    }

    /// Rank assets by composite score
    pub fn rank_assets(&self, mut scored: Vec<ScoredAsset>) -> Result<Vec<RankedAsset>, FactorEngineError> {
        scored.sort_by(|a, b| b.raw_score.partial_cmp(&a.raw_score).unwrap());

        let scored_len = scored.len();
        let mut result = Vec::new();
        for (rank, s) in scored.into_iter().enumerate() {
            // 1-based rank: rank 1 = highest composite score
            let rank_1based = rank + 1;
            let percentile = rank_1based as f64 / scored_len as f64;
            result.push(RankedAsset {
                asset: s.asset,
                raw_score: s.raw_score,
                normalized_score: 0.0, // will be set in post_processing
                rank: rank_1based,
                percentile,
                factor_contributions: s.factor_contributions.clone(),
                sector: s.sector.clone(),
                uncertainty: 0.0, // will be set in post_processing
                turnover_adjusted: 0.0, // will be set in post_processing
                liquidity_adjusted: 0.0, // will be set in post_processing
                exposure: 0.0, // will be set in post_processing
                liquidity_score: s.liquidity_score,
                turnover: s.turnover,
            });
        }

        Ok(result)
    }

    /// Adjust for turnover and liquidity
    pub fn adjust_for_turnover_and_liquidity(&self, ranked: Vec<RankedAsset>) -> Result<Vec<RankedAsset>, FactorEngineError> {
        let mut ranked = ranked;
        for r in &mut ranked {
            // Apply turnover penalty
            r.turnover_adjusted = r.raw_score * (1.0 - self.config.turnover_penalty * r.turnover);
            // Apply liquidity bonus
            r.liquidity_adjusted = r.turnover_adjusted * (0.5 + 0.5 * r.liquidity_score);
        }
        Ok(ranked)
    }

    /// Apply exposure constraints (simple version: cap weights)
    pub fn apply_exposure_constraints(&self, ranked: Vec<RankedAsset>) -> Result<Vec<RankedAsset>, FactorEngineError> {
        let mut ranked = ranked;
        let n = ranked.len();
        if n == 0 {
            return Ok(ranked);
        }

        // Equal weight baseline
        let max_weight = self.config.max_concentration;

        for r in &mut ranked {
            // Simple approach: scale down scores that exceed max concentration
            let weight = r.raw_score.abs(); // proxy for weight
            if weight > max_weight {
                r.exposure = (max_weight / weight) * r.raw_score;
            } else {
                r.exposure = r.raw_score;
            }
        }

        Ok(ranked)
    }

    /// Estimate uncertainty using bootstrap
    pub fn estimate_uncertainty(&self, mut ranked: Vec<RankedAsset>) -> Result<Vec<RankedAsset>, FactorEngineError> {
        // For production, would use actual bootstrap resampling
        for r in &mut ranked {
            // Approximate uncertainty based on score magnitude and position
            let score_mag = r.normalized_score.abs();
            r.uncertainty = 0.05 + 0.1 * (1.0 - r.percentile) + 0.02 * score_mag;
        }
        Ok(ranked)
    }

    /// Process factors through the full pipeline
    pub fn process(&self, factors: Vec<FactorValues>) -> Result<Vec<RankedAsset>, FactorEngineError> {
        // Step 1: Validate input
        if factors.is_empty() {
            return Ok(vec![]);
        }

        // Step 2: Winsorize each factor cross-sectionally
        let winsorized = self.winsorize_factors(factors)?;

        // Step 3: Cross-sectional normalization (z-score)
        let normalized = self.normalize_cross_sectional(winsorized)?;

        // Step 4: Sector neutralization (if enabled)
        let neutralized = if self.config.sector_neutralize {
            self.neutralize_sectors(normalized)?
        } else {
            normalized
        };

        // Step 5: Factor orthogonalization (if enabled)
        let orthogonalized = if self.config.orthogonalize {
            self.orthogonalize_factors(neutralized)?
        } else {
            neutralized
        };

        // Step 6: Combine into composite scores with factor weights
        let scored = self.compute_composite_scores(orthogonalized)?;

        // Step 7: Rank assets by composite score
        let ranked = self.rank_assets(scored)?;

        // Step 8: Adjust for turnover and liquidity
        let adjusted = self.adjust_for_turnover_and_liquidity(ranked)?;

        // Step 9: Apply exposure constraints
        let constrained = self.apply_exposure_constraints(adjusted)?;

        // Step 10: Estimate uncertainty
        let mut final_ranked = self.estimate_uncertainty(constrained)?;

        // Step 11: Calculate final normalized scores (z-score of ranked scores)
        if final_ranked.len() >= 2 {
            let scores: Vec<f64> = final_ranked.iter().map(|r| r.raw_score).collect();
            let mean = scores.iter().sum::<f64>() / scores.len() as f64;
            let variance = scores.iter()
                .map(|&x| (x - mean) * (x - mean))
                .sum::<f64>() / scores.len() as f64;
            let std = variance.sqrt();

            if std > 1e-12 {
                for r in &mut final_ranked {
                    r.normalized_score = (r.raw_score - mean) / std;
                }
            } else {
                // If std is zero, set all normalized scores to 0
                for r in &mut final_ranked {
                    r.normalized_score = 0.0;
                }
            }
        }

        Ok(final_ranked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use std::collections::HashMap;
use rand::{thread_rng, Rng};

    fn create_test_data() -> Vec<FactorValues> {
        let mut data = Vec::new();
        let mut rng = thread_rng();
        for i in 0..50 {
            let mut values = HashMap::new();
            values.insert("factor1".into(), rng.r#gen::<f64>());
            values.insert("factor2".into(), rng.r#gen::<f64>());
            values.insert("factor3".into(), rng.r#gen::<f64>());

            data.push(FactorValues {
                asset: format!("ASSET{}", i),
                values,
                sector: if i % 3 == 0 { Some("tech".into()) } else if i % 3 == 1 { Some("finance".into()) } else { Some("energy".into()) },
                liquidity_score: rng.r#gen::<f64>(),
                turnover: rng.r#gen::<f64>(),
            });
        }
        data
    }

    #[test]
    fn test_winsorize_factors() {
        let config = FactorEngineConfig::default();
        let engine = FactorEngine::new(config);
        let data = create_test_data();
        let result = engine.winsorize_factors(data).unwrap();
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_normalize_cross_sectional() {
        let config = FactorEngineConfig::default();
        let engine = FactorEngine::new(config);
        let data = create_test_data();
        let result = engine.normalize_cross_sectional(data).unwrap();
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_neutralize_sectors() {
        let mut config = FactorEngineConfig::default();
        config.sector_neutralize = true;
        let sector_map = SectorMap::new({
            let mut m = HashMap::new();
            for i in 0..50 {
                m.insert(format!("ASSET{}", i), if i % 3 == 0 { "tech" } else if i % 3 == 1 { "finance" } else { "energy" }.into());
            }
            m
        });
        let engine = FactorEngine::new(config).with_sector_map(sector_map);
        let data = create_test_data();
        let result = engine.neutralize_sectors(data).unwrap();
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_orthogonalize_factors() {
        let mut config = FactorEngineConfig::default();
        config.orthogonalize = true;
        let engine = FactorEngine::new(config);
        let data = create_test_data();
        let result = engine.orthogonalize_factors(data).unwrap();
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_compute_composite_scores() {
        let config = FactorEngineConfig::default();
        let engine = FactorEngine::new(config);
        let data = create_test_data();
        let result = engine.compute_composite_scores(data).unwrap();
        assert_eq!(result.len(), 50);
    }

    #[test]
    fn test_rank_assets() {
        let config = FactorEngineConfig::default();
        let engine = FactorEngine::new(config);
        let mut data = Vec::new();
        for i in 0..5 {
            data.push(ScoredAsset {
                asset: format!("ASSET{}", i),
                raw_score: i as f64,
                sector: Some("test".into()),
                liquidity_score: 0.5,
                turnover: 0.1,
                factor_contributions: [("factor1".into(), 0.5), ("factor2".into(), 0.5)].iter().cloned().collect(),
            });
        }
        let result = engine.rank_assets(data).unwrap();
        assert_eq!(result.len(), 5);
        assert!(result[0].rank == 1);
        assert!(result[4].rank == 5);
    }

    #[test]
    fn test_full_pipeline() {
        let sector_map = SectorMap::new({
            let mut m = HashMap::new();
            for i in 0..50 {
                m.insert(format!("ASSET{}", i), if i % 3 == 0 { "tech" } else if i % 3 == 1 { "finance" } else { "energy" }.into());
            }
            m
        });
        let engine = FactorEngine::new(FactorEngineConfig::default()).with_sector_map(sector_map);
        let data = create_test_data();
        let result = engine.process(data).unwrap();
        assert_eq!(result.len(), 50);
        assert!(result[0].rank == 1);
        assert!(result[0].normalized_score >= result.last().unwrap().normalized_score);
    }
}