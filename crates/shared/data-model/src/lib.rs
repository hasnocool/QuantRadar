//! data-model crate documentation.
// QuantRadar immutable market data model with full observation fields.
// Fields: timestamp, exchange, symbol, base, quote, OHLCV, trade_count, bid, ask, bid_depth, ask_depth, source, ingested_at, quality_flags
use quantaradar_core::{Observation, QualityFlag, Regime, SourceKind};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Immutable market observation with all required fields for auditability and replay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketObservation {
    /// Unix timestamp in milliseconds
    pub timestamp: u64,
    /// Order-book sequence number for delta ordering
    pub sequence: u64,
    /// Exchange update id
    pub update_id: u64,
    /// Optional checksum for integrity verification
    pub checksum: Option<String>,
    /// Exchange identifier (e.g., "kraken", "binance")
    pub exchange: String,
    /// Trading symbol (e.g., "BTC/USD")
    pub symbol: String,
    /// Base asset (e.g., "BTC")
    pub base: String,
    /// Quote asset (e.g., "USD")
    pub quote: String,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Volume in base asset
    pub volume: f64,
    /// Number of trades in this period
    pub trade_count: u64,
    /// Best bid price
    pub bid: f64,
    /// Best ask price
    pub ask: f64,
    /// Bid depth levels (price, quantity)
    pub bid_depth: Vec<(f64, f64)>,
    /// Ask depth levels (price, quantity)
    pub ask_depth: Vec<(f64, f64)>,
    /// Data source kind
    pub source: SourceKind,
    /// Ingestion timestamp (Unix ms)
    pub ingested_at: u64,
    /// Quality validation flags
    pub quality_flags: Vec<QualityFlag>,
    /// Optional regime at observation time
    pub regime: Option<Regime>,
    /// Additional metadata
    pub metadata: BTreeMap<String, String>,
}

impl MarketObservation {
    /// Create a new market observation
    pub fn new(
        timestamp: u64,
        exchange: String,
        symbol: String,
        base: String,
        quote: String,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        trade_count: u64,
        bid: f64,
        ask: f64,
        bid_depth: Vec<(f64, f64)>,
        ask_depth: Vec<(f64, f64)>,
        source: SourceKind,
        ingested_at: u64,
        quality_flags: Vec<QualityFlag>,
    ) -> Self {
        Self {
            timestamp,
            sequence: 0,
            update_id: 0,
            checksum: None,
            exchange,
            symbol,
            base,
            quote,
            open,
            high,
            low,
            close,
            volume,
            trade_count,
            bid,
            ask,
            bid_depth,
            ask_depth,
            source,
            ingested_at,
            quality_flags,
            regime: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Create from core Observation
    pub fn from_core(obs: Observation) -> Self {
        Self {
            timestamp: obs.timestamp,
            sequence: 0,
            update_id: 0,
            checksum: None,
            exchange: obs.exchange,
            symbol: obs.symbol,
            base: obs.base,
            quote: obs.quote,
            open: obs.ohlcv.open,
            high: obs.ohlcv.high,
            low: obs.ohlcv.low,
            close: obs.ohlcv.close,
            volume: obs.ohlcv.volume,
            trade_count: obs.trade_count,
            bid: obs.bid,
            ask: obs.ask,
            bid_depth: obs.bid_depth,
            ask_depth: obs.ask_depth,
            source: obs.source,
            ingested_at: obs.ingested_at,
            quality_flags: obs.quality_flags,
            regime: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Convert to core Observation
    pub fn to_core(&self) -> Observation {
        Observation {
            timestamp: self.timestamp,
            exchange: self.exchange.clone(),
            symbol: self.symbol.clone(),
            base: self.base.clone(),
            quote: self.quote.clone(),
            ohlcv: OHLCV { open: self.open, high: self.high, low: self.low, close: self.close, volume: self.volume },
            trade_count: self.trade_count,
            bid: self.bid,
            ask: self.ask,
            bid_depth: self.bid_depth.clone(),
            ask_depth: self.ask_depth.clone(),
            source: self.source,
            ingested_at: self.ingested_at,
            quality_flags: self.quality_flags.clone(),
        }
    }

    /// Check if observation is valid (no error flags)
    pub fn is_valid(&self) -> bool {
        self.quality_flags.iter().all(|f| *f == QualityFlag::Valid)
    }

    /// Get mid price
    pub fn mid_price(&self) -> f64 {
        (self.bid + self.ask) / 2.0
    }

    /// Get spread
    pub fn spread(&self) -> f64 {
        self.ask - self.bid
    }

    /// Get spread in basis points
    pub fn spread_bps(&self) -> f64 {
        let mid = self.mid_price();
        if mid > 0.0 {
            self.spread() / mid * 10_000.0
        } else {
            0.0
        }
    }
}

/// Trade observation for tick-level data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeObservation {
    pub timestamp: u64,
    pub exchange: String,
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub side: quantaradar_core::OrderSide,
    pub trade_id: String,
    pub ingested_at: u64,
    pub quality_flags: Vec<QualityFlag>,
}

/// Order book snapshot with full L2 depth
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderBookObservation {
    pub timestamp: u64,
    pub exchange: String,
    pub symbol: String,
    pub bids: Vec<(f64, f64)>, // (price, quantity)
    pub asks: Vec<(f64, f64)>,
    pub sequence: u64,
    pub ingested_at: u64,
    pub quality_flags: Vec<QualityFlag>,
}

/// Market data batch for efficient storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataBatch {
    pub observations: Vec<MarketObservation>,
    pub trades: Vec<TradeObservation>,
    pub order_books: Vec<OrderBookObservation>,
    pub batch_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl MarketDataBatch {
    pub fn new() -> Self {
        Self {
            observations: Vec::new(),
            trades: Vec::new(),
            order_books: Vec::new(),
            batch_id: Uuid::new_v4(),
            created_at: Utc::now(),
        }
    }

    pub fn with_capacity(obs: usize, trades: usize, books: usize) -> Self {
        Self {
            observations: Vec::with_capacity(obs),
            trades: Vec::with_capacity(trades),
            order_books: Vec::with_capacity(books),
            batch_id: Uuid::new_v4(),
            created_at: Utc::now(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.observations.is_empty() && self.trades.is_empty() && self.order_books.is_empty()
    }

    pub fn len(&self) -> usize {
        self.observations.len() + self.trades.len() + self.order_books.len()
    }
}

impl Default for MarketDataBatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Dataset manifest for data lineage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetManifest {
    pub dataset_id: String,
    pub dataset_type: DatasetType,
    pub start_time: u64,
    pub end_time: u64,
    pub symbols: Vec<String>,
    pub exchanges: Vec<String>,
    pub row_count: usize,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub git_commit: String,
    pub config_hash: String,
    pub parent_dataset: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetType {
    RawTrades,
    RawBooks,
    RawOhlcv,
    Normalized,
    Features,
    Labels,
}

impl DatasetManifest {
    pub fn compute_checksum(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.dataset_id.hash(&mut hasher);
        self.dataset_type.hash(&mut hasher);
        self.start_time.hash(&mut hasher);
        self.end_time.hash(&mut hasher);
        let mut symbols = self.symbols.clone();
        symbols.sort();
        for s in &symbols { s.hash(&mut hasher); }
        format!("{:x}", hasher.finish())
    }
}

/// Storage layout constants
pub const STORAGE_LAYOUT: &[&str] = &[
    "raw/trades",
    "raw/books",
    "raw/ohlcv",
    "normalized",
    "features",
    "datasets",
    "manifests",
    "rejected",
];

pub fn ensure_storage_layout(base: &std::path::Path) -> anyhow::Result<()> {
    for dir in STORAGE_LAYOUT {
        std::fs::create_dir_all(base.join(dir))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_observation_creation() {
        let obs = MarketObservation::new(
            1_700_000_000_000,
            "kraken".into(),
            "BTC/USD".into(),
            "BTC".into(),
            "USD".into(),
            50_000.0, 51_000.0, 49_000.0, 50_500.0,
            100.0, 50,
            50_490.0, 50_510.0,
            vec![(50_490.0, 1.0), (50_480.0, 2.0)],
            vec![(50_510.0, 1.5), (50_520.0, 2.5)],
            SourceKind::Rest,
            1_700_000_000_100,
            vec![QualityFlag::Valid],
        );

        assert_eq!(obs.symbol, "BTC/USD");
        assert_eq!(obs.mid_price(), 50_500.0);
        assert_eq!(obs.spread(), 20.0);
        assert!(obs.is_valid());
    }

    #[test]
    fn test_market_data_batch() {
        let mut batch = MarketDataBatch::new();
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);

        batch.observations.push(MarketObservation::new(
            1_700_000_000_000, "kraken".into(), "BTC/USD".into(),
            "BTC".into(), "USD".into(),
            50_000.0, 51_000.0, 49_000.0, 50_500.0,
            100.0, 50, 50_490.0, 50_510.0,
            vec![], vec![],
            SourceKind::Rest, 1_700_000_000_100, vec![QualityFlag::Valid],
        ));

        assert!(!batch.is_empty());
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_dataset_manifest_checksum() {
        let manifest = DatasetManifest {
            dataset_id: "test_dataset".into(),
            dataset_type: DatasetType::RawTrades,
            start_time: 1_700_000_000_000,
            end_time: 1_700_000_000_000 + 86_400_000,
            symbols: vec!["BTC/USD".into(), "ETH/USD".into()],
            exchanges: vec!["kraken".into()],
            row_count: 1000,
            checksum: String::new(),
            created_at: Utc::now(),
            git_commit: "abc123".into(),
            config_hash: "def456".into(),
            parent_dataset: None,
        };

        let checksum = manifest.compute_checksum();
        assert!(!checksum.is_empty());

        // Same symbols in different order should produce same checksum
        let mut manifest2 = manifest.clone();
        manifest2.symbols = vec!["ETH/USD".into(), "BTC/USD".into()];
        assert_eq!(checksum, manifest2.compute_checksum());
    }
}