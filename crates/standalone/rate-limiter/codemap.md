#rate-limiter

## Responsibility

Rate limiting for ingestion and API access.

## Source Map

- `// QuantRadar adaptive rate limiter for API and WebSocket connections.`

- `use anyhow::Result;`

- `use chrono::{DateTime, Utc};`

- `use serde::{Deserialize, Serialize};`

- `use std::collections::HashMap;`

## Dependencies

- `anyhow`

- `chrono`

- `serde`

- `tokio`

- `tracing`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `rate-limiter` crate in the QuantRadar workspace
