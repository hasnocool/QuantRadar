# crates/standalone/feature-engine/codemap

## Responsibility
Feature computation engine.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → feature-engine computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
