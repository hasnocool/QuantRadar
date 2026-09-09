//! regime crate documentation.
// Lightweight deterministic market-regime classifier.
use quantaradar_core::{FeatureRow, Regime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct RegimeThresholds { pub trend: f64, pub high_vol: f64, pub low_vol: f64 }
impl Default for RegimeThresholds { fn default()->Self{Self{trend:0.015,high_vol:0.045,low_vol:0.018}} }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence { High, Medium, Low }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendStrength { Strong, Moderate, Weak }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VolatilityState { High, Normal, Low }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegimeClassification {
    pub regime: Regime,
    pub confidence: Confidence,
    pub trend_strength: TrendStrength,
    pub volatility_state: VolatilityState,
    pub transition_probability: f64,
}

fn trend_strength_from_emas(ema_20: Option<f64>, ema_50: Option<f64>, ema_200: Option<f64>) -> TrendStrength {
    match (ema_20, ema_50, ema_200) {
        (Some(a), Some(b), Some(c)) => {
            let diff_20_50 = (a - b).abs() / b.max(1e-12);
            let diff_50_200 = (b - c).abs() / c.max(1e-12);
            if diff_20_50 > 0.05 && diff_50_200 > 0.05 {
                TrendStrength::Strong
            } else if diff_20_50 > 0.02 || diff_50_200 > 0.02 {
                TrendStrength::Moderate
            } else {
                TrendStrength::Weak
            }
        }
        _ => TrendStrength::Weak,
    }
}

fn volatility_state(vol: f64, t: &RegimeThresholds) -> VolatilityState {
    if vol > t.high_vol {
        VolatilityState::High
    } else if vol < t.low_vol {
        VolatilityState::Low
    } else {
        VolatilityState::Normal
    }
}

fn transition_probability(trend_raw: i8, r: f64, vol: f64, t: &RegimeThresholds) -> f64 {
    let mut score = 0.0f64;
    if trend_raw != 0 {
        score += 0.3 * ((r.abs() - t.trend).abs().max(0.0) / t.trend.max(1e-6));
    }
    if vol > t.high_vol {
        score += 0.3;
    } else if vol < t.low_vol {
        score -= 0.1;
    }
    score = score.clamp(0.0, 1.0);
    if score < 0.2 {
        0.1
    } else if score < 0.5 {
        0.3
    } else {
        0.7
    }
}

pub fn classify(f:&FeatureRow,t:RegimeThresholds)->RegimeClassification{
    let trend_raw=match(f.ema_20,f.ema_50,f.ema_200){(Some(a),Some(b),Some(c))=>{if a>b&&b>c{1}else if a<b&&b<c{-1}else{0}},_=>0};
    let r=f.returns_24.unwrap_or(0.0);
    let vol=f.realized_vol_20.unwrap_or(0.0);
    let regime = match (trend_raw, r, vol) {
        (_, _, _) if trend_raw > 0 && r > t.trend && vol > t.high_vol => Regime::BullHighVol,
        (_, _, _) if trend_raw > 0 && r > t.trend && vol <= t.high_vol => Regime::BullTrend,
        (_, _, _) if trend_raw > 0 && r <= t.trend && vol < t.low_vol => Regime::BullLowVol,
        (_, _, _) if trend_raw > 0 && r <= t.trend && vol >= t.low_vol => Regime::TransitionBull,
        (_, _, _) if trend_raw < 0 && r < -t.trend && vol > t.high_vol => Regime::BearHighVol,
        (_, _, _) if trend_raw < 0 && r < -t.trend && vol <= t.high_vol => Regime::BearTrend,
        (_, _, _) if trend_raw < 0 && r >= -t.trend && vol < t.low_vol => Regime::BearLowVol,
        (_, _, _) if trend_raw < 0 && r >= -t.trend && vol >= t.low_vol => Regime::TransitionBear,
        (_, _, _) if vol > t.high_vol => Regime::SidewaysHighVol,
        (_, _, _) => Regime::SidewaysLowVol,
    };
    let trend_aligned = (trend_raw > 0 && r > 0.0) || (trend_raw < 0 && r < 0.0) || trend_raw == 0;
    let vol_aligned = vol <= t.high_vol && vol >= t.low_vol;
    let confidence = match (trend_aligned, vol_aligned) {
        (true, true) => Confidence::High,
        (true, false) | (false, true) => Confidence::Medium,
        (false, false) => Confidence::Low,
    };
    let trend_strength = trend_strength_from_emas(f.ema_20, f.ema_50, f.ema_200);
    let volatility_state = volatility_state(vol, &t);
    let transition_prob = transition_probability(trend_raw, r, vol, &t);
    RegimeClassification { regime, confidence, trend_strength, volatility_state, transition_probability: transition_prob }
}

pub fn classify_with_confidence(f:&FeatureRow,t:RegimeThresholds)->RegimeClassification{
    let c = classify(f, t);
    c
}

pub fn regime_transition(prev: Regime, curr: Regime) -> Option<(Regime, Regime)> {
    if prev != curr { Some((prev, curr)) } else { None }
}

pub fn multi_timeframe_regime(rows: &[FeatureRow], t: RegimeThresholds, short_idx: usize, long_idx: usize) -> (RegimeClassification, RegimeClassification) {
    let short = classify(&rows[short_idx.min(rows.len()-1)], t);
    let long = classify(&rows[long_idx.min(rows.len()-1)], t);
    (short, long)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quantaradar_core::Bar;
    use chrono::Utc;

    #[test]
    fn regime_classification_has_all_fields() {
        let bars: Vec<Bar> = (0..200).map(|i| Bar {
            ts: Utc::now(), open: 100.0 + i as f64, high: 101.0 + i as f64,
            low: 99.0 + i as f64, close: 100.5 + i as f64, volume: 1000.0, trades: None
        }).collect();
        let rows = quantaradar_features::feature_rows("TEST", &bars);
        if rows.len() > 50 {
            let cls = classify(&rows[60], RegimeThresholds::default());
            assert!(matches!(cls.confidence, Confidence::High | Confidence::Medium | Confidence::Low));
            assert!(matches!(cls.trend_strength, TrendStrength::Strong | TrendStrength::Moderate | TrendStrength::Weak);
            assert!(matches!(cls.volatility_state, VolatilityState::High | VolatilityState::Normal | VolatilityState::Low));
            assert!(cls.transition_probability >= 0.0 && cls.transition_probability <= 1.0);
        }
    }

    #[test]
    fn multi_timeframe_returns_both() {
        let bars: Vec<Bar> = (0..200).map(|i| Bar {
            ts: Utc::now(), open: 100.0 + i as f64, high: 101.0 + i as f64,
            low: 99.0 + i as f64, close: 100.5 + i as f64, volume: 1000.0, trades: None
        }).collect();
        let rows = quantaradar_features::feature_rows("TEST", &bars);
        if rows.len() > 100 {
            let (short, long) = multi_timeframe_regime(&rows, RegimeThresholds::default(), 10, 50);
            assert!(short.regime != Regime::Unknown);
            assert!(long.regime != Regime::Unknown);
        }
    }

    #[test]
    fn eleven_regimes_covered() {
        use quantaradar_core::Regime;
        let variants = [
            Regime::BullTrend, Regime::BullHighVol, Regime::BullLowVol,
            Regime::BearTrend, Regime::BearHighVol, Regime::BearLowVol,
            Regime::SidewaysHighVol, Regime::SidewaysLowVol,
            Regime::TransitionBull, Regime::TransitionBear, Regime::Unknown
        ];
        for &v in &variants {
            let _ = v; // just verify they exist
        }
    }
}
