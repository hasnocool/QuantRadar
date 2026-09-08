# crates/standalone/test-scale/codemap

## Responsibility
Test-scale and load-testing.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → test-scale computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
