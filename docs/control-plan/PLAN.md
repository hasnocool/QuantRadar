↓
Hypothesis generator
  ↓
Strategy DSL
  ↓
Parameter search
  ↓
Backtest
  ↓
OOS validation
  ↓
Robustness
  ↓
Promotion
```

Potential strategy generators:

* rule-based composition
* genetic programming
* symbolic regression
* feature selection
* Bayesian optimization
* Optuna-style search
* constrained combinatorial search
* ML policy candidates

But crucially, every generated strategy must go through the same anti-overfitting pipeline.

---

# 9. Backtesting is currently much too simplistic

This is one of the biggest weaknesses.

The current backtester is essentially an SMA crossover engine with fee and slippage assumptions.

It does **not** yet provide the realism you'd want for the project.

Missing:

### Execution realism

* bid/ask execution
* limit-order behavior
* partial fills
* order queue
* latency
* market impact
* spread widening
* liquidity depletion
* order rejection
* order expiry
* cancellation
* slippage distributions

### Portfolio realism

* multiple simultaneous symbols
* cash management
* leverage
* margin
* borrowing
* shorting
* funding
* collateral
* portfolio heat
* factor exposure
* correlation constraints
* rebalancing

### Statistical realism

* bootstrap
* Monte Carlo paths
* trade-order randomization
* block bootstrap
* regime permutation
* parameter perturbation
* transaction-cost perturbation

---

# 10. There is a concrete execution bug right now

This one is important.

Screeners generate:

```rust
direction: "long"
```

while `approve()` checks:

```rust
if signal.direction != "LONG"
```

So the screener output and risk/execution layer don't agree on the enum/string format.

That means the current approval path can reject valid screener signals.

The screener explicitly creates `"long"` signals.

The execution module explicitly requires `"LONG"`.

**This should be fixed immediately by replacing free-form strings with a strongly typed `Direction` enum.**

---

# 11. Paper trading is not yet a real paper-trading engine

The paper account is currently very small:

```text
cash
peak_equity
equity
realized_pnl
fills
positions
```

Missing:

* unrealized PnL
* equity mark-to-market
* position lifecycle
* stop orders
* take-profit orders
* trailing stops
* order state machine
* cancellations
* rejected orders
* partial fills
* pending orders
* reconciliation
* portfolio snapshots
* daily PnL
* risk events
* broker/exchange state comparison

Also, `PaperAccount` derives `Default`, which leaves numeric fields at zero rather than initializing a coherent starting account state.

---

# 12. Risk management needs to become portfolio-level

Current sizing is a useful start, but the system doesn't yet have full institutional-style portfolio risk.

Missing:

```text
portfolio VaR
CVaR
volatility targeting
beta targeting
factor exposure
cluster limits
correlation limits
gross exposure
net exposure
leverage limits
drawdown throttling
risk-of-ruin
liquidity-adjusted exposure
stress testing
scenario analysis
```

And dynamic risk scaling:

```text
normal regime → 100% risk
high vol → 50%
extreme vol → 25%
market dislocation → 0%
```

---

# 13. The validation system is conceptually good but operationally incomplete

The validation document says the correct things:

* train/test separation
* expanding WFO
* regime segmentation
* OOS trade count
* cost perturbation
* leakage review
* concentration controls
* failed experiment retention

But the actual Python implementation is still quite small.

The current robustness module mainly does parameter scaling and basic pass-rate logic.

You still need:

### Walk-forward engine

```text
train
validate
test
roll forward
repeat
```

### Multiple-test protection

This is particularly important because automated strategy generation can generate thousands of hypotheses.

Add:

* multiple hypothesis correction
* false-discovery controls
* Deflated Sharpe Ratio
* Probability of Backtest Overfitting
* White's Reality Check
* Hansen SPA
* nested walk-forward validation

Otherwise the system will eventually select lucky strategies.

---

# 14. No experiment registry

This is a major missing component.

Every experiment should produce:

```text
experiment_id
strategy_id
git_commit
dataset_id
universe_id
config_id
feature_version
model_version
timestamp
random_seed
training_period
validation_period
test_period
metrics
cost assumptions
artifacts
promotion decision
```

Then:

```text
Experiment
    ↓
Candidate
    ↓
Backtest
    ↓
Validation
    ↓
Robustness
    ↓
Champion/Challenger
```

This is essential for reproducibility.

---

# 15. No model registry

You're going to need something like:

```text
models/
  champion/
  challengers/
  retired/
```

with metadata:

```text
model_id
version
features
training_data
hyperparameters
metrics
regimes
deployment_status
```

Eventually:

```text
Champion
   ↑
Challenger
   ↓
Shadow
   ↓
Retired
```

---

# 16. No feature/data lineage system

This is particularly important for avoiding accidental leakage.

Every feature should know:

```text
feature_id
formula
inputs
lookback
minimum_history
availability_delay
version
```

Example:

```text
momentum_24h
inputs:
  close[t]
  close[t-24]

available_at:
  t
```

versus:

```text
future_return_24h
available_at:
  t+24h
```

The system should mechanically prevent the latter from entering a feature matrix.

---

# 17. Universe construction is underdeveloped

Dynamic discovery exists, which is good.

But you need historical universe membership.

Otherwise you get survivorship bias.

You need:

```text
Universe(t)
```

rather than:

```text
Universe(now)
```

Historical backtests need to know:

* which assets existed
* when they listed
* when they delisted
* which markets were suspended
* liquidity at that time
* price availability at that time

---

# 18. No delisting / listing / corporate-event-style handling

For crypto that translates to:

* listings
* delistings
* migrations
* symbol changes
* token redenominations
* chain migrations
* wrapped/unwrapped assets
* exchange-specific symbol changes

Those events need to become first-class data.

---

# 19. No multi-exchange architecture

The repository is currently centered on Kraken.

That's fine for the first implementation, but the eventual architecture should be:

```text
Exchange trait
 ├── Kraken
 ├── Coinbase
 ├── Binance
 ├── OKX
 ├── Bybit
 ├── Bitfinex
 └── ...
```

Then normalize everything into common domain models.

This also enables:

* cross-exchange arbitrage signals
* price dislocations
* venue liquidity comparison
* cross-venue volume
* lead/lag relationships

---

# 20. No derivatives data layer

For a serious crypto quant platform, this is a huge omission.

Eventually add:

* perpetual futures
* funding rates
* open interest
* liquidations
* basis
* futures term structure
* mark/index price
* long/short ratios
* options
* implied volatility
* skew
* volatility surface

Then signals become much stronger:

```text
price
+ spot volume
+ order flow
+ open interest
+ funding
+ liquidations
+ basis
```

---

# 21. No event intelligence layer

The current event detection is basic:

* new high
* new low
* volume anomaly
* volatility spike

You want a proper event bus:

```text
Event
 ├── market
 ├── asset
 ├── liquidity
 ├── volatility
 ├── derivatives
 ├── exchange
 └── external/news
```

Examples:

```text
LIQUIDITY_COLLAPSE
FUNDING_EXTREME
OI_SURGE
LIQUIDATION_CASCADE
VOL_BREAKOUT
SPREAD_EXPANSION
ORDERBOOK_IMBALANCE
CORRELATION_BREAK
REGIME_CHANGE
MOMENTUM_FAILURE
```

---

# 22. No news/on-chain/sentiment layer

For your long-term vision, this is another major expansion area.

Add optional data providers for:

### News

* headlines
* sentiment
* entity extraction
* event classification
* source credibility
* novelty

### On-chain

* exchange inflows/outflows
* whale transfers
* active addresses
* stablecoin supply
* token velocity
* holder concentration
* staking flows

These should become features, not hard-coded trading decisions.

---

# 23. No feature store

You need a reusable feature store instead of recomputing everything ad hoc.

Something like:

```text
FeatureStore
  ├── price
  ├── volume
  ├── technical
  ├── microstructure
  ├── cross-sectional
  ├── regime
  ├── derivatives
  ├── on-chain
  └── event
```

with timestamp-safe retrieval:

```python
features.as_of(timestamp)
```

---

# 24. No real-time signal pipeline

The repo has individual components, but not yet the production stream:

```text
market event
   ↓
state update
   ↓
feature update
   ↓
regime update
   ↓
screener evaluation
   ↓
cross-sectional ranking
   ↓
signal aggregation
   ↓
portfolio risk
   ↓
order intent
   ↓
execution simulator
```

That's the core missing "nervous system".

---

# 25. No signal ensemble / meta-model

Currently screeners operate independently.

You need:

```text
Trend              ─┐
Breakout            │
Mean Reversion      │
Vol Expansion       ├──> Signal Ensemble
Microstructure      │
Relative Strength   │
Anomaly             │
Event               ┘
```

Then:

```text
raw signal
↓
confidence
↓
regime compatibility
↓
liquidity
↓
cross-sectional rank
↓
expected return
↓
risk-adjusted score
```

---

# 26. No expected-return model

A score of `0.82` is not an expected return.

The eventual system should estimate something like:

```text
P(up)
expected return
expected adverse excursion
expected favorable excursion
expected holding period
expected volatility
confidence
```

Then you can calculate:

```text
expected alpha / expected risk / expected cost
```

instead of simply ranking heuristic scores.

---

# 27. No holding-period model

Different strategies have different horizons.

Add predictions for:

```text
5m
15m
1h
4h
1d
3d
7d
```

This is important for combining strategies without mixing incompatible signals.

---

# 28. No automated research scheduler

Eventually the platform should continually run:

```text
discover markets
↓
update datasets
↓
calculate features
↓
run screeners
↓
genera...
```

---

# Phase 2 Audit — Production WebSocket Engine (2026-09-08)

> **Status**: Partially implemented — skeleton/structure present, live exchange integration missing

## Phase 2 Checklist (Original Plan Items)

| # | Item | PLAN Status | Actual State |
|---|------|-------------|--------------|
| 2.1 | MarketFeedManager with bounded concurrency | [x] COMPLETED | **Implemented** (`crates/shared/websocket/`, 981 lines) — `FeedManager` with `Arc<RwLock<HashMap>>`, `tokio::Semaphore`, `VecDeque` buffers, `MessageRateLimiter`. Simulated connection only (comments: `// In production: use tokio-tungstenite`). |
| 2.2 | Adaptive rate limiter (replace fixed sleep) | [x] COMPLETED | **Implemented** (`crates/standalone/rate-limiter/`, 392 lines + `crates/shared/websocket/` uses `MessageRateLimiter`). Replaces fixed 350ms sleep (`crates/shared/exchange-kraken/src/lib.rs` had `sleep(Duration::from_millis(350))`). |
| 2.3 | Reconnect/backoff, heartbeat monitoring | [x] COMPLETED | **Implemented** (`crates/shared/websocket/`). `spawn_feed_task()` with exponential backoff (`*1.5` + 1000ms jitter), `reconnect_attempts` cap (default 10), `heartbeat_interval` (30s), `tokio::time::interval`. `FeedHealth` tracks `reconnect_count`, `last_reconnect`, `stale`. |
| 2.4 | Subscription management, sequence validation | [x] COMPLETED | **Implemented** (`crates/shared/websocket/`). `Subscription` struct (symbol/exchange/data_types/depth/interval). `SubscriptionStatus` enum (`Unsubscribed → Subscribing → Subscribed → Failed`). `handle_message()` validates sequence (`last_sequence + 1` expected), detects gaps (`seq > last + 1`), rejects duplicates (`seq == last`), rejects out-of-order (`seq < last`). `reconnect_feed()` triggers replay from last sequence. |
| 2.5 | Stale-feed detection, auto-resubscription | [x] COMPLETED | **Implemented** (`crates/shared/websocket/`). `check_stale_feeds()` compares `last_message` vs `stale_timeout` (60s). `auto_resubscribe_stale()` reconnects stale feeds if `enable_auto_resubscribe` (default true). Per-feed `stale` bool updated in `update_feed_health()`. |
| 2.6 | Per-symbol buffers, backpressure, feed health scores | [x] COMPLETED | **Implemented** (`crates/shared/websocket/`). Per-symbol `message_buffer: VecDeque` (`max_buffer_size` default 100,000). Global `global_buffer: VecDeque`. `FeedHealth` computes `health_score` (0.0–1.0) from message count, drop rate, error rate, staleness, sequence OK, latency. `FeedManagerStats` tracks total feeds, active feeds, messages, drops, reconnects, avg health. `message_buffer.pop_front()` on overflow increments `dropped_count`. |

---

## What's Actually In The Code

### `crates/shared/websocket/src/lib.rs` (981 lines, the core Phase 2 artifact)

- **Feed manager**: `MarketFeedManager::new()` returns `(Self, mpsc::Receiver<MarketDataMessage>)`.
- **Bounded concurrency**: `feed_semaphore: Arc<tokio::sync::Semaphore>` with `max_concurrent_feeds` (default 100); `MessageRateLimiter` with `max_concurrent_messages` (default 50).
- **Reconnection**: `spawn_feed_task()` loops with `reconnect_delay` exponentially increasing (`*1.5`) + jitter (`rand::random::<u64>() % 1000`). Breaks on `max_reconnect_attempts`.
- **Heartbeat**: Separate `tokio::spawn()` with `interval(heartbeat_interval)` sending pings; breaks when feed disconnects.
- **Sequence validation**: `handle_message()` checks `seq == last_sequence` (duplicate), `seq > last + 1` (gap — increments `dropped_count`, sets `sequence_ok = false`, triggers reconnect if gap >= `sequence_gap_threshold`), `seq < last` (out-of-order — warns).
- **Late event rejection**: `ts < last_processed_time - max_lateness_ms` → `late_event_count++`; message rejected (`return Ok(false)`).
- **Buffer management**: Per-feed `VecDeque` + global `Mutex<VecDeque>`. Overflows drop oldest (`pop_front()`). Stats updated (`stats.dropped_messages += 1`).
- **Health scoring**: `update_feed_health()` calculates `messages_per_second`, `health_score` (0.0–1.0, penalized for drops/errors/stale/sequence failures), `avg_latency_ms` (exponential moving average, 0.9/0.1 weighting).
- **Stale detection**: `check_stale_feeds()` uses `stale_timeout` (60s) vs `last_message`. `auto_resubscribe_stale()` reconnects.
- **Subscription API**: `add_feed()`, `start_feed()`, `subscribe()` (adds `Subscription` to pending or sends immediately if connected), `unsubscribe()` (retains non-matching subs), `get_buffered_messages()`, `get_global_buffer()`, `reconnect_feed()`, `shutdown()`.
- **Tests**: `test_feed_manager_creation`, `test_add_feed`, `test_handle_message`, `test_sequence_gap_detection`, `test_stale_detection`, `test_buffer_management` — all pass.
- **Limitation**: The connection loop is simulated (`// Simulate WebSocket connection (in production, use tokio-tungstenite)`). No real `tokio-tungstenite` or `ws` client is wired. No actual Kraken/Binance/Coinbase endpoint is invoked — only `info!("Connecting to {} feed for {}", exchange, symbol)` and simulated `MarketDataMessage` injection.

### `crates/standalone/rate-limiter/src/lib.rs` (392 lines)

Replaces `crates/shared/exchange-kraken/src/lib.rs` fixed `sleep(Duration::from_millis(350))`. Adaptive token-bucket style rate limiter with configurable `RateLimiterConfig`. Used by websocket `MarketFeedManager` via `MessageRateLimiter::process_message()`.

### `crates/shared/ingestion/src/lib.rs` (500 lines)

- `Collector` trait (`source_kind()`, `collect()` → `Vec<MarketObservation>`).
- `Normalizer`: `normalize_bar()`, `normalize_trade()`, `normalize_orderbook()`.
- `QualityValidator`: `validate()` with `max_price_deviation_pct` (10%), `max_volume_zscore` (5.0), `max_spread_bps` (500 = 5%), `min_trade_count` (1). Uses `validate_observation()` from core + additional checks.
- `IngestionPipeline`: `submit_observation()`, `submit_trade()`, `submit_orderbook()`, batch flush (`flush_observations()`, `flush_trades()`, `flush_orderbooks()`), `start_flush_task()` (background `tokio::spawn()` with `interval()`), `stop()` (shutdown via `mpsc::Sender`). Stores to parquet via `MarketDataWriter`.
- `tests`: `test_normalizer`, `test_quality_validator`, `test_ingestion_pipeline` (writes parquet to tempdir).

---

## What's Still Missing to Make Phase 2 Production

| Capability | Status | Gap |
|------------|--------|-----|
| Real WebSocket client (`tokio-tungstenite` / `ws`) | ❗ Not wired | `spawn_feed_task()` never opens TCP/TLS. Connection is simulated. Need actual `Client` connecting to `wss://ws.kraken.com/v2` or equivalent. |
| Multi-exchange feed manager | ❗ Partial | `MarketFeedManager` supports arbitrary `exchange` strings, but only Kraken REST (`crates/shared/exchange-kraken/`, 137 lines) exists. No Binance/Coinbase/WebSocket endpoints for `add_feed()`. |
| Real heartbeat / ping-pong | ❗ Partial | `heartbeat_interval` sends `debug!("Sending heartbeat...")` but no actual WebSocket `ping`/`pong` frames. Need `tungstenite::protocol::frame::Frame::Ping` / `Pong`. |
| Sequence replay / gap fill from archive | ❗ Partial | `reconnect_feed()` triggers reconnect but no replay request to exchange (`replay_from_sequence`). `crates/standalone/replay/` (406 lines) exists but is not integrated with `MarketFeedManager`. |
| Per-symbol backpressure (drop policy) | ❌ Not configured | `max_buffer_size` is set but no configurable drop policy (drop oldest vs newest vs block). `FeedHealth.dropped_count` tracks but no alerting/threshold action beyond reconnect. |
| Feed health alerts / monitoring hooks | ❌ Not wired | `FeedHealth` computed but never emitted to `monitoring/` (5-line stub) or `dashboard/` (5-line stub). No `tracing::error!()` on critical health drop (only `warn!` on stale). |
| Actual subscription message format | ❌ Not implemented | `subscribe()` pushes to `pending_subscriptions` but never serializes `Subscription` to exchange-specific JSON (e.g., Kraken `{"event":"subscribe","pair":["XBT/USD"],"subscription":{"name":"trade"}}`). |
| Order-book delta updates | ❌ Not implemented | `DataType::OrderBook` exists but only `normalize_orderbook()` handles full snapshots. No delta (`+`/`-`/`update`) stream processing. `crates/shared/order-book/` (696 lines) has `OrderBookSnapshot` only. |

---

## Phase 2 Verdict

**Plan claims**: All 6 items complete (100%).  
**Actual**: **Structure is complete** (types, state machines, reconnect logic, health scoring, buffers, sequence validation, rate limiter, ingestion pipeline all have substantial code). **Live execution is incomplete** (no real WebSocket connection, no multi-exchange endpoints, no replay integration, no monitoring/alerting hooks, no subscription serialization).

**Count**: ~980 lines of production-grade feed-management logic + 392 lines of adaptive rate limiter + 500 lines of ingestion pipeline = **~1,870 lines of Phase 2 code**.  
**Remaining work to make Phase 2 truly production**: Wire `tokio-tungstenite` into `spawn_feed_task()`, implement exchange-specific subscription messages, integrate `replay` crate with reconnect, wire health metrics to monitoring stack, add delta-order-book stream handling.