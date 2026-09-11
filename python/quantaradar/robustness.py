# QuantRadar robustness testing, multiple-test protection, and champion/challenger promotion rules.
from __future__ import annotations
from dataclasses import dataclass
from typing import Iterable
import math
import numpy as np
import pandas as pd
from scipy import stats
from scipy.optimize import brentq


@dataclass(frozen=True)
class Performance:
    returns: float
    sharpe: float
    sortino: float
    max_drawdown: float
    profit_factor: float
    turnover: float
    trades: int


@dataclass(frozen=True)
class RobustnessResult:
    baseline: Performance
    variants: tuple[Performance, ...]
    worst_return: float
    median_sharpe: float
    max_drawdown: float
    pass_rate: float
    robust: bool


def percentile(values: list[float], p: float) -> float:
    if not values:
        return float("nan")
    xs = sorted(values)
    k = (len(xs) - 1) * p
    lo, hi = math.floor(k), math.ceil(k)
    return xs[lo] if lo == hi else xs[lo] + (xs[hi] - xs[lo]) * (k - lo)


def perturb_parameters(
    params: dict[str, float], scales: Iterable[float] = (0.8, 0.9, 1.0, 1.1, 1.2)
) -> list[dict[str, float]]:
    numeric = [k for k, v in params.items() if isinstance(v, (int, float))]
    out = []
    for scale in scales:
        out.append({k: (params[k] * scale if k in numeric else params[k]) for k in params})
    return out


def robustness_test(
    baseline: Performance,
    variants: list[Performance],
    min_sharpe: float = 0.5,
    max_dd: float = 0.30,
    min_pass_rate: float = 0.60,
) -> RobustnessResult:
    all_metrics = [baseline, *variants]
    pass_rate = sum(
        1 for x in all_metrics if x.sharpe >= min_sharpe and abs(x.max_drawdown) <= max_dd
    ) / len(all_metrics)
    robust = (
        pass_rate >= min_pass_rate
        and min(x.returns for x in all_metrics) > -0.10
        and percentile([x.sharpe for x in all_metrics], 0.5) >= min_sharpe
    )
    return RobustnessResult(
        baseline=baseline,
        variants=tuple(variants),
        worst_return=min(x.returns for x in all_metrics),
        median_sharpe=percentile([x.sharpe for x in all_metrics], 0.5),
        max_drawdown=max(abs(x.max_drawdown) for x in all_metrics),
        pass_rate=pass_rate,
        robust=robust,
    )


def should_promote(
    champion: Performance,
    challenger: Performance,
    robustness: RobustnessResult,
    min_improvement_sharpe: float = 0.10,
) -> bool:
    return (
        robustness.robust
        and challenger.sharpe >= champion.sharpe + min_improvement_sharpe
        and challenger.max_drawdown <= champion.max_drawdown
        and challenger.profit_factor >= champion.profit_factor
    )


# =============================================================================
# Multiple-Test Protection: Deflated Sharpe, PBO, White's Reality Check, Hansen SPA
# =============================================================================


def deflated_sharpe_ratio(
    sharpe: float,
    n_trials: int,
    n_obs: int,
    autocorr: float = 0.0,
) -> float:
    """
    Compute Deflated Sharpe Ratio (Bailey & Lopez de Prado, 2014).

    Adjusts the observed Sharpe ratio for multiple testing and autocorrelation.

    Args:
        sharpe: Observed Sharpe ratio
        n_trials: Number of independent trials/strategies tested
        n_obs: Number of observations (returns)
        autocorr: First-order autocorrelation of returns

    Returns:
        Deflated Sharpe ratio (lower than observed if multiple testing)
    """
    if n_obs <= 1 or n_trials <= 1:
        return sharpe

    # Expected maximum of n_trials standard normal variables
    # Using extreme value theory approximation
    gamma_em = 0.5772156649  # Euler-Mascheroni constant
    expected_max = (2 * math.log(n_trials)) ** 0.5 - (gamma_em + math.log(math.log(n_trials))) / (2 * (2 * math.log(n_trials)) ** 0.5)

    # Adjust for autocorrelation
    if autocorr != 0:
        n_eff = n_obs * (1 - autocorr) / (1 + autocorr)
    else:
        n_eff = n_obs

    # Standard error of Sharpe ratio
    se = math.sqrt((1 + 0.5 * sharpe**2) / n_eff)

    # Deflated Sharpe: observed - expected_max * se
    deflated = sharpe - expected_max * se

    return max(deflated, 0.0)


def probabilistic_backtest_overfitting(
    sharpe_obs: float,
    sharpe_trials: list[float],
    n_obs: int,
    autocorr: float = 0.0,
) -> float:
    """
    Probabilistic Backtest Overfitting (PBO) - Bailey et al. (2014).

    Computes the probability that the best observed Sharpe is due to chance.

    Args:
        sharpe_obs: Observed Sharpe of the selected strategy
        sharpe_trials: List of Sharpe ratios from all trials
        n_obs: Number of observations
        autocorr: Autocorrelation of returns

    Returns:
        PBO value in [0, 1] - probability of overfitting
    """
    if not sharpe_trials:
        return 0.0

    # Rank of observed Sharpe among all trials (including ties at top)
    rank = sum(1 for s in sharpe_trials if s >= sharpe_obs)
    pbo = rank / len(sharpe_trials)

    # Adjust for autocorrelation
    if autocorr != 0:
        n_eff = n_obs * (1 - autocorr) / (1 + autocorr)
        # Higher autocorrelation -> fewer effective trials -> lower PBO
        pbo = pbo * (n_eff / n_obs)

    return min(max(pbo, 0.0), 1.0)


def whites_reality_check(
    returns_matrix: np.ndarray,
    benchmark_returns: np.ndarray,
    n_bootstrap: int = 1000,
) -> dict[str, float]:
    """
    White's Reality Check (2000) for data snooping.

    Tests whether the best strategy's performance is statistically significant
    after accounting for the search over multiple strategies.

    Args:
        returns_matrix: Shape (n_strategies, n_obs) - returns of each strategy
        benchmark_returns: Shape (n_obs,) - benchmark returns
        n_bootstrap: Number of bootstrap iterations

    Returns:
        Dict with p-value and test statistics
    """
    n_strategies, n_obs = returns_matrix.shape

    # Excess returns over benchmark
    excess = returns_matrix - benchmark_returns.reshape(1, -1)

    # Mean excess returns for each strategy
    mean_excess = excess.mean(axis=1)
    best_idx = np.argmax(mean_excess)
    best_mean = mean_excess[best_idx]

    # Stationary bootstrap (Politis & Romano, 1994)
    # Block length parameter
    block_len = int(n_obs ** (1 / 3)) or 1

    bootstrapped_max = np.zeros(n_bootstrap)
    for b in range(n_bootstrap):
        # Generate bootstrap sample using stationary bootstrap
        idx = np.zeros(n_obs, dtype=int)
        i = 0
        while i < n_obs:
            if i == 0 or np.random.random() > 1.0 / block_len:
                idx[i] = np.random.randint(0, n_obs)
            else:
                idx[i] = (idx[i - 1] + 1) % n_obs
            i += 1

        boot_excess = excess[:, idx]
        boot_means = boot_excess.mean(axis=1)
        bootstrapped_max[b] = boot_means.max()

    # p-value: proportion of bootstrap max >= observed best
    p_value = (bootstrapped_max >= best_mean).mean()

    return {
        "p_value": float(p_value),
        "best_sharpe": float(best_mean / returns_matrix[best_idx].std() * np.sqrt(252)),
        "n_strategies": n_strategies,
        "n_obs": n_obs,
    }


def hansens_spa_test(
    returns_matrix: np.ndarray,
    benchmark_returns: np.ndarray,
    n_bootstrap: int = 1000,
) -> dict[str, float]:
    """
    Hansen's Superior Predictive Ability (SPA) Test (2005).

    Tests whether any strategy significantly outperforms the benchmark,
    using a studentized test statistic for better power.

    Args:
        returns_matrix: Shape (n_strategies, n_obs) - returns of each strategy
        benchmark_returns: Shape (n_obs,) - benchmark returns
        n_bootstrap: Number of bootstrap iterations

    Returns:
        Dict with p-value and test statistics
    """
    n_strategies, n_obs = returns_matrix.shape

    excess = returns_matrix - benchmark_returns.reshape(1, -1)
    mean_excess = excess.mean(axis=1)
    std_excess = excess.std(axis=1, ddof=1)

    # Studentized statistics
    t_stats = mean_excess / (std_excess / np.sqrt(n_obs))
    best_idx = np.argmax(t_stats)
    best_t = t_stats[best_idx]

    # Stationary bootstrap for SPA
    block_len = int(n_obs ** (1 / 3)) or 1

    bootstrapped_max = np.zeros(n_bootstrap)
    for b in range(n_bootstrap):
        idx = np.zeros(n_obs, dtype=int)
        i = 0
        while i < n_obs:
            if i == 0 or np.random.random() > 1.0 / block_len:
                idx[i] = np.random.randint(0, n_obs)
            else:
                idx[i] = (idx[i - 1] + 1) % n_obs
            i += 1

        boot_excess = excess[:, idx]
        boot_means = boot_excess.mean(axis=1)
        boot_stds = boot_excess.std(axis=1, ddof=1)
        boot_t = boot_means / (boot_stds / np.sqrt(n_obs))
        bootstrapped_max[b] = boot_t.max()

    p_value = (bootstrapped_max >= best_t).mean()

    return {
        "p_value": float(p_value),
        "best_t_stat": float(best_t),
        "best_strategy_idx": int(best_idx),
        "n_strategies": n_strategies,
        "n_obs": n_obs,
    }


def deflated_sharpe_from_returns(
    returns: np.ndarray,
    n_trials: int,
) -> tuple[float, float, float]:
    """
    Convenience function to compute Deflated Sharpe from returns array.

    Args:
        returns: Strategy returns
        n_trials: Number of trials/strategies tested

    Returns:
        (observed_sharpe, deflated_sharpe, pbo)
    """
    n_obs = len(returns)
    if n_obs < 2:
        return 0.0, 0.0, 0.0

    mean_ret = returns.mean()
    std_ret = returns.std(ddof=1)
    if std_ret == 0:
        return 0.0, 0.0, 0.0

    sharpe = mean_ret / std_ret * np.sqrt(252)

    # Autocorrelation
    if n_obs > 2:
        autocorr = np.corrcoef(returns[:-1], returns[1:])[0, 1]
        if np.isnan(autocorr):
            autocorr = 0.0
    else:
        autocorr = 0.0

    deflated = deflated_sharpe_ratio(sharpe, n_trials, n_obs, autocorr)

    # PBO
    # Simulate trials under null (random strategies with same stats)
    trial_sharpes = np.random.normal(0, 1/np.sqrt(n_obs), n_trials) * np.sqrt(252)
    pbo = probabilistic_backtest_overfitting(sharpe / np.sqrt(252), trial_sharpes.tolist(), n_obs, autocorr)

    return float(sharpe), float(deflated), float(pbo)


@dataclass(frozen=True)
class MultipleTestResult:
    deflated_sharpe: float
    pbo: float
    wrc_p_value: float
    spa_p_value: float
    observed_sharpe: float
    n_trials: int
    n_obs: int


def multiple_test_protection(
    returns_matrix: np.ndarray,
    benchmark_returns: np.ndarray,
    n_trials: int,
    n_bootstrap: int = 1000,
) -> MultipleTestResult:
    """
    Run all multiple-test protection diagnostics.

    Args:
        returns_matrix: Shape (n_strategies, n_obs) - all tested strategies
        benchmark_returns: Shape (n_obs,) - benchmark returns
        n_trials: Number of strategies tested (for Deflated Sharpe/PBO)
        n_bootstrap: Bootstrap iterations for WRC/SPA

    Returns:
        MultipleTestResult with all diagnostics
    """
    n_strategies, n_obs = returns_matrix.shape

    # Observed best strategy
    mean_rets = returns_matrix.mean(axis=1)
    std_rets = returns_matrix.std(axis=1, ddof=1)
    sharpes = mean_rets / std_rets * np.sqrt(252)
    best_idx = np.argmax(sharpes)
    observed_sharpe = sharpes[best_idx]

    # Deflated Sharpe
    autocorr = np.corrcoef(returns_matrix[best_idx, :-1], returns_matrix[best_idx, 1:])[0, 1] if n_obs > 2 else 0.0
    if np.isnan(autocorr):
        autocorr = 0.0
    deflated = deflated_sharpe_ratio(observed_sharpe, n_trials, n_obs, autocorr)

    # PBO
    pbo = probabilistic_backtest_overfitting(observed_sharpe / np.sqrt(252), (sharpes / np.sqrt(252)).tolist(), n_obs, autocorr)

    # White's Reality Check
    wrc = whites_reality_check(returns_matrix, benchmark_returns, n_bootstrap)
    wrc_p = wrc["p_value"]

    # Hansen SPA
    spa = hansens_spa_test(returns_matrix, benchmark_returns, n_bootstrap)
    spa_p = spa["p_value"]

    return MultipleTestResult(
        deflated_sharpe=deflated,
        pbo=pbo,
        wrc_p_value=wrc_p,
        spa_p_value=spa_p,
        observed_sharpe=float(observed_sharpe),
        n_trials=n_trials,
        n_obs=n_obs,
    )