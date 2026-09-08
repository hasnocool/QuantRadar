# crates/shared/dashboard/codemap

## Responsibility
Web/dashboard reporting and visualization.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → dashboard computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
