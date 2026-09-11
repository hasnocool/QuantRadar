#pipeline

## Responsibility

Module functionality to be documented.

## Source Map

- `pub struct Pipeline { stages: Vec<String>, active: bool, results: Vec<String> }`

- `impl Pipeline {`

- `}`

- `mod verify_output {`

- `let manifest = env!("CARGO_MANIFEST_DIR");`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `pipeline` crate in the QuantRadar workspace
