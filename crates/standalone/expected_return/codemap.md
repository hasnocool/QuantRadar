# crates/standalone/expected_return/codemap

## Responsibility
Standalone expected-return computation.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → expected_return computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
