#multi_exchange

## Responsibility

Ingestion adapter for multi_exchange.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl ExchangeRef { pub fn list() -> Vec<Self> { vec![Self{name:"kraken".into()}, Self{name:"binance".into()}] } }`

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

- Part of the `multi_exchange` crate in the QuantRadar workspace
