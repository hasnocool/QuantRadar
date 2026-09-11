#replay

## Responsibility

Module functionality to be documented.

## Source Map

- `use chrono::Utc;`

- `use quantaradar_core::{Direction, OrderSide, QualityFlag, SourceKind, SignalFamily};`

- `use quantaradar_data_model::{DatasetManifest, MarketObservation};`

- `use anyhow::{Context, Result};`

- `use sha2::{Sha256, Digest};`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

- `sha2.workspace`

- `quantaradar-core`

- `quantaradar-data-model`

- `parquet`

- `arrow`

- `uuid`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `replay` crate in the QuantRadar workspace
