#ranking

## Responsibility

Cross-sectional ranking and scoring.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `pub struct RankScore { pub symbol: String, pub score: f64 }`

- `impl RankScore { pub fn rank_all(items: &[(String, f64)]) -> Vec<RankScore> { items.iter().map(|(s,v)| RankScore{symbol:s.clone(),score:*v}).collect() } }`

- `mod verify_output {`

- `let manifest = env!("CARGO_MANIFEST_DIR");`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `ranking` crate in the QuantRadar workspace
