use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use crate::OnChainEvent;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct OnChainRpcClient {
    client: Client,
    rpc_url: String,
}

impl OnChainRpcClient {
    pub fn new(rpc_url: String) -> Self {
        Self { client: Client::new(), rpc_url }
    }

    pub async fn get_latest_block(&self) -> Result<u64> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });
        let resp: serde_json::Value = self.client.post(&self.rpc_url).json(&payload).send().await?.json().await?;
        let hex = resp.get("result").and_then(|v| v.as_str()).unwrap_or("0x0");
        let num = u64::from_str_radix(hex.trim_start_matches("0x"), 16)?;
        Ok(num)
    }

    pub async fn get_logs(&self, address: &str, from_block: u64, to_block: u64) -> Result<Vec<OnChainEvent>> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "eth_getLogs",
            "params": [{
                "address": address,
                "fromBlock": format!("0x{:x}", from_block),
                "toBlock": format!("0x{:x}", to_block),
            }],
            "id": 1
        });
        let resp: serde_json::Value = self.client.post(&self.rpc_url).json(&payload).send().await?.json().await?;
        let logs_arr = resp.get("result").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let mut events = Vec::new();
        for log in logs_arr {
            let tx_hash = log.get("transactionHash").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let block = log.get("blockNumber").and_then(|v| v.as_str()).unwrap_or("0x0");
            let block_num = u64::from_str_radix(block.trim_start_matches("0x"), 16).unwrap_or(0);
            events.push(OnChainEvent {
                tx_hash,
                block_number: block_num,
                timestamp: Utc::now(),
                event_type: "log".into(),
                address: address.into(),
                value: None,
                raw: log.clone(),
            });
        }
        Ok(events)
    }
}
