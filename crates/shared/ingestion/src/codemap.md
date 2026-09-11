#ingestion

## Responsibility

Data ingestion and validation pipeline.

## Source Map

- `// QuantRadar ingestion pipeline: collector, normalizer, quality validator.`

- `use quantaradar_core::{Bar, Observation, QualityCheckResult, QualityFlag, SourceKind, validate_bar, validate_observation, is_valid_price, is_valid_volume};`

- `use quantaradar_data_model::{MarketObservation, TradeObservation, OrderBookObservation, MarketDataBatch};`

- `use quantaradar_storage::{MarketDataWriter, StorageConfig, ManifestWriter, DatasetManifest, DatasetType};`

- `use anyhow::Result;`

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

- Part of the `ingestion` crate in the QuantRadar workspace
