// Lightweight deterministic market-regime classifier.
use quantaradar_core::{FeatureRow, Regime};
#[derive(Debug, Clone, Copy)] pub struct RegimeThresholds { pub trend: f64, pub high_vol: f64, pub low_vol: f64 }
impl Default for RegimeThresholds { fn default()->Self{Self{trend:0.015,high_vol:0.045,low_vol:0.018}} }
pub fn classify(f:&FeatureRow,t:RegimeThresholds)->Regime{let trend=match(f.ema_20,f.ema_50,f.ema_200){(Some(a),Some(b),Some(c))=>{if a>b&&b>c{1}else if a<b&&b<c{-1}else{0}},_=>0};let r=f.returns_24.unwrap_or(0.0);let vol=f.realized_vol_20.unwrap_or(0.0);if trend>0&&r>t.trend{if vol>t.high_vol{Regime::BullHighVol}else{Regime::BullTrend}}else if trend>0{if vol<t.low_vol{Regime::BullLowVol}else{Regime::TransitionBull}}else if trend<0&&r< -t.trend{if vol>t.high_vol{Regime::BearHighVol}else{Regime::BearTrend}}else if trend<0{if vol<t.low_vol{Regime::BearLowVol}else{Regime::TransitionBear}}else if vol>t.high_vol{Regime::SidewaysHighVol}else{Regime::SidewaysLowVol}}
