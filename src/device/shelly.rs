use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize, Serialize)]
pub struct ShellyStatus {
    pub total_current: f64,
    pub total_act_power: f64,
    pub total_aprt_power: f64,
}

pub struct ShellyClient {
    client: Client,
    base_url: String,
}

impl ShellyClient {
    pub fn new(ip: &std::net::IpAddr) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://{}", ip),
        }
    }
   pub async fn test_shelly(ip: &str) -> bool {
        let url = format!("http://{}/rpc/Shelly.GetStatus", ip);

        let resp = reqwest::get(&url).await;

        if resp.is_err().clone() {
            return false;
        }

        let status: serde_json::Value = resp.unwrap().json().await.unwrap_or_default();

        if !status.get("em:0").is_some() {
            return false;
        }

        true
    }
    
    pub async fn get_status(&self) -> Result<ShellyStatus> {
        let url = format!("{}/rpc/Shelly.GetStatus", self.base_url);
        let resp = self.client.get(&url).send().await?;
        let status: serde_json::Value = resp.json().await?;

        let total_current = status["em:0"]["total_current"].as_f64().unwrap_or(0.0);
        let total_act_power = status["em:0"]["total_act_power"].as_f64().unwrap_or(0.0);
        let total_aprt_power = status["em:0"]["total_aprt_power"].as_f64().unwrap_or(0.0);
        Ok(ShellyStatus {
            total_current,
            total_act_power,
            total_aprt_power,
        })
    }
}
