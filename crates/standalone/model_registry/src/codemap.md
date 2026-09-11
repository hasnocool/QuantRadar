#model_registry

## Responsibility

Model management and registry.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl ModelEntry { pub fn register(name: &str) -> Self { Self{name:name.into(), version:1} } }`

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

- Part of the `model_registry` crate in the QuantRadar workspace
