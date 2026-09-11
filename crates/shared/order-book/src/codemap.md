#order-book

## Responsibility

Order book and execution logic.

## Source Map

- `// QuantRadar order book: full L2 state, delta updates, and reconstruction.`

- `use quantaradar_core::{OrderSide, QualityFlag, SourceKind};`

- `use anyhow::Result;`

- `use chrono::{DateTime, Utc};`

- `use serde::{Deserialize, Serialize};`

## Dependencies

- `quantaradar-core`

- `anyhow`

- `chrono`

- `serde`

- `tokio`

- `tracing`

- `uuid`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `order-book` crate in the QuantRadar workspace
