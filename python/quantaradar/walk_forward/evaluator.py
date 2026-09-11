# QuantRadar walk-forward evaluation with regime segmentation and full OOS metrics.
from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum
from typing import Callable, Optional
import numpy as np
import pandas as pd


class FoldType(Enum):
    EXPANDING = "expanding"
    ANCHORED = "anchored"
    ROLLING = "rolling"


@dataclass(frozen=True)
class Fold:
    train_start: int
    train_end: int
    validate_start: int
    validate_end: int
    test_start: int
    test_end: int
    regime: Optional[str] = None


@dataclass(frozen=True)
class OOSMetrics:
    total_return: float
    sharpe: float
    sortino: float
    calmar: float
    max_drawdown: float
    profit_factor: float
    win_rate: float
    avg_trade_pnl: float
    num_trades: int
    turnover: float
    volatility: float
    skew: float
    kurtosis: float
    var_95: float
    cvar_95: float

    def to_dict(self) -> dict[str, float]:
        return {
            "total_return": self.total_return,
            "sharpe": self.sharpe,
            "sortino": self.sortino,
            "calmar": self.calmar,
            "max_drawdown": self.max_drawdown,
            "profit_factor": self.profit_factor,
            "win_rate": self.win_rate,
            "avg_trade_pnl": self.avg_trade_pnl,
            "num_trades": self.num_trades,
            "turnover": self.turnover,
            "volatility": self.volatility,
            "skew": self.skew,
            "kurtosis": self.kurtosis,
            "var_95": self.var_95,
            "cvar_95": self.cvar_95,
        }


@dataclass(frozen=True)
class FoldResult:
    fold: Fold
    train_metrics: OOSMetrics
    validate_metrics: OOSMetrics
    test_metrics: OOSMetrics
    regime: Optional[str]


@dataclass
class WalkForwardConfig:
    n: int
    initial_train: int
    test_size: int
    validate_size: int = 0
    step: Optional[int] = None
    fold_type: FoldType = FoldType.EXPANDING
    min_train_size: int = 100
    regimes: Optional[np.ndarray] = None
    regime_labels: Optional[list[str]] = None


def expanding_folds(n: int, initial_train: int, test_size: int, step: Optional[int] = None) -> list[Fold]:
    if initial_train <= 0 or test_size <= 0 or initial_train + test_size > n:
        raise ValueError("invalid fold geometry")
    step = step or test_size
    folds = []
    train_end = initial_train
    while train_end + test_size <= n:
        folds.append(Fold(0, train_end, train_end, train_end, train_end, train_end + test_size))
        train_end += step
    return folds


def anchored_folds(n: int, initial_train: int, test_size: int, step: Optional[int] = None) -> list[Fold]:
    if initial_train <= 0 or test_size <= 0 or initial_train + test_size > n:
        raise ValueError("invalid fold geometry")
    step = step or test_size
    folds = []
    train_end = initial_train
    while train_end + test_size <= n:
        folds.append(Fold(0, train_end, train_end, train_end, train_end, train_end + test_size))
        train_end += step
    return folds


def rolling_folds(n: int, train_size: int, test_size: int, step: Optional[int] = None) -> list[Fold]:
    if train_size <= 0 or test_size <= 0 or train_size + test_size > n:
        raise ValueError("invalid fold geometry")
    step = step or test_size
    folds = []
    train_start = 0
    while train_start + train_size + test_size <= n:
        train_end = train_start + train_size
        folds.append(Fold(train_start, train_end, train_end, train_end, train_end, train_end + test_size))
        train_start += step
    return folds


def generate_folds(config: WalkForwardConfig) -> list[Fold]:
    if config.fold_type == FoldType.EXPANDING:
        return expanding_folds(config.n, config.initial_train, config.test_size, config.step)
    elif config.fold_type == FoldType.ANCHORED:
        return anchored_folds(config.n, config.initial_train, config.test_size, config.step)
    elif config.fold_type == FoldType.ROLLING:
        return rolling_folds(config.n, config.initial_train, config.test_size, config.step)
    else:
        raise ValueError(f"unknown fold type: {config.fold_type}")


def assign_regimes_to_folds(folds: list[Fold], regimes: np.ndarray, regime_labels: Optional[list[str]] = None) -> list[Fold]:
    if regimes is None or len(regimes) == 0:
        return folds
    labeled = []
    for fold in folds:
        test_regimes = regimes[fold.test_start:fold.test_end]
        if len(test_regimes) > 0:
            unique, counts = np.unique(test_regimes, return_counts=True)
            dominant = unique[np.argmax(counts)]
            regime_name = regime_labels[dominant] if regime_labels and dominant < len(regime_labels) else str(dominant)
            labeled.append(Fold(
                fold.train_start, fold.train_end,
                fold.validate_start, fold.validate_end,
                fold.test_start, fold.test_end,
                regime=regime_name
            ))
        else:
            labeled.append(fold)
    return labeled


def compute_metrics(returns: pd.Series, trades: Optional[pd.DataFrame] = None, periods_per_year: int = 365) -> OOSMetrics:
    r = pd.to_numeric(returns, errors="coerce").dropna().to_numpy(dtype=float)
    if len(r) == 0:
        return OOSMetrics(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)

    equity = np.cumprod(1 + r)
    peak = np.maximum.accumulate(equity)
    dd = equity / peak - 1.0
    max_drawdown = float(-dd.min())

    total_return = float(equity[-1] - 1)

    vol = np.std(r, ddof=1) if len(r) > 1 else 0.0
    sharpe = float(np.mean(r) / vol * np.sqrt(periods_per_year)) if vol > 0 else 0.0

    downside = r[r < 0]
    downside_vol = np.std(downside, ddof=1) if len(downside) > 1 else 0.0
    sortino = float(np.mean(r) / downside_vol * np.sqrt(periods_per_year)) if downside_vol > 0 else 0.0

    calmar = float(total_return / max_drawdown) if max_drawdown > 0 and total_return != 0 else 0.0

    var_95 = float(np.percentile(r, 5)) if len(r) > 0 else 0.0
    cvar_95 = float(r[r <= var_95].mean()) if len(r[r <= var_95]) > 0 else 0.0

    skew = float(pd.Series(r).skew()) if len(r) > 2 else 0.0
    kurt = float(pd.Series(r).kurtosis()) if len(r) > 3 else 0.0

    trade_metrics = {"profit_factor": 0.0, "win_rate": 0.0, "avg_trade_pnl": 0.0, "num_trades": 0, "turnover": 0.0}
    if trades is not None and len(trades) > 0:
        pnls = trades["pnl"].values if "pnl" in trades.columns else np.array([])
        if len(pnls) > 0:
            gross_profit = pnls[pnls > 0].sum()
            gross_loss = abs(pnls[pnls < 0].sum())
            profit_factor = gross_profit / gross_loss if gross_loss > 0 else float("inf")
            win_rate = (pnls > 0).mean()
            avg_trade_pnl = pnls.mean()
            num_trades = len(pnls)
            trade_metrics = {
                "profit_factor": float(profit_factor),
                "win_rate": float(win_rate),
                "avg_trade_pnl": float(avg_trade_pnl),
                "num_trades": num_trades,
                "turnover": 0.0,
            }

    return OOSMetrics(
        total_return=total_return,
        sharpe=sharpe,
        sortino=sortino,
        calmar=calmar,
        max_drawdown=max_drawdown,
        profit_factor=trade_metrics["profit_factor"],
        win_rate=trade_metrics["win_rate"],
        avg_trade_pnl=trade_metrics["avg_trade_pnl"],
        num_trades=trade_metrics["num_trades"],
        turnover=trade_metrics["turnover"],
        volatility=float(vol * np.sqrt(periods_per_year)),
        skew=skew,
        kurtosis=kurt,
        var_95=var_95,
        cvar_95=cvar_95,
    )


def run_walk_forward(
    returns: pd.Series,
    config: WalkForwardConfig,
    strategy_fn: Callable[[np.ndarray, np.ndarray], np.ndarray],
    trades_by_fold: Optional[list[pd.DataFrame]] = None,
) -> list[FoldResult]:
    folds = generate_folds(config)
    if config.regimes is not None:
        folds = assign_regimes_to_folds(folds, config.regimes, config.regime_labels)

    results = []
    for i, fold in enumerate(folds):
        train_returns = returns.iloc[fold.train_start:fold.train_end]
        test_returns = returns.iloc[fold.test_start:fold.test_end]

        train_signals = strategy_fn(train_returns.values, np.arange(fold.train_start, fold.train_end))
        test_signals = strategy_fn(test_returns.values, np.arange(fold.test_start, fold.test_end))

        train_pnl = train_returns.values * train_signals
        test_pnl = test_returns.values * test_signals

        train_metrics = compute_metrics(pd.Series(train_pnl))
        test_metrics = compute_metrics(pd.Series(test_pnl))

        # Validate segment: must be a distinct OOS slice AFTER the test period
        # to prevent lookahead bias. Reusing test data as validate would leak
        # future information into the validation metrics.
        validate_metrics = OOSMetrics(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        if config.validate_size > 0:
            validate_start_idx = fold.test_end
            validate_end_idx = validate_start_idx + config.validate_size
            if validate_end_idx <= len(returns):
                validate_returns = returns.iloc[validate_start_idx:validate_end_idx]
                validate_signals = strategy_fn(
                    validate_returns.values,
                    np.arange(validate_start_idx, validate_end_idx),
                )
                validate_pnl = validate_returns.values * validate_signals
                validate_metrics = compute_metrics(pd.Series(validate_pnl))
            else:
                # Not enough data for a separate validate period - leave as zero
                # rather than reusing test data (prevents test-train contamination)
                validate_metrics = OOSMetrics(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)

        fold_trades = trades_by_fold[i] if trades_by_fold and i < len(trades_by_fold) else None
        if fold_trades is not None:
            test_metrics = compute_metrics(pd.Series(test_pnl), fold_trades)

        results.append(FoldResult(
            fold=fold,
            train_metrics=train_metrics,
            validate_metrics=validate_metrics,
            test_metrics=test_metrics,
            regime=fold.regime,
        ))

    return results


def aggregate_oos(results: list[FoldResult], weight_by: str = "equal") -> dict[str, float]:
    if not results:
        return {}

    if weight_by == "equal":
        weights = np.ones(len(results)) / len(results)
    elif weight_by == "trades":
        trades = np.array([r.test_metrics.num_trades for r in results])
        weights = trades / trades.sum() if trades.sum() > 0 else np.ones(len(results)) / len(results)
    else:
        weights = np.ones(len(results)) / len(results)

    metrics_dict = {}
    for key in [
        "total_return", "sharpe", "sortino", "calmar", "max_drawdown",
        "profit_factor", "win_rate", "avg_trade_pnl", "num_trades",
        "turnover", "volatility", "skew", "kurtosis", "var_95", "cvar_95"
    ]:
        values = np.array([getattr(r.test_metrics, key) for r in results])
        if key in ("profit_factor",):
            values = np.where(np.isinf(values), np.nan, values)
            valid = ~np.isnan(values)
            if valid.any():
                metrics_dict[f"oos_{key}"] = float(np.average(values[valid], weights=weights[valid]))
            else:
                metrics_dict[f"oos_{key}"] = 0.0
        else:
            metrics_dict[f"oos_{key}"] = float(np.average(values, weights=weights))

    metrics_dict["n_folds"] = len(results)
    return metrics_dict