# crates/standalone/derivatives/codemap

## Responsibility
Derivatives/option analytics.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → derivatives computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
