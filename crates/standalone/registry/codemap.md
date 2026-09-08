# crates/standalone/registry/codemap

## Responsibility
Experiment/registry management.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → registry computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
