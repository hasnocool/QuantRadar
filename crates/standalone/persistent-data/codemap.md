# crates/standalone/persistent-data/codemap

## Responsibility
Persistent market data and replay.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → persistent-data computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
