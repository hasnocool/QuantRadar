// QuantRadar core domain models and deterministic utilities.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bar {
    pub ts: DateTime<Utc>, pub open: f64, pub high: f64, pub low: f64, pub close: f64, pub volume: f64,
    #[serde(default)] pub trades: Option<f64>,
}
impl Bar { pub fn range(&self) -> f64 { self.high - self.low } pub fn typical_price(&self) -> f64 { (self.high + self.low + self.close) / 3.0 } }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketId { pub exchange: String, pub symbol: String, pub base: String, pub quote: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Regime { BullTrend, BullHighVol, BullLowVol, BearTrend, BearHighVol, BearLowVol, SidewaysHighVol, SidewaysLowVol, TransitionBull, TransitionBear, Unknown }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Direction { Long, Short, Flat }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum OrderSide { Buy, Sell }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceKind { Rest, #[serde(rename="websocket")] WebSocket, Replay }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityFlag { Valid, LateArrival, OutOfSequence, NegativePrice, NegativeVolume, InvalidSpread, Stale }
impl std::fmt::Display for Direction { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { Direction::Long => write!(f, "long"), Direction::Short => write!(f, "short"), Direction::Flat => write!(f, "flat"), } } }
impl std::fmt::Display for OrderSide { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self { OrderSide::Buy => write!(f, "buy"), OrderSide::Sell => write!(f, "sell"), } } }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRow { pub ts: DateTime<Utc>, pub symbol: String, pub close: f64, pub returns_1: Option<f64>, pub returns_4: Option<f64>, pub returns_24: Option<f64>, pub returns_72: Option<f64>, pub ema_20: Option<f64>, pub ema_50: Option<f64>, pub ema_200: Option<f64>, pub rsi_14: Option<f64>, pub atr_14: Option<f64>, pub atr_pct: Option<f64>, pub realized_vol_20: Option<f64>, pub bb_width_20: Option<f64>, pub volume_z_20: Option<f64>, pub distance_ema20_atr: Option<f64>, pub breakout_20: bool, pub new_high_20: bool, pub new_low_20: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal { pub id: Uuid, pub ts: DateTime<Utc>, pub symbol: String, pub family: String, pub direction: Direction, pub score: f64, pub regime: Regime, pub rationale: Vec<String>, pub features: BTreeMap<String, f64> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenResult { pub generated_at: DateTime<Utc>, pub regime: Regime, pub signals: Vec<Signal> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot { pub ts: i64, pub bid: f64, pub ask: f64, pub bid_depth: Vec<(f64,f64)>, pub ask_depth: Vec<(f64,f64)> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeTick { pub ts: i64, pub price: f64, pub quantity: f64, pub side: OrderSide }
pub fn safe_return(now: f64, then: f64) -> Option<f64> { if then.is_finite() && now.is_finite() && then.abs() > f64::EPSILON { Some(now / then - 1.0) } else { None } }
#[cfg(test)]
mod tests { use super::*; #[test] fn direction_serde() { assert_eq!(Direction::Long.to_string(), "long"); assert_eq!(Direction::Short.to_string(), "short"); assert_eq!(Direction::Flat.to_string(), "flat"); assert_eq!(OrderSide::Buy.to_string(), "buy"); assert_eq!(OrderSide::Sell.to_string(), "sell"); } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub timestamp: u64,
    pub exchange: String,
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub open: f64, pub high: f64, pub low: f64, pub close: f64, pub volume: f64,
    pub trade_count: u64,
    pub bid: f64, pub ask: f64,
    pub bid_depth: Vec<(f64,f64)>,
    pub ask_depth: Vec<(f64,f64)>,
    pub source: SourceKind,
    pub ingested_at: u64,
    pub quality_flags: Vec<QualityFlag>,
}
impl Observation {
    pub fn new(timestamp: u64, exchange: String, symbol: String, base: String, quote: String,
               open: f64, high: f64, low: f64, close: f64, volume: f64,
               trade_count: u64, bid: f64, ask: f64,
               bid_depth: Vec<(f64,f64)>, ask_depth: Vec<(f64,f64)>,
               source: SourceKind, ingested_at: u64, quality_flags: Vec<QualityFlag>) -> Self {
        Self { timestamp, exchange, symbol, base, quote, open, high, low, close, volume,
               trade_count, bid, ask, bid_depth, ask_depth, source, ingested_at, quality_flags }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetManifest {
    pub dataset_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub symbols: Vec<String>,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub git_commit: String,
    pub config_hash: String,
}
impl DatasetManifest {
    pub fn compute_checksum(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.dataset_id.hash(&mut hasher);
        let mut symbols = self.symbols.clone();
        symbols.sort();
        for s in &symbols { s.hash(&mut hasher); }
        format!("{:x}", hasher.finish())
    }
    pub fn to_json(&self) -> anyhow::Result<String> { Ok(serde_json::to_string_pretty(self)?) }
    pub fn from_json(s: &str) -> anyhow::Result<Self> { Ok(serde_json::from_str(s)?) }
    pub fn write_to(&self, path: &std::path::Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }
    pub fn read_from(path: &std::path::Path) -> anyhow::Result<Self> {
        let s = std::fs::read_to_string(path)?;
        Self::from_json(&s)
    }
}

pub const STORAGE_LAYOUT: &[&str] = &["raw/trades", "raw/books", "raw/ohlcv", "normalized", "manifests", "rejected"];
pub fn ensure_storage_layout(base: &std::path::Path) -> anyhow::Result<()> {
    for dir in STORAGE_LAYOUT {
        std::fs::create_dir_all(base.join(dir))?;
    }
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct QualityCheckResult {
    pub flags: Vec<QualityFlag>,
    pub is_valid: bool,
}

impl QualityCheckResult {
    pub fn valid() -> Self { Self { flags: vec![QualityFlag::Valid], is_valid: true } }
    pub fn invalid(flags: Vec<QualityFlag>) -> Self { Self { flags, is_valid: false } }
}

pub fn validate_observation(obs: &Observation, last_timestamp: Option<u64>) -> QualityCheckResult {
    let mut flags = Vec::new();

    if let Some(last) = last_timestamp {
        if obs.timestamp <= last {
            flags.push(QualityFlag::OutOfSequence);
        }
    }

    if obs.open <= 0.0 || obs.high <= 0.0 || obs.low <= 0.0 || obs.close <= 0.0 {
        flags.push(QualityFlag::NegativePrice);
    }
    if obs.volume < 0.0 {
        flags.push(QualityFlag::NegativeVolume);
    }

    if obs.ask < obs.bid {
        flags.push(QualityFlag::InvalidSpread);
    }

    if obs.bid <= 0.0 || obs.ask <= 0.0 {
        flags.push(QualityFlag::NegativePrice);
    }

    for (p, q) in &obs.bid_depth { if *p <= 0.0 || *q < 0.0 { flags.push(QualityFlag::NegativePrice); } }
    for (p, q) in &obs.ask_depth { if *p <= 0.0 || *q < 0.0 { flags.push(QualityFlag::NegativePrice); } }

    if flags.is_empty() { QualityCheckResult::valid() } else { QualityCheckResult::invalid(flags) }
}

pub fn validate_bar(bar: &Bar, last_timestamp: Option<u64>) -> QualityCheckResult {
    let mut flags = Vec::new();

    if let Some(last) = last_timestamp {
        if bar.ts.timestamp() as u64 <= last { flags.push(QualityFlag::OutOfSequence); }
    }
    if bar.open <= 0.0 || bar.high <= 0.0 || bar.low <= 0.0 || bar.close <= 0.0 { flags.push(QualityFlag::NegativePrice); }
    if bar.volume < 0.0 { flags.push(QualityFlag::NegativeVolume); }
    if bar.high < bar.low { flags.push(QualityFlag::InvalidSpread); }

    if flags.is_empty() { QualityCheckResult::valid() } else { QualityCheckResult::invalid(flags) }
}

pub fn is_valid_price(v: f64) -> bool { v.is_finite() && v > 0.0 }
pub fn is_valid_volume(v: f64) -> bool { v.is_finite() && v >= 0.0 }
