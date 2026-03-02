use serde::{Deserialize, Serialize};
use std::fs;
use std::net::IpAddr;
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Miner {
    pub ip: IpAddr,
    pub timestamp: DateTime<Utc>,
}

/// Collection of miners.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Miners {
    pub list: Vec<Miner>,
}