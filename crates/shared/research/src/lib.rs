//! research crate documentation.
// QuantRadar research analytics: breadth, relative strength, ranking, correlation/PCA, events and strategy generation.
use quantaradar_core::{FeatureRow, Regime, EventKind, StrategyFamily};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSnapshot { pub symbol: String, pub row: FeatureRow, pub liquidity_score: f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadth { pub count: usize, pub positive_1: f64, pub above_ema20: f64, pub above_ema50: f64, pub above_ema200: f64, pub new_highs: usize, pub new_lows: usize }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeStrength { pub symbol:String, pub vs_benchmark:f64, pub percentile:f64, pub weak_while_market_up:bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedAsset { pub symbol: String, pub score: f64, pub trend: f64, pub momentum: f64, pub value: f64, pub breakout: f64, pub liquidity: f64, pub rationale: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event { pub ts: DateTime<Utc>, pub symbol: String, pub kind: EventKind, pub severity: f64, pub details: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySpec { pub id: String, pub family: StrategyFamily, pub regime: Regime, pub params: std::collections::BTreeMap<String, f64> }

pub fn breadth(items: &[AssetSnapshot]) -> Breadth { let n=items.len().max(1) as f64; Breadth{count:items.len(),positive_1:items.iter().filter(|x|x.row.returns_1.unwrap_or(0.0)>0.0).count() as f64/n,above_ema20:items.iter().filter(|x|x.row.ema_20.map(|v|x.row.close>v).unwrap_or(false)).count() as f64/n,above_ema50:items.iter().filter(|x|x.row.ema_50.map(|v|x.row.close>v).unwrap_or(false)).count() as f64/n,above_ema200:items.iter().filter(|x|x.row.ema_200.map(|v|x.row.close>v).unwrap_or(false)).count() as f64/n,new_highs:items.iter().filter(|x|x.row.new_high_20).count(),new_lows:items.iter().filter(|x|x.row.new_low_20).count()} }

pub fn relative_strength(asset_return:f64, benchmark_return:f64, cross_section:&[f64], symbol:String)->RelativeStrength { let pct=if cross_section.is_empty(){0.5}else{cross_section.iter().filter(|x|**x<=asset_return).count() as f64/cross_section.len() as f64}; RelativeStrength{symbol,vs_benchmark:asset_return-benchmark_return,percentile:pct,weak_while_market_up:benchmark_return>0.0&&asset_return<0.0} }

pub fn rank(items:&[AssetSnapshot],regime:&Regime)->Vec<RankedAsset>{let mut out=items.iter().map(|x|{let trend=match(x.row.ema_20,x.row.ema_50,x.row.ema_200){(Some(a),Some(b),Some(c))=>((x.row.close/a-1.0)+(x.row.close/b-1.0)+(x.row.close/c-1.0))/3.0,_=>0.0};let momentum=0.45*x.row.returns_24.unwrap_or(0.0)+0.35*x.row.returns_72.unwrap_or(0.0)+0.20*x.row.returns_4.unwrap_or(0.0);let value=-x.row.distance_ema20_atr.unwrap_or(0.0).abs();let breakout=if x.row.breakout_20{1.0}else{0.0};let regime_bonus=match regime{Regime::BullTrend|Regime::BullLowVol|Regime::TransitionBull=>1.0,Regime::BearTrend|Regime::BearHighVol|Regime::TransitionBear=>-0.25,_=>0.5};let score=0.35*trend+0.30*momentum+0.10*value+0.15*breakout*regime_bonus+0.10*x.liquidity_score.tanh();let mut rationale=Vec::new();if trend>0.0{rationale.push("multi-EMA trend positive".into());}if momentum>0.0{rationale.push("multi-horizon momentum positive".into());}if breakout>0.0{rationale.push("20-period breakout".into());}if x.liquidity_score>0.7{rationale.push("high executable liquidity".into());}RankedAsset{symbol:x.symbol.clone(),score,trend,momentum,value,breakout,liquidity:x.liquidity_score,rationale}}).collect::<Vec<_>>();out.sort_by(|a,b|b.score.total_cmp(&a.score));out}

pub fn events(x:&AssetSnapshot)->Vec<Event>{let r=&x.row;let mut e=Vec::new();if r.new_high_20{e.push(Event{ts:r.ts,symbol:x.symbol.clone(),kind:EventKind::NewHigh,severity:0.8,details:"Close exceeded prior 20-period high".into()});}if r.new_low_20{e.push(Event{ts:r.ts,symbol:x.symbol.clone(),kind:EventKind::NewLow,severity:0.8,details:"Close fell below prior 20-period low".into()});}if r.volume_z_20.unwrap_or(0.0)>3.0{e.push(Event{ts:r.ts,symbol:x.symbol.clone(),kind:EventKind::VolumeAnomaly,severity:(r.volume_z_20.unwrap_or(0.0)/5.0).min(1.0),details:"Volume z-score exceeded 3".into()});}if r.atr_pct.unwrap_or(0.0)>0.15{e.push(Event{ts:r.ts,symbol:x.symbol.clone(),kind:EventKind::VolatilitySpike,severity:1.0,details:"ATR as percent of price exceeded 15%".into()});}e}

pub fn correlation_matrix(rows:&[Vec<f64>])->Vec<Vec<f64>>{let n=rows.len();let mut c=vec![vec![0.0;n];n];for i in 0..n{for j in i..n{let len=rows[i].len().min(rows[j].len());if len<2{continue;}let mi=rows[i][..len].iter().sum::<f64>()/len as f64;let mj=rows[j][..len].iter().sum::<f64>()/len as f64;let(mut a,mut b,mut d)=(0.0,0.0,0.0);for k in 0..len{let x=rows[i][k]-mi;let y=rows[j][k]-mj;a+=x*y;b+=x*x;d+=y*y;}let v=if b>0.0&&d>0.0{a/(b.sqrt()*d.sqrt())}else{0.0};c[i][j]=v;c[j][i]=v;}}c}
pub fn pca_first_component(rows:&[Vec<f64>],iterations:usize)->Vec<f64>{let n=rows.len();if n==0{return vec![];}let c=correlation_matrix(rows);let mut v=vec![1.0/(n as f64).sqrt();n];for _ in 0..iterations.max(1){let mut nv=vec![0.0;n];for i in 0..n{for j in 0..n{nv[i]+=c[i][j]*v[j];}}let norm=nv.iter().map(|x|x*x).sum::<f64>().sqrt().max(1e-12);for x in &mut nv{*x/=norm;}v=nv;}v}

pub fn generate_strategies(regimes:&[Regime])->Vec<StrategySpec>{let mut out=Vec::new();for r in regimes{let (family,name)=match r{Regime::BullTrend|Regime::BullLowVol|Regime::TransitionBull=>(StrategyFamily::TrendBreakout,"trend_breakout"),Regime::BearTrend|Regime::BearHighVol|Regime::TransitionBear=>(StrategyFamily::DefensiveRelativeStrength,"defensive_relative_strength"),_=>(StrategyFamily::MeanReversion,"mean_reversion")};let mut p=std::collections::BTreeMap::new();p.insert("risk_per_trade".into(),0.005);p.insert("atr_stop".into(),2.0);p.insert("take_profit_atr".into(),3.0);p.insert("min_liquidity".into(),0.65);out.push(StrategySpec{id:format!("{}_{:?}",name,r).to_lowercase(),family,regime:r.clone(),params:p});}out}
