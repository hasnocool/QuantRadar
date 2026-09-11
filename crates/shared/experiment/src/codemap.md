#experiment

## Responsibility

Experiment framework and reproducibility.

## Source Map

- `// QuantRadar experiment registry: lineage, monitoring, autonomous promotion/retirement.`

- `use anyhow::{Context, Result};`

- `use chrono::{DateTime, Utc};`

- `use quantaradar_core::Regime;`

- `use serde::{Deserialize, Serialize};`

## Dependencies

- `chrono.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `uuid.workspace`

- `anyhow.workspace`

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `experiment` crate in the QuantRadar workspace
