# crates/shared/experiment/codemap

## Responsibility
Experiment registry and lineage tracking.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → experiment computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
