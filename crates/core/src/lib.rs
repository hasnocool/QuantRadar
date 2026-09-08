// QuantRadar core domain models and deterministic utilities.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bar {
    pub ts: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    #[serde(default)]
    pub trades: Option<f64>,
}
impl Bar { pub fn range(&self) -> f64 { self.high - self.low } pub fn typical_price(&self) -> f64 { (self.high + self.low + self.close) / 3.0 } }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketId { pub exchange: String, pub symbol: String, pub base: String, pub quote: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Regime { BullTrend, BullHighVol, BullLowVol, BearTrend, BearHighVol, BearLowVol, SidewaysHighVol, SidewaysLowVol, TransitionBull, TransitionBear, Unknown }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRow { pub ts: DateTime<Utc>, pub symbol: String, pub close: f64, pub returns_1: Option<f64>, pub returns_4: Option<f64>, pub returns_24: Option<f64>, pub returns_72: Option<f64>, pub ema_20: Option<f64>, pub ema_50: Option<f64>, pub ema_200: Option<f64>, pub rsi_14: Option<f64>, pub atr_14: Option<f64>, pub atr_pct: Option<f64>, pub realized_vol_20: Option<f64>, pub bb_width_20: Option<f64>, pub volume_z_20: Option<f64>, pub distance_ema20_atr: Option<f64>, pub breakout_20: bool, pub new_high_20: bool, pub new_low_20: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal { pub id: Uuid, pub ts: DateTime<Utc>, pub symbol: String, pub family: String, pub direction: String, pub score: f64, pub regime: Regime, pub rationale: Vec<String>, pub features: BTreeMap<String, f64> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenResult { pub generated_at: DateTime<Utc>, pub regime: Regime, pub signals: Vec<Signal> }
pub fn safe_return(now: f64, then: f64) -> Option<f64> { if then.is_finite() && now.is_finite() && then.abs() > f64::EPSILON { Some(now / then - 1.0) } else { None } }
