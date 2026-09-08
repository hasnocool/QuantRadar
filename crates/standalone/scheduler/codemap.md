# crates/standalone/scheduler/codemap

## Responsibility
Task scheduling and execution.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → scheduler computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
