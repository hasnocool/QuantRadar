#dashboard

## Responsibility

Dashboard and visualization.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `use std::collections::HashMap;`

- `impl DashMetric { pub fn new(label: &str, value: f64) -> Self { Self { label: label.into(), value } } }`

- `// Minimal per-feed card (primitives only — no dep on websocket crate).`

- `impl DashboardFeeds {`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `dashboard` crate in the QuantRadar workspace
