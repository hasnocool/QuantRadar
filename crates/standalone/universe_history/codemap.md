# crates/standalone/universe_history/codemap

## Responsibility
Universe history tracking.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → universe_history computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
