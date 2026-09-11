#live-exec

## Responsibility

Module functionality to be documented.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl LiveExec { pub fn start() -> Self { Self { active: true } } }`

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

- Part of the `live-exec` crate in the QuantRadar workspace
