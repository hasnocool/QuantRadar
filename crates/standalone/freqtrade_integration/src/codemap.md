#freqtrade_integration

## Responsibility

Frequency-domain analysis.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl FreqAdapter { pub fn connect() -> Self { Self { connected: true } } }`

- `mod verify_output {`

- `let manifest = env!("CARGO_MANIFEST_DIR");`

- `let pkg = env!("CARGO_PKG_NAME");`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `freqtrade_integration` crate in the QuantRadar workspace
