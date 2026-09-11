# CRATE_CANONICAL_core.md — Canonical Core Crate Documentation

## quantaradar-core

### Purpose
Core domain models and foundational types for the QuantRadar quantitative research platform.
Provides the typed foundation upon which all other crates build.

### Version
0.2.0 (workspace-managed)

### License
MIT (workspace license)

### Rust Version
1.85 (workspace requirement)

### Key Types and APIs

#### Market Data Types
- `OHLC` — Open-High-Low-Close price bar with timestamp
  - Fields: `timestamp: chrono::DateTime<Utc>`, `open: f64`, `high: f64`, `low: f64`, `close: f64`, `volume: f64`
  - Methods: `mid_price()`, `is_up_tick()`, `bar_length()`
- `Tick` — Raw tick data from exchange WebSocket
  - Fields: `symbol: String`, `price: f64`, `volume: f64`, `timestamp: chrono::DateTime<Utc>`
- `Trade` — Executed trade record
  - Fields: `id: UUID`, `price: f64`, `volume: f64`, `bid: bool`, `timestamp: chrono::DateTime<Utc>`

#### Order Types
- `Order` — Client order representation
  - Fields: `id: UUID`, `symbol: String`, `side: OrderSide`, `order_type: OrderType`, `price: Option<f64>`, `quantity: f64`, `status: OrderStatus`, `timestamp: chrono::DateTime<Utc>`
  - Methods: `is_active()`, `is_filled()`, `average_fill_price()`
- `OrderSide` — Enum: `Buy`, `Sell`
- `OrderType` — Enum: `Market`, `Limit`, `Stop`, `StopLimit`
- `OrderStatus` — Enum: `New`, `PartiallyFilled`, `FullyFilled`, `Canceled`, `Replaced`

#### Portfolio Types
- `Position` — Holdings of a symbol
  - Fields: `symbol: String`, `quantity: f64`, `average_price: f64`, `unrealized_pnl: f64`, `realized_pnl: f64`
  - Methods: `pnl()`, `market_value()`, `update(fills: &[Fill])`
- `Fill` — Order fill record
  - Fields: `id: UUID`, `order_id: UUID`, `price: f64`, `quantity: f64`, `commission: f64`, `timestamp: chrono::DateTime<Utc>`

#### UUID and Identifiers
- `order_id`, `symbol`, `trade_id` — Type aliases for UUID-based identifiers
- Uses `uuid` crate v1 with serde support for serialization

### Dependencies
**Runtime:**
- `anyhow` v1 — Error handling
- `chrono` v0.4 with serde — Time types and serialization
- `serde` v1 with derive — JSON/XML serialization
- `serde_json` v1 — JSON handling
- `uuid` v1 with serde — Unique identifiers

**Development:**
- `proptest` v1 — Property-based testing
- `rand` v0.8 — Random number generation for simulations

### Integration Points

#### Used By
- `crates/shared/data-model` — Consumes OHLC/Tick/Trade types
- `crates/shared/execution` — Uses Order/Position types
- `crates/shared/features` — Uses MarketData for feature engineering
- `crates/standalone/signal-ensemble` — Uses core types for signal modeling
- `crates/standalone/paper-trading` — Uses Order/Position/fill types

#### Consumes From
- `crates/standalone/ingestion` — Receives raw Tick/OHLC data
- `crates/standalone/exchange-kraken` — Parses exchange responses into core types
- `crates/standalone/order-book` — Provides order book snapshots as OHLC bars

### Representative Code Example

```rust
use quantaradar_core::{OHLC, Trade, Order, OrderSide};
use chrono::Utc;
use uuid::Uuid;

// Create an OHLC bar
let bar = OHLC {
    timestamp: Utc::now(),
    open: 100.0,
    high: 105.0,
    low: 99.0,
    close: 103.0,
    volume: 1500.0,
};

// Check if bar closed up
if bar.is_up_tick() {
    println!("Bullish bar close");
}

// Create a limit buy order
let order = Order {
    id: Uuid::new_v4(),
    symbol: "BTC/USD".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    price: Some(99.5),
    quantity: 0.1,
    status: OrderStatus::New,
    timestamp: Utc::now(),
};

// Check order status
assert!(order.is_active());
assert!(!order.is_filled());
```

### Verification
- All types compile with `cargo check -p quantaradar-core`
- Serialization round-trips verified via serde
- No unused import warnings
- Type assertions pass in test module