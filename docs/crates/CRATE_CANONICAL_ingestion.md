# CRATE_CANONICAL_ingestion.md — Canonical Ingestion Crate Documentation

## quantaradar-ingestion

### Purpose
Market data ingestion system responsible for collecting, normalizing, and distributing
OHLC and tick data from multiple exchanges. Forms the data acquisition layer for the
entire QuantRadar platform.

### Version
0.2.0 (workspace-managed)

### License
MIT (workspace license)

### Rust Version
1.85 (workspace requirement)

### Key Types and APIs

#### Exchange Collectors
- `KrakenOhlcCollector` — Kraken REST OHLC endpoint client
  - Methods: `fetch_ohlc(symbol: &str, timeframe: &str) -> Result<Vec<OHLC>, Error>`
  - Rate-limited: max 3 requests/second per symbol
  - Supports: `1m`, `5m`, `15m`, `1h`, `4h`, `1d` timeframes
- `BinanceWsCollector` — Binance WebSocket ticker and kline collector
  - Methods: `start(symbol: &str, callback: fn(OHLC) -> void)`, `stop()`
  - Real-time WebSocket connection with automatic reconnection
  - Supports: `1m`, `3m`, `5m`, `15m`, `1h`, `4h`, `1d` timeframes
- `CoinbaseProOhlcCollector` — Coinbase Pro REST OHLC client
  - Similar interface to Kraken collector

#### Data Normalization
- `normalize_ohlc(raw: RawOhlc) -> Result<NormalizedOhlc, NormalizationError>`
  - Strips exchange-specific fields
  - Ensures timestamp consistency (UTC)
  - Validates price ranges (no negative prices, high >= low >= open close range)
- `TickNormalizer` — Converts raw tick data into internal `Tick` type
  - Handles bid/ask mid-price calculation
  - Filters duplicate ticks by timestamp

#### Distribution
- `DataPublisher` — Broadcasts normalized OHLC to subscribers
  - `subscribe(symbol: &str, receiver: mpsc::Receiver<NormalizedOhlc>)`
  - `unsubscribe(symbol: &str)`
  - Uses `tokio::sync::mpsc` for async broadcasting
- `DataSubscriber` — Trait for consuming normalized data
  - `on_ohlc(ohlc: NormalizedOhlc)` — Called when new OHLC bar arrives

#### Error Types
- `IngestionError` — Enum with variants:
  - `RateLimitExceeded { symbol: String, retry_after: Duration }`
  - `NetworkError { source: std::io::Error, url: String }`
  - `InvalidResponse { source: serde_json::Error, raw: String }`
  - `ExchangeNotSupported { exchange: String, symbol: String }`
- `TimeoutError` — Request timeout handling

### Async Architecture
- All collector operations are `async`
- Uses `tokio` runtime with `rt-multi-thread`
- `mpsc` channels for fan-out to multiple consumers
- Graceful shutdown via `tokio::signal::ctrl_c`
- Metrics tracked via `tracing` subscriber

### Dependencies
**Runtime:**
- `tokio` v1 with macros, rt-multi-thread, time, sync, fs features
- `reqwest` v0.12 with json, rustls-tls features (for REST collectors)
- `futures` v0.3 — Stream processing
- `tracing` v0.1 — Instrumentation and logging
- `serde` v1 with derive — Data serialization
- `chrono` v0.4 with serde feature — Time handling

**Development:**
- No specific dev dependencies beyond workspace deps

### Integration Points

#### Used By
- `crates/shared/features` — Consumes OHLC data for feature engineering
- `crates/standalone/pipeline` — Receives market data for signal generation
- `crates/standalone/backtest_engine` — Historical data feed
- `crates/standalone/regime-detector` — Regime detection input
- `crates/standalone/signals` — Signal generation from features

#### Consumes From
- `crates/shared/data-model` — Uses OHLC/Tick types for normalization
- Exchange REST/WebSocket APIs (external)
- User-configured data feeds (external)

### Representative Code Example

```rust
use quantaradar_ingestion::{KrakenOhlcCollector, NormalizedOhlc};
use tokio::sync::mpsc;
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a channel for data distribution
    let (tx, rx) = mpsc::channel(100);

    // Subscribe to data
    let mut subscriber = rx.subscribe();

    // Start Kraken collector
    let collector = KrakenOhlcCollector::new("BTC/USD".to_string());
    collector.start(tx.clone()).await;

    // Process incoming bars
    while let Some(ohlc) = subscriber.recv().await {
        info!("Received {:?}", ohlc.mid_price());
        // Forward to pipeline or feature engineering
    }

    // Graceful shutdown
    collector.stop().await;
}
```

### Verification
- All collectors compile with `cargo check -p quantaradar-ingestion`
- Rate limiting tested with mock HTTP responses
- WebSocket connection/disconnection cycles verified
- Error path: invalid JSON responses handled correctly
- No deadlocks in async test suite