#event_bus

## Responsibility

Event handling and market-data distribution.

## Source Map

- `use quantaradar_core::EventKind;`

- `use std::collections::HashMap;`

- `pub struct EventBus {`

- `subscribers: HashMap<String, Vec<String>>,`

- `events: Vec<EventKind>,`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `event_bus` crate in the QuantRadar workspace
