#monitoring

## Responsibility

Module functionality to be documented.

## Source Map

- `use serde::{Serialize, Deserialize};`

- `use std::collections::HashMap;`

- `impl MonitorHealth { pub fn check() -> Self { Self{healthy:true, latency_ms:5} } }`

- `// Minimal feed-health snapshot (primitives only — no dep on websocket crate).`

- `impl Default for HealthRegistry { fn default() -> Self { Self { feeds: HashMap::new(), critical_score: 0.5 } } }`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `monitoring` crate in the QuantRadar workspace
