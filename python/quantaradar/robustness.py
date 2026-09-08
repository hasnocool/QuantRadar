# QuantRadar robustness testing and champion/challenger promotion rules.
from __future__ import annotations
from dataclasses import dataclass
from typing import Iterable
import math

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
    if not values: return float("nan")
    xs=sorted(values); k=(len(xs)-1)*p; lo,hi=math.floor(k),math.ceil(k)
    return xs[lo] if lo==hi else xs[lo]+(xs[hi]-xs[lo])*(k-lo)

def perturb_parameters(params: dict[str,float], scales: Iterable[float]=(0.8,0.9,1.0,1.1,1.2)) -> list[dict[str,float]]:
    numeric=[k for k,v in params.items() if isinstance(v,(int,float))]
    out=[]
    for scale in scales:
        out.append({k:(params[k]*scale if k in numeric else params[k]) for k in params})
    return out

def robustness_test(baseline: Performance, variants: list[Performance], min_sharpe: float=0.5, max_dd: float=0.30, min_pass_rate: float=0.60) -> RobustnessResult:
    all_metrics=[baseline,*variants]
    pass_rate=sum(1 for x in all_metrics if x.sharpe>=min_sharpe and abs(x.max_drawdown)<=max_dd)/len(all_metrics)
    robust=pass_rate>=min_pass_rate and min(x.returns for x in all_metrics)>-0.10 and percentile([x.sharpe for x in all_metrics],0.5)>=min_sharpe
    return RobustnessResult(baseline=baseline,variants=tuple(variants),worst_return=min(x.returns for x in all_metrics),median_sharpe=percentile([x.sharpe for x in all_metrics],0.5),max_drawdown=max(abs(x.max_drawdown) for x in all_metrics),pass_rate=pass_rate,robust=robust)

def should_promote(champion: Performance, challenger: Performance, robustness: RobustnessResult, min_improvement_sharpe: float=0.10) -> bool:
    return robustness.robust and challenger.sharpe >= champion.sharpe + min_improvement_sharpe and challenger.max_drawdown <= champion.max_drawdown and challenger.profit_factor >= champion.profit_factor
