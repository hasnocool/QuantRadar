#data-model

## Responsibility

Data ingestion and validation pipeline.

## Source Map

- `// QuantRadar immutable market data model with full observation fields.`

- `// Fields: timestamp, exchange, symbol, base, quote, OHLCV, trade_count, bid, ask, bid_depth, ask_depth, source, ingested_at, quality_flags`

- `use quantaradar_core::{Observation, OHLCV, QualityFlag, Regime, SourceKind};`

- `use chrono::{DateTime, Utc};`

- `use serde::{Deserialize, Serialize};`

## Dependencies

- `quantaradar-core`

- `chrono`

- `serde`

- `uuid`

- `anyhow`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `data-model` crate in the QuantRadar workspace
