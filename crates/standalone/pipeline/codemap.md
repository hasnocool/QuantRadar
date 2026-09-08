# crates/standalone/pipeline/codemap

## Responsibility
Data pipeline orchestration.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → pipeline computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
