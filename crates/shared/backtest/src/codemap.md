#backtest

## Responsibility

Backtest execution and performance analysis.

## Source Map

- `// Cost-aware deterministic event-driven backtester with realistic execution.`

- `use chrono::{DateTime,Utc};`

- `use quantaradar_core::Bar;`

- `use quantaradar_microstructure::MicrostructureFeatures;`

- `use serde::{Deserialize,Serialize};`

## Dependencies

- `chrono.workspace`

- `serde.workspace`

- `quantaradar-core`

- `quantaradar-microstructure`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `backtest` crate in the QuantRadar workspace
