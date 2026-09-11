#research

## Responsibility

Module functionality to be documented.

## Source Map

- `// QuantRadar research analytics: breadth, relative strength, ranking, correlation/PCA, events and strategy generation.`

- `use quantaradar_core::{FeatureRow, Regime, EventKind, StrategyFamily};`

- `use chrono::{DateTime, Utc};`

- `use serde::{Deserialize, Serialize};`

- `pub mod factor_engine;`

## Dependencies

- `chrono.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `quantaradar-core`

- `rand`

- `thiserror`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `research` crate in the QuantRadar workspace
