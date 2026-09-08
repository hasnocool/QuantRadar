// QuantRadar signal ensemble and meta-model for signal aggregation.
use quantaradar_core::{Direction, Regime, SignalFamily};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub symbol: String,
    pub family: SignalFamily,
    pub direction: Direction,
    pub score: f64,
    pub confidence: f64,
    pub regime_compatibility: f64,
    pub liquidity_score: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleSignal {
    pub symbol: String,
    pub direction: Direction,
    pub composite_score: f64,
    pub component_scores: ComponentScores,
    pub rationale: Vec<String>,
    pub expected_return: Option<f64>,
    pub expected_risk: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentScores {
    pub trend: f64,
    pub momentum: f64,
    pub breakout: f64,
    pub mean_reversion: f64,
    pub microstructure: f64,
    pub event: f64,
    pub relative_strength: f64,
}

impl EnsembleSignal {
    pub fn new(symbol: String, signals: Vec<Signal>) -> Self {
        let n = signals.len() as f64;
        if n == 0.0 {
            return Self {
                symbol,
                direction: Direction::Flat,
                composite_score: 0.0,
                component_scores: ComponentScores {
                    trend: 0.0,
                    momentum: 0.0,
                    breakout: 0.0,
                    mean_reversion: 0.0,
                    microstructure: 0.0,
                    event: 0.0,
                    relative_strength: 0.0,
                },
                rationale: vec![],
                expected_return: None,
                expected_risk: None,
            };
        }

        let mut weights = vec![];
        let mut scores = vec![];
        let mut rationale = vec![];

        for s in &signals {
            let weight = s.confidence * s.liquidity_score * s.regime_compatibility;
            weights.push(weight);
            scores.push(s.score * weight);
            rationale.extend(s.evidence.clone());
        }

        let total_weight: f64 = weights.iter().sum();
        let composite_score = if total_weight > 0.0 {
            scores.iter().sum::<f64>() / total_weight
        } else {
            0.0
        };

        let direction = if composite_score > 0.55 {
            Direction::Long
        } else if composite_score < 0.45 {
            Direction::Short
        } else {
            Direction::Flat
        };

        let component_scores = aggregate_components(&signals);

        Self {
            symbol,
            direction,
            composite_score,
            component_scores,
            rationale: rationale.into_iter().take(10).collect(),
            expected_return: None,
            expected_risk: None,
        }
    }
}

fn aggregate_components(signals: &[Signal]) -> ComponentScores {
    let mut components = ComponentScores {
        trend: 0.0,
        momentum: 0.0,
        breakout: 0.0,
        mean_reversion: 0.0,
        microstructure: 0.0,
        event: 0.0,
        relative_strength: 0.0,
    };

    for s in signals {
        let score = s.score;
        match s.family {
            SignalFamily::Trend => components.trend = components.trend.max(score),
            SignalFamily::Momentum | SignalFamily::MomentumDivergence => components.momentum = components.momentum.max(score),
            SignalFamily::Breakout => components.breakout = components.breakout.max(score),
            SignalFamily::MeanReversion => components.mean_reversion = components.mean_reversion.max(score),
            SignalFamily::VolatilityExpansion | SignalFamily::Microstructure => components.microstructure = components.microstructure.max(score),
            SignalFamily::VolumeSurge | SignalFamily::Event => components.event = components.event.max(score),
            SignalFamily::SupportResistanceBounce => components.relative_strength = components.relative_strength.max(score),
        }
    }

    components
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalEnsembleConfig {
    pub min_signals: usize,
    pub min_composite_score: f64,
    pub max_correlation_threshold: f64,
    pub weight_confidence: bool,
    pub weight_liquidity: bool,
    pub weight_regime: bool,
}

impl Default for SignalEnsembleConfig {
    fn default() -> Self {
        Self {
            min_signals: 3,
            min_composite_score: 0.6,
            max_correlation_threshold: 0.7,
            weight_confidence: true,
            weight_liquidity: true,
            weight_regime: true,
        }
    }
}

pub fn should_trade(ensemble: &EnsembleSignal, config: &SignalEnsembleConfig) -> bool {
    if ensemble.direction == Direction::Flat {
        return false;
    }
    if ensemble.composite_score < config.min_composite_score {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensemble_signal_creation() {
        let signals = vec![
            Signal {
                symbol: "BTC/USD".into(),
                family: SignalFamily::Trend,
                direction: Direction::Long,
                score: 0.8,
                confidence: 0.9,
                regime_compatibility: 0.85,
                liquidity_score: 0.95,
                evidence: vec!["EMA alignment".into()],
            },
            Signal {
                symbol: "BTC/USD".into(),
                family: SignalFamily::MomentumDivergence,
                direction: Direction::Long,
                score: 0.75,
                confidence: 0.8,
                regime_compatibility: 0.8,
                liquidity_score: 0.9,
                evidence: vec!["Positive momentum".into()],
            },
        ];

        let ensemble = EnsembleSignal::new("BTC/USD".into(), signals);
        assert_eq!(ensemble.symbol, "BTC/USD");
        assert_eq!(ensemble.direction, Direction::Long);
        assert!(ensemble.composite_score > 0.7);
        assert!(!ensemble.rationale.is_empty());
    }

    #[test]
    fn test_ensemble_threshold() {
        let signals = vec![
            Signal {
                symbol: "BTC/USD".into(),
                family: SignalFamily::Trend,
                direction: Direction::Long,
                score: 0.4,
                confidence: 0.5,
                regime_compatibility: 0.5,
                liquidity_score: 0.5,
                evidence: vec![],
            },
        ];

        let ensemble = EnsembleSignal::new("BTC/USD".into(), signals);
        let config = SignalEnsembleConfig::default();
        assert!(!should_trade(&ensemble, &config));
    }

    #[test]
    fn test_component_aggregation() {
        let signals = vec![
            Signal {
                symbol: "ETH/USD".into(),
                family: SignalFamily::Breakout,
                direction: Direction::Long,
                score: 0.85,
                confidence: 1.0,
                regime_compatibility: 1.0,
                liquidity_score: 1.0,
                evidence: vec![],
            },
            Signal {
                symbol: "ETH/USD".into(),
                family: SignalFamily::Breakout,
                direction: Direction::Long,
                score: 0.7,
                confidence: 1.0,
                regime_compatibility: 1.0,
                liquidity_score: 1.0,
                evidence: vec![],
            },
        ];

        let ensemble = EnsembleSignal::new("ETH/USD".into(), signals);
        assert_eq!(ensemble.component_scores.breakout, 0.85);
    }
}
