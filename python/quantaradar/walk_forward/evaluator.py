# QuantRadar expanding walk-forward evaluation primitives.
from __future__ import annotations
from dataclasses import dataclass
import numpy as np
import pandas as pd
@dataclass(frozen=True)
class Fold:
    train_start:int; train_end:int; test_start:int; test_end:int
def expanding_folds(n:int,initial_train:int,test_size:int,step:int|None=None)->list[Fold]:
    if initial_train<=0 or test_size<=0 or initial_train+test_size>n: raise ValueError("invalid fold geometry")
    step=step or test_size; folds=[]; train_end=initial_train
    while train_end+test_size<=n:
        folds.append(Fold(0,train_end,train_end,train_end+test_size)); train_end+=step
    return folds
def summarize_returns(returns:pd.Series,periods_per_year:int=365)->dict[str,float]:
    r=pd.to_numeric(returns,errors="coerce").dropna().to_numpy(dtype=float)
    if len(r)==0:return {"total_return":0.0,"sharpe":0.0,"max_drawdown":0.0}
    equity=np.cumprod(1+r); peak=np.maximum.accumulate(equity); dd=equity/peak-1.0
    vol=np.std(r,ddof=1) if len(r)>1 else 0.0; sharpe=float(np.mean(r)/vol*np.sqrt(periods_per_year)) if vol>0 else 0.0
    return {"total_return":float(equity[-1]-1),"sharpe":sharpe,"max_drawdown":float(-dd.min())}
