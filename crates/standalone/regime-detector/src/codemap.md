#regime-detector

## Responsibility

Regime detection and market regime analysis.

## Source Map

- `use serde::{Deserialize, Serialize};`

- `pub enum Regime { Bull, Bear, Neutral, Unknown }`

- `pub struct RegimeDetector {`

- `pub enabled: bool,`

- `pub config: String,`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `regime-detector` crate in the QuantRadar workspace
