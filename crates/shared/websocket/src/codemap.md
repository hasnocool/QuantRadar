#websocket

## Responsibility

Module functionality to be documented.

## Source Map

- `// QuantRadar production-grade WebSocket feed handler with bounded concurrency,`

- `// adaptive rate limiting, reconnect/backoff, heartbeat monitoring, subscription`

- `// management, sequence validation, stale-feed detection, auto-resubscription,`

- `// per-symbol buffers, backpressure, and feed health scores.`

- `use quantaradar_core::SourceKind;`

## Dependencies

- `quantaradar-core`

- `quantaradar-rate-limiter`

- `anyhow`

- `chrono`

- `serde`

- `serde_json`

- `tokio`

- `tracing`

- `uuid`

- `rand`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `websocket` crate in the QuantRadar workspace
