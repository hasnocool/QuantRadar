# crates/standalone/paper-trading/codemap

## Responsibility
Paper-trading state machine.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → paper-trading computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
