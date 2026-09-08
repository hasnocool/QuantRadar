# crates/standalone/regime-detector/codemap

## Responsibility
Regime detection classifier.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → regime-detector computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
