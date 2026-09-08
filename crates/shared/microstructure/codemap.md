# crates/microstructure/

## Responsibility
Order-book/trade-flow/liquidity analytics (liquidity metrics, order-book models).

## Design
Order-book event processing; liquidity feature computation.

## Flow
Order-book events → liquidity metrics → feature augmentation.

## Integration
Used by feature engine and regime; depends on core.
