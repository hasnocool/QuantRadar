# crates/standalone/multi_exchange/codemap

## Responsibility
Multi-exchange data aggregation.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → multi_exchange computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
