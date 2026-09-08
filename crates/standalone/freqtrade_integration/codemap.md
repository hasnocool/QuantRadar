# crates/standalone/freqtrade_integration/codemap

## Responsibility
Freqtrade integration adapter.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → freqtrade_integration computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
