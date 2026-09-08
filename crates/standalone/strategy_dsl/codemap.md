# crates/standalone/strategy_dsl/codemap

## Responsibility
Strategy DSL and contract definitions.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → strategy_dsl computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
