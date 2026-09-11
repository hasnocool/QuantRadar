#features

## Responsibility

Feature computation and engineering.

## Source Map

- `// Deterministic technical indicators. No network or global mutable state.`

- `use quantaradar_core::{safe_return,Bar,FeatureRow};`

- `pub struct Lookback { pub required: usize }`

- `impl Lookback { pub const fn new(required: usize) -> Self { Self { required } } pub fn ok_at(&self, idx: usize) -> bool { idx >= self.required } }`

- `let closes:Vec<f64>=bars.iter().map(|b|b.close).collect();`

## Dependencies

- `chrono.workspace`

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `features` crate in the QuantRadar workspace
