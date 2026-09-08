# crates/shared/order-book/codemap

## Responsibility
Order-book models and depth analytics.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → order-book computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
