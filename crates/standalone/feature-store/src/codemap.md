#feature-store

## Responsibility

Feature computation and engineering.

## Source Map

- `// QuantRadar feature store with versioned persistence, lineage tracking, and point-in-time queries.`

- `use anyhow::Result;`

- `use chrono::{DateTime, Utc};`

- `use serde::{Deserialize, Serialize};`

- `use std::collections::{BTreeMap, HashMap};`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `feature-store` crate in the QuantRadar workspace
