use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub ws_clients: Arc<RwLock<Vec<tokio::sync::mpsc::UnboundedSender<String>>>>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self> {
        // 连接数据库
        let db = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect(&config.database.url)
            .await?;

        tracing::info!("✅ PostgreSQL 连接成功");

        // 连接 Redis
        let redis_client = redis::Client::open(config.redis.url.as_str())?;
        let redis = ConnectionManager::new(redis_client).await?;

        tracing::info!("✅ Redis 连接成功");

        Ok(Self {
            config,
            db,
            redis,
            ws_clients: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn broadcast_message(&self, message: String) {
        let clients = self.ws_clients.read().await;
        for client in clients.iter() {
            let _ = client.send(message.clone());
        }
    }
}
