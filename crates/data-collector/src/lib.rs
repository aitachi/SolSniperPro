pub mod websocket;
pub mod program_subscriber;
pub mod event_parser;
pub mod kafka_producer;
pub mod parallel_fetcher;

use solsniper_core::{Event, Result};
use std::sync::Arc;
use tokio::sync::mpsc;
use dashmap::DashMap;
use solana_sdk::pubkey::Pubkey;

pub use program_subscriber::ProgramSubscriber;
pub use kafka_producer::KafkaProducer;
pub use parallel_fetcher::{ParallelDataCollector, DataSource, ParallelFetchResult, ApiTokenData};

/// 多源数据采集器
pub struct MultiSourceCollector {
    /// 各DEX的Program订阅器
    subscribers: Vec<ProgramSubscriber>,

    /// Kafka生产者
    kafka_producer: Arc<KafkaProducer>,

    /// 事件去重缓存
    seen_events: Arc<DashMap<u64, std::time::Instant>>,

    /// 后台清理任务句柄
    cleanup_handle: Option<tokio::task::JoinHandle<()>>,
}

impl MultiSourceCollector {
    pub fn new(kafka_brokers: Vec<String>) -> Result<Self> {
        let kafka_producer = Arc::new(KafkaProducer::new(kafka_brokers)?);
        let seen_events = Arc::new(DashMap::new());

        // 启动后台清理任务
        let cleanup_handle = Self::spawn_cleanup_task(Arc::clone(&seen_events));

        Ok(Self {
            subscribers: Vec::new(),
            kafka_producer,
            seen_events,
            cleanup_handle: Some(cleanup_handle),
        })
    }

    /// 启动后台清理任务
    ///
    /// 每5分钟清理一次超过1小时的旧事件，防止内存泄漏
    fn spawn_cleanup_task(
        seen_events: Arc<DashMap<u64, std::time::Instant>>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5分钟

            loop {
                interval.tick().await;

                let before_count = seen_events.len();

                // 清理超过1小时的事件
                seen_events.retain(|_, timestamp| {
                    timestamp.elapsed().as_secs() < 3600
                });

                let after_count = seen_events.len();
                let removed_count = before_count.saturating_sub(after_count);

                if removed_count > 0 {
                    tracing::info!(
                        "🧹 Cleaned up {} expired events from dedup cache ({} -> {})",
                        removed_count,
                        before_count,
                        after_count
                    );
                } else {
                    tracing::debug!(
                        "🧹 Cleanup task ran: {} events in cache, 0 expired",
                        after_count
                    );
                }
            }
        })
    }

    /// 添加订阅器
    pub fn add_subscriber(&mut self, subscriber: ProgramSubscriber) {
        self.subscribers.push(subscriber);
    }

    /// 启动所有订阅
    pub async fn start_all(&self) -> Result<()> {
        tracing::info!("Starting {} subscribers", self.subscribers.len());

        let mut handles = Vec::new();

        for subscriber in &self.subscribers {
            let kafka = Arc::clone(&self.kafka_producer);
            let seen = Arc::clone(&self.seen_events);
            let sub = subscriber.clone();

            let handle = tokio::spawn(async move {
                if let Err(e) = sub.subscribe_and_forward(kafka, seen).await {
                    tracing::error!("Subscriber error: {}", e);
                }
            });

            handles.push(handle);
        }

        // 等待所有订阅器
        futures::future::join_all(handles).await;

        Ok(())
    }
}

impl Drop for MultiSourceCollector {
    fn drop(&mut self) {
        // 优雅地停止后台清理任务
        if let Some(handle) = self.cleanup_handle.take() {
            handle.abort();
            tracing::debug!("Stopped event dedup cleanup task");
        }
    }
}

/// DEX Program IDs
pub mod dex_programs {
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    /// Raydium AMM V4
    pub fn raydium_amm() -> Pubkey {
        Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap()
    }

    /// Raydium CLMM
    pub fn raydium_clmm() -> Pubkey {
        Pubkey::from_str("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK").unwrap()
    }

    /// Orca Whirlpool
    pub fn orca_whirlpool() -> Pubkey {
        Pubkey::from_str("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc").unwrap()
    }

    /// Meteora DLMM
    pub fn meteora_dlmm() -> Pubkey {
        Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap()
    }

    /// Pump.fun
    pub fn pumpfun() -> Pubkey {
        Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dex_programs() {
        let programs = vec![
            dex_programs::raydium_amm(),
            dex_programs::orca_whirlpool(),
            dex_programs::pumpfun(),
        ];

        assert_eq!(programs.len(), 3);
    }
}
