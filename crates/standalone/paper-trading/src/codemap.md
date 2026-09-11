#paper-trading

## Responsibility

Module functionality to be documented.

## Source Map

- `pub struct PaperAccount { pub cash: f64, pub peak_equity: f64, pub equity: f64, pub unrealized_pnl: f64, pub realized_pnl: f64, pub fills: Vec<String>, pub positions: Vec<String>, pub stop_orders: Vec<String>, pub take_profit_orders: Vec<String>, pub pending: Vec<String> }`

- `impl PaperAccount {`

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

- Part of the `paper-trading` crate in the QuantRadar workspace
