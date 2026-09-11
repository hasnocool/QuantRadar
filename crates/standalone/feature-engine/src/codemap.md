#feature-engine

## Responsibility

Feature computation and engineering.

## Source Map

- `use serde::{Deserialize, Serialize};`

- `use anyhow::Result;`

- `pub struct FeatureSet {`

- `pub mean: f64,`

- `pub std_dev: f64,`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `feature-engine` crate in the QuantRadar workspace
