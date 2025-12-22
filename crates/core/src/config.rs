use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc: RpcConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
    pub ml: MLConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcConfig {
    pub endpoints: Vec<String>,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub postgres_url: String,
    pub scylla_nodes: Vec<String>,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub group_id: String,
    pub topics: KafkaTopics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaTopics {
    pub raw_events: String,
    pub pool_created: String,
    pub token_analyzed: String,
    pub snipe_signals: String,
    pub trade_executions: String,
    pub risk_alerts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub model_path: String,
    pub enable_online_learning: bool,
    pub update_interval_hours: u64,
}
