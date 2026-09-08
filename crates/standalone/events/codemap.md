# crates/standalone/events/codemap

## Responsibility
Event definitions (trade, quote, order).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → events computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
