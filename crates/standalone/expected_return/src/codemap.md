#expected_return

## Responsibility

Expected return computation.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl ExpectedReturn { pub fn compute(price: f64) -> Self { Self{rate: 0.05, confidence: 0.8} } }`

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

- Part of the `expected_return` crate in the QuantRadar workspace
