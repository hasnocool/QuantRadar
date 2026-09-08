# crates/standalone/feature-store/codemap

## Responsibility
Feature persistence/store.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → feature-store computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
