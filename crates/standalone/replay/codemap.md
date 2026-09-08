# crates/standalone/replay/codemap

## Responsibility
Replay/rebuild with dataset integration.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → replay computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
