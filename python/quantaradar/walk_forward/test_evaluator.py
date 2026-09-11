# Tests for walk-forward evaluator
import numpy as np
import pandas as pd
import pytest

from quantaradar.walk_forward.evaluator import (
    Fold,
    FoldType,
    WalkForwardConfig,
    expanding_folds,
    anchored_folds,
    rolling_folds,
    generate_folds,
    assign_regimes_to_folds,
    compute_metrics,
    OOSMetrics,
    run_walk_forward,
    aggregate_oos,
)


def test_expanding_folds_basic():
    folds = expanding_folds(n=100, initial_train=50, test_size=10, step=10)
    assert len(folds) == 5
    assert folds[0].train_start == 0
    assert folds[0].train_end == 50
    assert folds[0].test_start == 50
    assert folds[0].test_end == 60
    assert folds[-1].test_end == 100


def test_expanding_folds_invalid():
    with pytest.raises(ValueError):
        expanding_folds(n=100, initial_train=0, test_size=10)
    with pytest.raises(ValueError):
        expanding_folds(n=100, initial_train=50, test_size=60)


def test_anchored_folds():
    folds = anchored_folds(n=100, initial_train=50, test_size=10, step=10)
    assert len(folds) == 5
    assert folds[0].train_start == 0
    assert folds[0].train_end == 50
    assert folds[-1].test_end == 100


def test_rolling_folds():
    folds = rolling_folds(n=100, train_size=50, test_size=10, step=10)
    assert len(folds) == 5
    assert folds[0].train_start == 0
    assert folds[0].train_end == 50
    assert folds[-1].train_start == 40
    assert folds[-1].train_end == 90


def test_generate_folds_expanding():
    config = WalkForwardConfig(n=100, initial_train=50, test_size=10, fold_type=FoldType.EXPANDING)
    folds = generate_folds(config)
    assert len(folds) == 5


def test_generate_folds_rolling():
    config = WalkForwardConfig(n=100, initial_train=50, test_size=10, fold_type=FoldType.ROLLING)
    folds = generate_folds(config)
    assert len(folds) == 5


def test_assign_regimes_to_folds():
    folds = expanding_folds(n=100, initial_train=50, test_size=10, step=10)
    regimes = np.array([0]*60 + [1]*40)
    labeled = assign_regimes_to_folds(folds, regimes, regime_labels=["bull", "bear"])
    assert labeled[0].regime == "bull"
    assert labeled[-1].regime == "bear"


def test_compute_metrics_empty():
    metrics = compute_metrics(pd.Series(dtype=float))
    assert metrics.total_return == 0.0
    assert metrics.sharpe == 0.0


def test_compute_metrics_positive_returns():
    returns = pd.Series([0.01, 0.02, -0.005, 0.015, 0.01])
    metrics = compute_metrics(returns)
    assert metrics.total_return > 0
    assert metrics.sharpe > 0
    assert metrics.max_drawdown >= 0
    assert metrics.profit_factor >= 0


def test_compute_metrics_with_trades():
    returns = pd.Series([0.01, 0.02, -0.005, 0.015, 0.01])
    trades = pd.DataFrame({"pnl": [100, -50, 200, -30, 150]})
    metrics = compute_metrics(returns, trades)
    assert metrics.num_trades == 5
    assert metrics.win_rate == 0.6
    assert metrics.profit_factor > 1


def test_compute_metrics_all_winning():
    returns = pd.Series([0.01, 0.02, 0.015])
    trades = pd.DataFrame({"pnl": [100, 200, 150]})
    metrics = compute_metrics(returns, trades)
    assert metrics.profit_factor == float("inf")
    assert metrics.win_rate == 1.0


def test_run_walk_forward_basic():
    np.random.seed(42)
    n = 200
    returns = pd.Series(np.random.normal(0.0005, 0.01, n))
    config = WalkForwardConfig(n=n, initial_train=100, test_size=20, validate_size=10, step=20)

    def strategy(train_rets, train_idx):
        return np.ones_like(train_rets) * np.sign(np.mean(train_rets))

    results = run_walk_forward(returns, config, strategy)
    assert len(results) > 0
    for r in results:
        assert isinstance(r.test_metrics, OOSMetrics)
        assert r.fold.test_end <= n


def test_run_walk_forward_with_regimes():
    np.random.seed(42)
    n = 200
    returns = pd.Series(np.random.normal(0.0005, 0.01, n))
    regimes = np.array([0]*100 + [1]*100)
    config = WalkForwardConfig(n=n, initial_train=100, test_size=20, step=20, regimes=regimes, regime_labels=["bull", "bear"])

    def strategy(train_rets, train_idx):
        return np.ones_like(train_rets) * np.sign(np.mean(train_rets))

    results = run_walk_forward(returns, config, strategy)
    assert len(results) > 0
    for r in results:
        assert r.regime in ("bull", "bear")


def test_aggregate_oos_equal_weight():
    results = []
    for i in range(5):
        fold = Fold(0, 100, 100, 100, 100, 120)
        results.append(type('obj', (object,), {
            'fold': fold,
            'test_metrics': OOSMetrics(
                total_return=0.1 + i*0.02,
                sharpe=1.0 + i*0.1,
                sortino=1.2,
                calmar=1.5,
                max_drawdown=0.1,
                profit_factor=1.5,
                win_rate=0.55,
                avg_trade_pnl=0.01,
                num_trades=10,
                turnover=0.5,
                volatility=0.15,
                skew=0.0,
                kurtosis=3.0,
                var_95=-0.02,
                cvar_95=-0.03,
            )
        })())
    agg = aggregate_oos(results, weight_by="equal")
    assert "oos_total_return" in agg
    assert "oos_sharpe" in agg
    assert agg["n_folds"] == 5


def test_aggregate_oos_trade_weight():
    results = []
    for i, trades in enumerate([10, 20, 30, 40, 50]):
        fold = Fold(0, 100, 100, 100, 100, 120)
        results.append(type('obj', (object,), {
            'fold': fold,
            'test_metrics': OOSMetrics(
                total_return=0.1,
                sharpe=1.0,
                sortino=1.2,
                calmar=1.5,
                max_drawdown=0.1,
                profit_factor=1.5,
                win_rate=0.55,
                avg_trade_pnl=0.01,
                num_trades=trades,
                turnover=0.5,
                volatility=0.15,
                skew=0.0,
                kurtosis=3.0,
                var_95=-0.02,
                cvar_95=-0.03,
            )
        })())
    agg = aggregate_oos(results, weight_by="trades")
    assert "oos_total_return" in agg


if __name__ == "__main__":
    pytest.main([__file__, "-v"])