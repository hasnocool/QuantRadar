# crates/standalone/hold-period/codemap

## Responsibility
Hold-period and position-duration logic.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → hold-period computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
