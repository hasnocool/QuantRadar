#reporting

## Responsibility

Report generation and output formatting.

## Source Map

- `// QuantRadar machine-readable report writer.`

- `use anyhow::{Context, Result};`

- `use chrono::{DateTime, Utc};`

- `use serde::Serialize;`

- `use std::path::Path;`

## Dependencies

- `quantaradar-core`

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `serde_yaml`

- `chrono.workspace`

- `uuid`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `reporting` crate in the QuantRadar workspace
