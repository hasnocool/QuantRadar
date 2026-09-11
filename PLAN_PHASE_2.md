# QuantRadar Phase 2 — Production WebSocket Engine

> Status: Partially implemented — skeleton/structure present, live exchange integration missing  
> Created: 2026-09-08  
> Source: PLAN.md (Phase 2 — Weeks 3-4) + runtime audit  

---

## Phase 2 Checklist (from PLAN.md)

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
