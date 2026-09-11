#events

## Responsibility

Event handling and market-data distribution.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl Event { pub fn new(s: &str, t: &str) -> Self { Self{symbol:s.into(), event_type:t.into()} } }`

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

- Part of the `events` crate in the QuantRadar workspace
