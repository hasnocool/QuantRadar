// QuantRadar Kraken REST and WebSocket market-data clients.
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use quantaradar_core::{Bar, MarketId};
use reqwest::Client;
use serde::Deserialize;
use std::{collections::HashMap,time::Duration};
use tokio::time::sleep;
pub mod ws;
#[derive(Debug,Clone)]pub struct KrakenClient{http:Client,base_url:String,min_interval:Duration}
impl Default for KrakenClient{fn default()->Self{Self{http:Client::new(),base_url:"https://api.kraken.com".into(),min_interval:Duration::from_millis(350)}}}
impl KrakenClient{async fn get_json<T:for<'de>Deserialize<'de>>(&self,path:&str)->Result<T>{sleep(self.min_interval).await;Ok(self.http.get(format!("{}{}",self.base_url,path)).send().await?.error_for_status()?.json::<T>().await?)}pub async fn asset_pairs(&self)->Result<Vec<KrakenPair>>{let r:KrakenResponse<HashMap<String,PairRaw>>=self.get_json("/0/public/AssetPairs").await?;Ok(r.result.into_iter().filter_map(|(symbol,p)|{let(base,quote)=(p.base?,p.quote?);Some(KrakenPair{symbol,base,quote,wsname:p.wsname.unwrap_or_default(),status:p.status.unwrap_or_else(||"online".into())})}).collect())}pub async fn ohlc(&self,pair:&str,interval:u32)->Result<Vec<Bar>>{let path=format!("/0/public/OHLC?pair={}&interval={}",urlencoding::encode(pair),interval);let r:KrakenResponse<HashMap<String,serde_json::Value>>=self.get_json(&path).await?;let rows=r.result.get(pair).and_then(|v|v.as_array()).or_else(||r.result.values().find_map(|v|v.as_array())).context("Kraken returned no OHLC rows")?;let mut out=Vec::with_capacity(rows.len());for row in rows{let arr=row.as_array().context("row not an array")?;if arr.len()<8{continue}let ts=arr[0].as_i64().context("invalid timestamp")?;out.push(Bar{ts:DateTime::<Utc>::from_timestamp(ts,0).context("invalid timestamp")?,open:num(&arr[1])?,high:num(&arr[2])?,low:num(&arr[3])?,close:num(&arr[4])?,volume:num(&arr[6])?,trades:arr[7].as_f64()});}Ok(out)}pub async fn discover_spot(&self)->Result<Vec<MarketId>>{Ok(self.asset_pairs().await?.into_iter().filter(|p|p.status=="online").map(|p|MarketId{exchange:"kraken".into(),symbol:if p.wsname.is_empty(){p.symbol}else{p.wsname},base:p.base,quote:p.quote}).collect())}}
#[derive(Debug,Clone)]pub struct KrakenPair{pub symbol:String,pub base:String,pub quote:String,pub wsname:String,pub status:String}
#[derive(Debug,Deserialize)]struct KrakenResponse<T>{result:T}
#[derive(Debug,Deserialize)]struct PairRaw{base:Option<String>,quote:Option<String>,wsname:Option<String>,status:Option<String>}
fn num(v:&serde_json::Value)->Result<f64>{if let Some(s)=v.as_str(){Ok(s.parse()?)}else{Ok(v.as_f64().context("expected numeric value")?)}}
