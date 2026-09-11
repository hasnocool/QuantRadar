#sentiment

## Responsibility

Module functionality to be documented.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl Sentiment { pub fn score_text(text: &str) -> Self { let s = if text.contains("good") { 0.8 } else if text.contains("bad") { -0.5 } else { 0.0 }; Self{score:s} } }`

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

- Part of the `sentiment` crate in the QuantRadar workspace
