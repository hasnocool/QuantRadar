#exchange-kraken

## Responsibility

Ingestion adapter for exchange-kraken.

## Source Map

- `// QuantRadar Kraken REST and WebSocket market-data clients.`

- `use anyhow::{Context, Result};`

- `use chrono::{DateTime, Utc};`

- `use quantaradar_core::{Bar, MarketId};`

- `use reqwest::Client;`

## Dependencies

- `anyhow.workspace`

- `chrono.workspace`

- `futures.workspace`

- `reqwest.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `tokio.workspace`

- `tokio-tungstenite`

- `quantaradar-core`

- `urlencoding`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `exchange-kraken` crate in the QuantRadar workspace
