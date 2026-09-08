# crates/standalone/validation/codemap

## Responsibility
Validation and schema checks.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → validation computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
