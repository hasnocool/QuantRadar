// Kraken WebSocket v2 public market-data ingestion primitives.
use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt};
use quantaradar_core::{OrderBookSnapshot, OrderSide, TradeTick};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Clone)]
pub enum MarketEvent { Book(OrderBookSnapshot), Trade(TradeTick) }

pub async fn stream(pair: &str, depth: u16, out: mpsc::Sender<MarketEvent>) -> Result<()> {
    let (mut ws, _) = connect_async("wss://ws.kraken.com/v2").await.context("connect Kraken websocket")?;
    ws.send(Message::Text(json!({"method":"subscribe","params":{"channel":"book","symbol":[pair],"depth":depth}}).to_string())).await?;
    ws.send(Message::Text(json!({"method":"subscribe","params":{"channel":"trade","symbol":[pair]}}).to_string())).await?;
    while let Some(msg)=ws.next().await { match msg? { Message::Text(text)=>parse_message(&text,&out).await?, Message::Ping(p)=>ws.send(Message::Pong(p)).await?, Message::Close(_)=>break, _=>{} } }
    Ok(())
}

async fn parse_message(text:&str,out:&mpsc::Sender<MarketEvent>)->Result<()> {
    let v:Value=serde_json::from_str(text)?; let channel=v.get("channel").and_then(Value::as_str).unwrap_or(""); let typ=v.get("type").and_then(Value::as_str).unwrap_or("");
    if typ!="update" && typ!="snapshot" { return Ok(()); }
    if channel=="trade" { if let Some(items)=v.get("data").and_then(Value::as_array){for t in items{let side_str=t.get("side").and_then(Value::as_str).unwrap_or(""); let side=match side_str { "buy" => OrderSide::Buy, "sell" => OrderSide::Sell, _ => OrderSide::Buy }; let price=as_f64(t.get("price"))?; let qty=as_f64(t.get("qty"))?; let ts=t.get("timestamp").and_then(Value::as_str).unwrap_or("").parse::<i64>().unwrap_or(0); let _=out.send(MarketEvent::Trade(TradeTick{ts,price,quantity:qty,side})).await;}}}
    if channel=="book" { if let Some(items)=v.get("data").and_then(Value::as_array){for b in items{let bid=level_price(b.get("bid"));let ask=level_price(b.get("ask"));let book=OrderBookSnapshot{ts:0,bid,ask,bid_depth:levels(b.get("bids")),ask_depth:levels(b.get("asks"))};let _=out.send(MarketEvent::Book(book)).await;}}}
    Ok(())
}
fn as_f64(v:Option<&Value>)->Result<f64>{match v{Some(x) if x.is_number()=>x.as_f64().context("numeric value missing"),Some(x) if x.is_string()=>x.as_str().context("string")?.parse().context("invalid numeric"),_=>anyhow::bail!("missing numeric")}}
fn level_price(v:Option<&Value>)->f64{v.and_then(|x|if x.is_object(){x.get("price").and_then(|p|p.as_f64())}else{x.as_f64()}).unwrap_or(0.0)}
fn levels(v:Option<&Value>)->Vec<(f64,f64)>{v.and_then(Value::as_array).map(|a|a.iter().filter_map(|x|{let p=level_price(Some(x));let q=x.get("qty").and_then(|v|v.as_f64()).or_else(||x.get("quantity").and_then(|v|v.as_f64())).unwrap_or(0.0);(p>0.0&&q>0.0).then_some((p,q))}).collect()).unwrap_or_default()}
