#archives

## Responsibility

Module functionality to be documented.

## Source Map

- `// QuantRadar archives: tick/trades/book storage with gap detection, reconnect/replay.`

- `use quantaradar_core::{QualityFlag, SourceKind};`

- `use quantaradar_data_model::{MarketObservation, TradeObservation, OrderBookObservation, MarketDataBatch};`

- `use quantaradar_storage::{MarketDataWriter, StorageConfig};`

- `use anyhow::{Context, Result};`

## Dependencies

- `quantaradar-core`

- `quantaradar-data-model`

- `quantaradar-storage`

- `anyhow`

- `chrono`

- `serde`

- `tokio`

- `tracing`

- `uuid`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `archives` crate in the QuantRadar workspace
