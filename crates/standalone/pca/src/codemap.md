#pca

## Responsibility

Dimensionality reduction and ranking.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `impl PcaResult { pub fn compute(values: &[f64]) -> Self { let mean = values.iter().sum::<f64>()/values.len() as f64; let var = values.iter().map(|v|(v-mean).powi(2)).sum::<f64>()/values.len() as f64; Self{variance:var} } }`

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

- Part of the `pca` crate in the QuantRadar workspace
