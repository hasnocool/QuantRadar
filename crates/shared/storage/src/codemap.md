#storage

## Responsibility

Data storage and persistence.

## Source Map

- `// QuantRadar Parquet/Arrow storage layer for market data persistence.`

- `use quantaradar_core::{QualityFlag, SourceKind};`

- `use quantaradar_data_model::{MarketObservation, TradeObservation, OrderBookObservation, MarketDataBatch};`

- `use anyhow::{anyhow, Context, Result};`

- `use arrow::array::{Float64Array, Int32Array, StringArray, UInt64Array, ListArray, ArrayRef, BooleanArray};`

## Dependencies

- `quantaradar-core`

- `quantaradar-data-model`

- `chrono`

- `serde`

- `serde_json`

- `anyhow`

- `uuid`

- `arrow`

- `parquet`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `storage` crate in the QuantRadar workspace
