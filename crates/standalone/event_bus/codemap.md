# crates/standalone/event_bus/codemap

## Responsibility
Event bus for market-data distribution.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → event_bus computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
