# crates/shared/expected-return/codemap

## Responsibility
Expected-return modeling and promotion scoring.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → expected-return computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
