use quantaradar_paper_trading::{PaperAccount, Order, LiquidityFlag};
use quantaradar_core::OrderSide;

#[test]
fn paper_account_new() {
    let account = PaperAccount::with_initial_cash(1000.0);
    assert_eq!(account.cash(), 1000.0);
    assert_eq!(account.equity(), 1000.0);
}

#[test]
fn paper_account_submit_and_fill() {
    let mut account = PaperAccount::with_initial_cash(100_000.0);
    let order = Order::new_market("BTC/USD".to_string(), OrderSide::Buy, 1.0);
    let id = account.submit_order(order).unwrap();
    let fill = account.process_fill(id, 50_000.0, 1.0, 1.0, LiquidityFlag::Taker).unwrap();
    assert_eq!(fill.quantity, 1.0);
    assert_eq!(fill.price, 50_000.0);
    let order = account.orders().get(&id).unwrap();
    assert_eq!(order.status, quantaradar_paper_trading::OrderStatus::Filled);
    assert_eq!(account.positions().get("BTC/USD").unwrap().quantity, 1.0);
}

#[test]
fn paper_account_cancel_order() {
    let mut account = PaperAccount::with_initial_cash(100_000.0);
    let order = Order::new_market("BTC/USD".to_string(), quantaradar_core::OrderSide::Buy, 1.0);
    let id = account.submit_order(order).unwrap();
    account.cancel_order(id).unwrap();
    let cancelled = account.orders().get(&id).unwrap();
    assert_eq!(cancelled.status, quantaradar_paper_trading::OrderStatus::Cancelled);
}

#[test]
fn paper_account_snapshot_reconcile() {
    let mut account = PaperAccount::with_initial_cash(100_000.0);
    let order = Order::new_market("BTC/USD".to_string(), quantaradar_core::OrderSide::Buy, 1.0);
    let id = account.submit_order(order).unwrap();
    account.process_fill(id, 50_000.0, 1.0, 1.0, LiquidityFlag::Taker).unwrap();
    let snapshot = account.snapshot();
    let report = account.reconcile(&snapshot);
    assert!(report.cash_match);
    assert!(report.equity_match);
    assert!(report.position_mismatches.is_empty());
}