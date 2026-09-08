// Explainable multi-family market screeners.
use chrono::Utc;
use quantaradar_core::{Direction, FeatureRow, Regime, Signal};
use std::collections::BTreeMap;
use uuid::Uuid;

pub trait Screener: Send + Sync { fn name(&self)->&'static str; fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>; }

fn make(symbol:&str,family:&str,direction:Direction,score:f64,regime:Regime,r:&FeatureRow,why:Vec<String>)->Signal{
    let mut features=BTreeMap::new();
    for (k,v) in [("r1",r.returns_1),("r24",r.returns_24),("r72",r.returns_72),("rsi",r.rsi_14),("atr_pct",r.atr_pct),("bb_width",r.bb_width_20),("volume_z",r.volume_z_20),("ema_distance",r.distance_ema20_atr)]{
        if let Some(x)=v { features.insert(k.to_string(),x); }
    }
    Signal{id:Uuid::new_v4(),ts:Utc::now(),symbol:symbol.into(),family:family.into(),direction,score:score.clamp(0.0,1.0),regime,rationale:why,features}
}

pub struct TrendScreener;impl Screener for TrendScreener{fn name(&self)->&'static str{"trend"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{let Some(r)=rows.last()else{return vec![]};let ok=r.ema_20.zip(r.ema_50).zip(r.ema_200).map(|((a,b),c)|a>b&&b>c).unwrap_or(false);let mom=r.returns_24.unwrap_or(0.0);if ok&&mom>0.0{vec![make(&r.symbol,self.name(),Direction::Long,0.65+mom.min(0.25),regime,r,vec!["EMA20 > EMA50 > EMA200".into(),"Positive 24-period momentum".into()])]}else{vec![]}}}

pub struct BreakoutScreener;impl Screener for BreakoutScreener{fn name(&self)->&'static str{"breakout"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{let Some(r)=rows.last()else{return vec![]};let vz=r.volume_z_20.unwrap_or(0.0);if r.breakout_20&&vz>=0.5{vec![make(&r.symbol,self.name(),Direction::Long,(0.68+vz*0.07).min(0.98),regime,r,vec!["20-period closing breakout".into(),format!("Volume z-score {:.2}",vz)])]}else{vec![]}}}

pub struct MeanReversionScreener;impl Screener for MeanReversionScreener{fn name(&self)->&'static str{"mean_reversion"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{let Some(r)=rows.last()else{return vec![]};let rsi=r.rsi_14.unwrap_or(50.0);let d=r.distance_ema20_atr.unwrap_or(0.0);if rsi<30.0&&d< -1.5{vec![make(&r.symbol,self.name(),Direction::Long,0.78,regime,r,vec![format!("RSI {:.1}",rsi),format!("EMA20 distance {:.2} ATR",d)])]}else{vec![]}}}

pub struct VolatilityExpansionScreener;impl Screener for VolatilityExpansionScreener{fn name(&self)->&'static str{"volatility_expansion"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{if rows.len()<2{return vec![]};let r=&rows[rows.len()-1];let p=&rows[rows.len()-2];let w=r.bb_width_20.unwrap_or(0.0);let pw=p.bb_width_20.unwrap_or(w);let vz=r.volume_z_20.unwrap_or(0.0);if pw>0.0&&w>pw*1.2&&vz>1.0&&r.returns_1.unwrap_or(0.0)>0.0{vec![make(&r.symbol,self.name(),Direction::Long,0.82,regime,r,vec!["Bollinger width expanding".into(),"Volume expansion".into()])]}else{vec![]}}}

pub struct VolumeSurgeScreener;impl Screener for VolumeSurgeScreener{fn name(&self)->&'static str{"volume_surge"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{let Some(r)=rows.last()else{return vec![]};let vz=r.volume_z_20.unwrap_or(0.0);let mom=r.returns_1.unwrap_or(0.0);if vz>=2.0&&mom>0.0{vec![make(&r.symbol,self.name(),Direction::Long,(0.70+vz*0.05).min(0.95),regime,r,vec![format!("Volume z-score {:.2}",vz),"Positive 1-period momentum".into()])]}else{vec![]}}}

pub struct MomentumDivergenceScreener;impl Screener for MomentumDivergenceScreener{fn name(&self)->&'static str{"momentum_divergence"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{if rows.len()<24{return vec![]};let r=&rows[rows.len()-1];let rsi=r.rsi_14.unwrap_or(50.0);let price_24=r.returns_24.unwrap_or(0.0);let price_4=r.returns_4.unwrap_or(0.0);let bull_div=rsi<40.0&&price_4>0.0&&price_24<0.0;let bear_div=rsi>60.0&&price_4<0.0&&price_24>0.0;if bull_div{vec![make(&r.symbol,self.name(),Direction::Long,0.75,regime,r,vec![format!("RSI {:.1} bull divergence",rsi),"Short-term momentum positive vs 24-period decline".into()])]}else if bear_div{vec![make(&r.symbol,self.name(),Direction::Short,0.75,regime,r,vec![format!("RSI {:.1} bear divergence",rsi),"Short-term momentum negative vs 24-period rise".into()])]}else{vec![]}}}

pub struct SupportResistanceBounceScreener;impl Screener for SupportResistanceBounceScreener{fn name(&self)->&'static str{"support_resistance_bounce"}fn evaluate(&self,rows:&[FeatureRow],regime:Regime)->Vec<Signal>{if rows.len()<20{return vec![]};let r=&rows[rows.len()-1];let dist=r.distance_ema20_atr.unwrap_or(0.0);let rsi=r.rsi_14.unwrap_or(50.0);let atr_pct=r.atr_pct.unwrap_or(0.0);let near_support=dist< -1.0&&rsi<40.0;let near_resistance=dist>1.0&&rsi>60.0;if near_support&&atr_pct>0.0{vec![make(&r.symbol,self.name(),Direction::Long,0.70,regime,r,vec![format!("EMA20 distance {:.2} ATR",dist),"RSI {:.1} oversold".into(),"Near support with volatility".into()])]}else if near_resistance&&atr_pct>0.0{vec![make(&r.symbol,self.name(),Direction::Short,0.70,regime,r,vec![format!("EMA20 distance {:.2} ATR",dist),"RSI {:.1} overbought".into(),"Near resistance with volatility".into()])]}else{vec![]}}}

pub fn run_default(rows:&[FeatureRow],regime:Regime)->Vec<Signal>{
    [Box::new(TrendScreener)as Box<dyn Screener>,Box::new(BreakoutScreener),Box::new(MeanReversionScreener),Box::new(VolatilityExpansionScreener),Box::new(VolumeSurgeScreener),Box::new(MomentumDivergenceScreener),Box::new(SupportResistanceBounceScreener)].into_iter().flat_map(|s|s.evaluate(rows,regime.clone())).collect()
}
