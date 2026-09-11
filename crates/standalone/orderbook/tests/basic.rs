use orderbook::*; #[test] fn orderbook_spread() { let b = OrderBookL2::new(vec![L2Depth { price: 100.0, qty: 1.0 }], vec![L2Depth { price: 100.5, qty: 1.0 }], 0, 0); assert!(b.spread() == 0.5); }
