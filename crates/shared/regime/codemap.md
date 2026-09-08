# crates/regime/

## Responsibility
Market-state classifier (regime confidence, regime detector, RegimeThresholds).

## Design
Classifier with typed confidence scores; integrates with feature engine outputs.

## Flow
Features → regime classification → confidence → regime-scaled screening/backtest.

## Integration
Used by screeners, portfolio, backtest; depends on core, features.
