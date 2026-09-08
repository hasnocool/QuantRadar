# crates/shared/storage/codemap

## Responsibility
Persistent storage layer (feature store, dataset persistence).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → storage computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
