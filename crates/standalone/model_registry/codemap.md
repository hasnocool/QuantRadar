# crates/standalone/model_registry/codemap

## Responsibility
Model registry and versioning.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → model_registry computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
