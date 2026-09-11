# Tests for multiple-test protection (Deflated Sharpe, PBO, White's Reality Check, Hansen SPA)
import numpy as np
import pandas as pd
import pytest

from quantaradar.robustness import (
    deflated_sharpe_ratio,
    probabilistic_backtest_overfitting,
    whites_reality_check,
    hansens_spa_test,
    deflated_sharpe_from_returns,
    multiple_test_protection,
    MultipleTestResult,
    Performance,
    RobustnessResult,
    robustness_test,
    should_promote,
)


def test_deflated_sharpe_ratio_basic():
    # Single trial should return observed Sharpe
    ds = deflated_sharpe_ratio(sharpe=1.5, n_trials=1, n_obs=252)
    assert ds == 1.5

    # Multiple trials should deflate
    ds = deflated_sharpe_ratio(sharpe=2.0, n_trials=100, n_obs=252)
    assert ds < 2.0
    assert ds >= 0.0


def test_deflated_sharpe_with_autocorr():
    ds_no_ac = deflated_sharpe_ratio(sharpe=1.5, n_trials=50, n_obs=252, autocorr=0.0)
    ds_with_ac = deflated_sharpe_ratio(sharpe=1.5, n_trials=50, n_obs=252, autocorr=0.3)
    # Positive autocorrelation reduces effective sample size -> more deflation
    assert ds_with_ac <= ds_no_ac


def test_pbo_basic():
    # PBO with single trial
    pbo = probabilistic_backtest_overfitting(1.5, [1.5], 252)
    assert pbo == 1.0

    # PBO with multiple trials where observed is best
    pbo = probabilistic_backtest_overfitting(2.0, [0.5, 1.0, 1.5, 2.0], 252)
    assert pbo == 0.25  # 1 out of 4


def test_pbo_with_autocorr():
    pbo_no_ac = probabilistic_backtest_overfitting(1.5, [0.5, 1.0, 1.5, 2.0], 252, autocorr=0.0)
    pbo_with_ac = probabilistic_backtest_overfitting(1.5, [0.5, 1.0, 1.5, 2.0], 252, autocorr=0.3)
    # Autocorrelation reduces effective trials
    assert pbo_with_ac <= pbo_no_ac


def test_whites_reality_check():
    np.random.seed(42)
    n_strategies = 10
    n_obs = 252
    returns_matrix = np.random.normal(0.0005, 0.01, (n_strategies, n_obs))
    benchmark = np.random.normal(0.0003, 0.01, n_obs)

    result = whites_reality_check(returns_matrix, benchmark, n_bootstrap=100)
    assert "p_value" in result
    assert 0.0 <= result["p_value"] <= 1.0
    assert result["n_strategies"] == n_strategies
    assert result["n_obs"] == n_obs


def test_hansens_spa_test():
    np.random.seed(42)
    n_strategies = 10
    n_obs = 252
    returns_matrix = np.random.normal(0.0005, 0.01, (n_strategies, n_obs))
    benchmark = np.random.normal(0.0003, 0.01, n_obs)

    result = hansens_spa_test(returns_matrix, benchmark, n_bootstrap=100)
    assert "p_value" in result
    assert 0.0 <= result["p_value"] <= 1.0
    assert result["n_strategies"] == n_strategies
    assert result["n_obs"] == n_obs
    assert "best_t_stat" in result


def test_deflated_sharpe_from_returns():
    np.random.seed(42)
    returns = np.random.normal(0.001, 0.01, 252)
    sharpe, deflated, pbo = deflated_sharpe_from_returns(returns, n_trials=100)
    assert sharpe >= 0
    assert deflated >= 0
    assert deflated <= sharpe
    assert 0.0 <= pbo <= 1.0


def test_multiple_test_protection():
    np.random.seed(42)
    n_strategies = 20
    n_obs = 500
    returns_matrix = np.random.normal(0.0005, 0.01, (n_strategies, n_obs))
    benchmark = np.random.normal(0.0003, 0.01, n_obs)

    result = multiple_test_protection(returns_matrix, benchmark, n_trials=n_strategies, n_bootstrap=200)
    assert isinstance(result, MultipleTestResult)
    assert result.n_trials == n_strategies
    assert result.n_obs == n_obs
    assert result.observed_sharpe >= 0
    assert result.deflated_sharpe >= 0
    assert result.deflated_sharpe <= result.observed_sharpe
    assert 0.0 <= result.pbo <= 1.0
    assert 0.0 <= result.wrc_p_value <= 1.0
    assert 0.0 <= result.spa_p_value <= 1.0


def test_robustness_test():
    baseline = Performance(returns=0.1, sharpe=1.2, sortino=1.5, max_drawdown=0.15, profit_factor=1.8, turnover=0.5, trades=50)
    variants = [
        Performance(returns=0.08, sharpe=1.0, sortino=1.3, max_drawdown=0.12, profit_factor=1.6, turnover=0.6, trades=45),
        Performance(returns=0.12, sharpe=1.4, sortino=1.6, max_drawdown=0.10, profit_factor=2.0, turnover=0.4, trades=55),
    ]
    result = robustness_test(baseline, variants, min_sharpe=0.8, max_dd=0.20)
    assert isinstance(result, RobustnessResult)
    assert result.baseline == baseline
    assert len(result.variants) == 2
    assert 0.0 <= result.pass_rate <= 1.0


def test_should_promote():
    champion = Performance(returns=0.1, sharpe=1.2, sortino=1.5, max_drawdown=0.15, profit_factor=1.8, turnover=0.5, trades=50)
    challenger = Performance(returns=0.15, sharpe=1.5, sortino=1.8, max_drawdown=0.10, profit_factor=2.0, turnover=0.4, trades=60)
    variants = [
        Performance(returns=0.08, sharpe=1.0, sortino=1.3, max_drawdown=0.12, profit_factor=1.6, turnover=0.6, trades=45),
    ]
    robustness = robustness_test(champion, variants)
    assert should_promote(champion, challenger, robustness, min_improvement_sharpe=0.1)

    # Challenger not better enough
    challenger2 = Performance(returns=0.11, sharpe=1.25, sortino=1.5, max_drawdown=0.14, profit_factor=1.8, turnover=0.5, trades=52)
    assert not should_promote(champion, challenger2, robustness, min_improvement_sharpe=0.1)


def test_performance_dataclass():
    perf = Performance(returns=0.1, sharpe=1.5, sortino=1.8, max_drawdown=0.12, profit_factor=2.0, turnover=0.4, trades=50)
    assert perf.sharpe == 1.5
    assert perf.returns == 0.1


if __name__ == "__main__":
    pytest.main([__file__, "-v"])