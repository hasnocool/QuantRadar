# Phase 1 — Data Foundation & Typed Domain

Working version of the persistent market-data layer and strongly typed contracts.

## 1.1 Typed Domain Model (fixes the `"long"` vs `"LONG"` bug)
```rust
enum Direction { Long, Short, Flat }
enum OrderSide { Buy, Sell }
enum EventKind { Breakout, Breakdown, VolumeAnomaly, VolatilitySpike, RegimeChange }
```
Every signal, order, and event uses these enums — no free-form strings.

## 1.2 Immutable Observation Schema
```text
Observation {
  timestamp: u64 (ms since epoch),
  exchange: String,
  symbol: String,
  source: SourceKind (Rest | WebSocket | Replay),
  ingestion_time: u64,
  quality_flags: Vec<QualityFlag>,
  event_type: EventType,
}
```
Raw observations are append-only; never mutated after ingestion.

## 1.3 Minimal Persistent Storage Layout
```
data/
  raw/
    trades/
    books/
    ohlcv/
  normalized/
  manifests/
```
- `manifests/` holds dataset manifests (`dataset_id`, `start`, `end`, `symbols`, `checksum`).
- `normalized/` holds cleaned, sequence-validated events.

## 1.4 Replay Engine (minimum)
```bash
quantaradar replay dataset_2026_09_01 --symbol BTC/USD --start 2026-09-01T00:00:00Z
```
Replays exact market state, features, signals, orders, fills, portfolio, PnL from manifest + raw archive. Same input + same commit = same result.

## 1.5 Sequence Validation & Gap Detection
- Every WebSocket message carries `sequence_number`.
- Gaps trigger reconnect/replay from last known sequence.
- Late events are rejected if `timestamp < last_processed_time - max_lateness`.

## 1.6 Data Quality Validator (minimum)
Checks: timestamp monotonicity, symbol consistency, price non-negative, volume non-negative, spread >= 0. Fails fast; bad observations go to `rejected/` with reason.

## Working version complete when:
- [ ] `Direction` enum replaces all string directions.
- [ ] `Observation` schema is serialized/deserialized deterministically.
- [ ] Replay command produces identical output on same dataset.
- [ ] Sequence gaps are detected and replayed.
- [ ] Data quality validator runs on ingestion pipeline.
