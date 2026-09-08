# crates/standalone/live-exec/codemap

## Responsibility
Live execution adapter (disabled, isolated).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → live-exec computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
