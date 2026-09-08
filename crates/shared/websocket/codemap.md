# crates/shared/websocket/codemap

## Responsibility
WebSocket ingestion and event-bus delivery.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → websocket computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
