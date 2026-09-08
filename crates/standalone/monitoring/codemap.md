# crates/standalone/monitoring/codemap

## Responsibility
Monitoring and health checks.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → monitoring computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
