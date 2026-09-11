#persistent-data

## Responsibility

Data ingestion and validation pipeline.

## Source Map

- `use anyhow::{Context, Result};`

- `use chrono::{DateTime, Utc};`

- `use serde::{Deserialize, Serialize};`

- `use std::collections::HashMap;`

- `use std::path::Path;`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `persistent-data` crate in the QuantRadar workspace
