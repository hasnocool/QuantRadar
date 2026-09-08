# Phase 1 — Data Foundation & Typed Domain

Working version of the persistent market-data layer and strongly typed contracts.

## 1.1 Typed Domain Model (fixes the "long" vs "LONG" bug)
Direction variants: Long, Short, Flat. OrderSide variants: Buy, Sell. EventKind variants: Breakout, Breakdown, VolumeAnomaly, VolatilitySpike, RegimeChange.
Every signal, order, and event uses these enums — no free-form strings.

## 1.2 Immutable Observation Schema
11 observation field groups per ARCHITECTURE.md:42 — timestamp (ms since epoch); exchange; symbol; base/quote pair; OHLCV grouped as one field (open, high, low, close, volume); trade_count; bid/ask pair; bid_depth/ask_depth levels (price, quantity); source (Rest, WebSocket, or Replay); ingested_at; quality_flags.
Raw observations are append-only; never mutated after ingestion.

## 1.3 Minimal Persistent Storage Layout
Storage layout under data: raw with trades, books, and ohlcv archives; normalized for cleaned sequence-validated events; manifests for dataset manifests.
- `manifests/` holds dataset manifests (`dataset_id`, `start`, `end`, `symbols`, `checksum`).
- `normalized/` holds cleaned, sequence-validated events.

## 1.4 Replay Engine (minimum)
Replay invocation: quantaradar replay with dataset identifier, symbol, and start timestamp arguments.
Replays exact market state, features, signals, orders, fills, portfolio, PnL from manifest + raw archive. Same input + same commit = same result.

## 1.5 Sequence Validation & Gap Detection
- Every WebSocket message carries `sequence_number`.
- Gaps trigger reconnect/replay from last known sequence.
- Late events are rejected if `timestamp < last_processed_time - max_lateness`.

## 1.6 Data Quality Validator (minimum)
Checks: timestamp monotonicity, symbol consistency, price non-negative, volume non-negative, spread >= 0. Fails fast; bad observations go to `rejected/` with reason.

## 1.7 Replay / Historical Data Lifecycle
The historical data pipeline follows an 8-stage lifecycle:
1. **VENUE**: Exchange feed generation.
2. **RAW EVENT**: Ingested append-only raw events.
3. **NORMALIZE**: Canonical representation across exchanges.
4. **VALIDATE**: Sequence and data-quality validation.
5. **ARCHIVE**: Persistent immutable storage.
6. **REPLAYABLE DATASET**: Frozen dataset with manifest and checksum.
7. **DERIVED DATASET**: Normalized trades, books, and OHLCV bars.
8. **FEATURE SNAPSHOT**: Point-in-time features computed without leakage.

Stage gates: raw stages (VENUE through RAW EVENT) are immutable; NORMALIZE through ARCHIVE must be reproducible from the raw archive; DERIVED DATASET and FEATURE SNAPSHOT are versioned outputs.

## 1.8 Reproducibility Contract
Deterministic replay and research reproducibility require tracking:
- `code_commit`: Exact git commit SHA.
- `config_hash`: SHA-256 hash of the configuration file.
- `dataset_id`: Manifest dataset identifier.
- `feature_versions`: Version map of feature calculation logic.
- `strategy_version`: Strategy algorithm version.
- `model_version`: Statistical or ML model artifact version.
- `random_seed`: Seed used for any stochastic components.
- `execution_model_version`: Execution simulator version.

## 1.9 Design Goals
- **Goal 1**: Dynamic asset discovery instead of hard-coded universes.
- **Goal 2**: Bounded asynchronous concurrency.
- **Goal 3**: No data leakage.
- **Goal 7**: Liquidity as a hard constraint.
- **Goal 8**: Live execution isolated from research until promotion gates pass.

## Working version complete when:
- [x] `Direction` enum replaces all string directions.
- [x] `Observation` schema is serialized/deserialized deterministically.
- [x] Replay command produces identical output on same dataset.
- [x] Sequence gaps are detected and replayed.
- [x] Data quality validator runs on ingestion pipeline.
