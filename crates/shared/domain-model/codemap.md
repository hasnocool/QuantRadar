# crates/shared/domain-model/codemap

## Responsibility
Domain entities (orders, trades, positions).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → domain-model computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
