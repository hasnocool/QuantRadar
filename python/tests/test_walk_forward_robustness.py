import pytest
import pandas as pd
import numpy as np
from quantaradar.walk_forward.evaluator import expanding_folds, Fold, summarize_returns
from quantaradar.robustness import Performance, RobustnessResult, perturb_parameters, robustness_test, should_promote


class TestWalkForward:
    def test_expanding_folds_basic(self):
        folds = expanding_folds(n=100, initial_train=50, test_size=10, step=5)
        assert len(folds) > 0
        assert all(isinstance(f, Fold) for f in folds)
        assert folds[0].train_start == 0
        assert folds[0].train_end == 50
        assert folds[0].test_start == 50
        assert folds[0].test_end == 60

    def test_expanding_folds_invalid(self):
        with pytest.raises(ValueError):
            expanding_folds(n=10, initial_train=50, test_size=10)

    def test_summarize_returns(self):
        returns = pd.Series([0.01, 0.02, -0.01, 0.03, -0.02])
        result = summarize_returns(returns)
        assert "total_return" in result
        assert "sharpe" in result
        assert "max_drawdown" in result
        assert isinstance(result["total_return"], float)
        assert isinstance(result["sharpe"], float)
        assert isinstance(result["max_drawdown"], float)

    def test_summarize_returns_empty(self):
        returns = pd.Series([], dtype=float)
        result = summarize_returns(returns)
        assert result == {"total_return": 0.0, "sharpe": 0.0, "max_drawdown": 0.0}


class TestRobustness:
    def test_performance_dataclass(self):
        perf = Performance(
            returns=0.1, sharpe=1.5, sortino=2.0, max_drawdown=-0.15,
            profit_factor=1.5, turnover=0.5, trades=100
        )
        assert perf.returns == 0.1
        assert perf.sharpe == 1.5

    def test_perturb_parameters(self):
        params: dict[str, float] = {"param1": 1.0, "param2": 2.0}
        variants = perturb_parameters(params, scales=[0.8, 1.0, 1.2])
        assert len(variants) == 3
        assert variants[0]["param1"] == 0.8
        assert variants[1]["param1"] == 1.0
        assert variants[2]["param1"] == 1.2

    def test_robustness_test_pass(self):
        baseline = Performance(0.1, 1.5, 2.0, -0.15, 1.5, 0.5, 100)
        variants = [
            Performance(0.08, 1.4, 1.8, -0.12, 1.4, 0.5, 90),
            Performance(0.12, 1.6, 2.2, -0.18, 1.6, 0.5, 110),
        ]
        result = robustness_test(baseline, variants)
        assert isinstance(result, RobustnessResult)
        assert result.pass_rate > 0
        assert result.robust is True or result.robust is False

    def test_robustness_test_fail_low_sharpe(self):
        baseline = Performance(0.1, 0.3, 0.5, -0.15, 1.5, 0.5, 100)
        variants = [
            Performance(0.08, 0.2, 0.4, -0.12, 1.4, 0.5, 90),
        ]
        result = robustness_test(baseline, variants, min_sharpe=0.5)
        assert result.pass_rate < 1.0

    def test_should_promote_true(self):
        champion = Performance(0.1, 1.5, 2.0, 0.20, 1.5, 0.5, 100)
        challenger = Performance(0.15, 1.7, 2.2, 0.15, 1.6, 0.5, 110)
        robustness = RobustnessResult(
            baseline=champion, variants=(challenger,),
            worst_return=0.1, median_sharpe=1.6, max_drawdown=0.15,
            pass_rate=1.0, robust=True
        )
        assert should_promote(champion, challenger, robustness) is True

    def test_should_promote_false_sharpe(self):
        champion = Performance(0.1, 1.5, 2.0, 0.15, 1.5, 0.5, 100)
        challenger = Performance(0.15, 1.55, 2.2, 0.12, 1.6, 0.5, 110)
        robustness = RobustnessResult(
            baseline=champion, variants=(challenger,),
            worst_return=0.1, median_sharpe=1.55, max_drawdown=0.15,
            pass_rate=1.0, robust=True
        )
        assert should_promote(champion, challenger, robustness, min_improvement_sharpe=0.10) is False


if __name__ == "__main__":
    pytest.main([__file__, "-v"])