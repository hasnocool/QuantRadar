# crates/shared/ingestion/codemap

## Responsibility
Data ingestion pipeline (REST/WebSocket normalization).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → ingestion computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
