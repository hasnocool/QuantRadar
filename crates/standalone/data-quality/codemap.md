# data-quality

## Responsibility
Validation metrics, completeness checks, and anomaly flags for ingestion and feature streams.

## Source Map
- `src/lib.rs`: `QualityReport`, `assess_quality()`, tests

## Dependencies
- `anyhow`, `serde`, `chrono`

## Tests
- `cargo test -p data-quality`
