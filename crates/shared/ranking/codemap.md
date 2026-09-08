# crates/shared/ranking/codemap

## Responsibility
Cross-sectional ranking and relative-strength.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → ranking computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
