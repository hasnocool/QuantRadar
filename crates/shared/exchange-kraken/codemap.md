# crates/exchange-kraken/

## Responsibility
Kraken REST + WebSocket ingestion; normalization to core contracts.

## Design
REST polling + WebSocket ingestion; normalization layer.

## Flow
Kraken API → ingestion/normalization → core contracts.

## Integration
Produces market data for core; depends on core.
