//! regime crate documentation.
// Lightweight deterministic market-regime classifier.
use quantaradar_core::{FeatureRow, Regime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct RegimeThresholds { pub trend: f64, pub high_vol: f64, pub low_vol: f64 }
impl Default for RegimeThresholds { fn default()->Self{Self{trend:0.015,high_vol:0.045,low_vol:0.018}} }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence { High, Medium, Low }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegimeState {
    pub regime: Regime,
    pub confidence: Confidence,
    pub trend_aligned: bool,
    pub vol_aligned: bool,
}

pub fn classify(f:&FeatureRow,t:RegimeThresholds)->Regime{
    let trend=match(f.ema_20,f.ema_50,f.ema_200){(Some(a),Some(b),Some(c))=>{if a>b&&b>c{1}else if a<b&&b<c{-1}else{0}},_=>0};
    let r=f.returns_24.unwrap_or(0.0);
    let vol=f.realized_vol_20.unwrap_or(0.0);
    if trend>0&&r>t.trend{if vol>t.high_vol{Regime::BullHighVol}else{Regime::BullTrend}}
    else if trend>0{if vol<t.low_vol{Regime::BullLowVol}else{Regime::TransitionBull}}
    else if trend<0&&r< -t.trend{if vol>t.high_vol{Regime::BearHighVol}else{Regime::BearTrend}}
    else if trend<0{if vol<t.low_vol{Regime::BearLowVol}else{Regime::TransitionBear}}
    else if vol>t.high_vol{Regime::SidewaysHighVol}else{Regime::SidewaysLowVol}
}

pub fn classify_with_confidence(f:&FeatureRow,t:RegimeThresholds)->RegimeState{
    let trend_raw=match(f.ema_20,f.ema_50,f.ema_200){(Some(a),Some(b),Some(c))=>{if a>b&&b>c{1}else if a<b&&b<c{-1}else{0}},_=>0};
    let r=f.returns_24.unwrap_or(0.0);
    let vol=f.realized_vol_20.unwrap_or(0.0);
    let trend_aligned = (trend_raw > 0 && r > 0.0) || (trend_raw < 0 && r < 0.0) || trend_raw == 0;
    let vol_aligned = vol <= t.high_vol && vol >= t.low_vol;

    let regime = classify(f, t);
    let confidence = match (trend_aligned, vol_aligned) {
        (true, true) => Confidence::High,
        (true, false) | (false, true) => Confidence::Medium,
        (false, false) => Confidence::Low,
    };

    RegimeState { regime, confidence, trend_aligned, vol_aligned }
}

pub fn regime_transition(prev: Regime, curr: Regime) -> Option<(Regime, Regime)> {
    if prev != curr { Some((prev, curr)) } else { None }
}

pub fn multi_timeframe_regime(rows: &[FeatureRow], t: RegimeThresholds, short_idx: usize, long_idx: usize) -> (RegimeState, RegimeState) {
    let short = classify_with_confidence(&rows[short_idx.min(rows.len()-1)], t);
    let long = classify_with_confidence(&rows[long_idx.min(rows.len()-1)], t);
    (short, long)
}
