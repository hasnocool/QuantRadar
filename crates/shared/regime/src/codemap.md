#regime

## Responsibility

Regime detection and market regime analysis.

## Source Map

- `// Lightweight deterministic market-regime classifier.`

- `use quantaradar_core::FeatureRow;`

- `pub use quantaradar_core::Regime;`

- `use serde::{Deserialize, Serialize};`

- `pub struct RegimeThresholds { pub trend: f64, pub high_vol: f64, pub low_vol: f64 }`

## Dependencies

- `serde.workspace`

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `regime` crate in the QuantRadar workspace
