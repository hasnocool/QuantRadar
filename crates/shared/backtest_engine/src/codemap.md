#backtest_engine

## Responsibility

Backtest execution and performance analysis.

## Source Map

- `use quantaradar_core::{Direction, OrderSide, Bar};`

- `use chrono::{DateTime, Utc, TimeZone};`

- `use serde::{Deserialize, Serialize};`

- `use std::collections::hash_map::DefaultHasher;`

- `use std::hash::{Hash, Hasher};`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `backtest_engine` crate in the QuantRadar workspace
