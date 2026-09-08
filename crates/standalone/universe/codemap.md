# crates/standalone/universe/codemap

## Responsibility
Universe selection and filtering.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → universe computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
