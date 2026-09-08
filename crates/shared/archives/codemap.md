# crates/shared/archives/codemap

## Responsibility
Archive/storage of historical datasets and replay artifacts.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → archives computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
