#universe

## Responsibility

Module functionality to be documented.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl Universe { pub fn new() -> Self { Self { symbols: vec!["BTC/USD".into()] } } pub fn add(&mut self, s: &str) { self.symbols.push(s.into()); } }`

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

- Part of the `universe` crate in the QuantRadar workspace
