# python/quantaradar/

## Responsibility
Python research layer: ML exploration, walk-forward, robustness, champion/challenger gates.

## Design
Pydantic v2 schemas, uv + ruff, polars/duckdb for data, basedpyright types.

## Flow
Market data → feature engineering (polars) → ML/research → walk-forward → champion/challenger → promotion.

## Integration
Connects to Rust crates (core, features, backtest, screening) via data files (data/raw, reports) and Python package entry points.
