#signal-ensemble

## Responsibility

Signal generation and ensemble modeling.

## Source Map

- `// QuantRadar signal ensemble and meta-model for signal aggregation.`

- `use quantaradar_core::{Direction, Regime, SignalFamily};`

- `use serde::{Deserialize, Serialize};`

- `pub struct Signal {`

- `pub symbol: String,`

## Dependencies

- `quantaradar-core`

- `serde.workspace`

- `serde_json.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `signal-ensemble` crate in the QuantRadar workspace
