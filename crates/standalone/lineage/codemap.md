# crates/standalone/lineage/codemap

## Responsibility
Dataset lineage and provenance.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → lineage computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
