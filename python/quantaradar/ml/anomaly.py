# QuantRadar unsupervised anomaly scoring.
from __future__ import annotations
import pandas as pd
from sklearn.ensemble import IsolationForest
def isolation_scores(frame:pd.DataFrame,columns:list[str],contamination:float=0.02)->pd.Series:
    x=frame[columns].replace([float("inf"),float("-inf")],pd.NA).dropna()
    if x.empty:return pd.Series(index=frame.index,dtype=float)
    model=IsolationForest(n_estimators=300,contamination=contamination,random_state=42,n_jobs=-1)
    raw=model.decision_function(x);out=pd.Series(index=frame.index,dtype=float);out.loc[x.index]=-raw
    return out.fillna(0.0)
