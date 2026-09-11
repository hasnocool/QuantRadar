#strategy_dsl

## Responsibility

Module functionality to be documented.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl StrategyExpr { pub fn parse(s: &str) -> Self { Self{expr:s.into()} } pub fn evaluate(&self) -> bool { !self.expr.is_empty() } }`

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

- Part of the `strategy_dsl` crate in the QuantRadar workspace
