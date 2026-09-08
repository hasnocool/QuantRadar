# crates/screeners/

## Responsibility
Explainable screener families (7 families, composite score, DSL contracts).

## Design
Screener DAG with explainable scores; DSL per ARCHITECTURE.md.

## Flow
Signals → screener families → composite score → candidate list.

## Integration
Used by pipeline, strategy DSL; depends on core, features, regime.
