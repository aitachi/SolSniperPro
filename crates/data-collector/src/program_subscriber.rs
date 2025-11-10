use solsniper_core::{Event, Result};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use dashmap::DashMap;
use chrono::Utc;

/// Program订阅器
#[derive(Clone)]
pub struct ProgramSubscriber {
    program_id: Pubkey,
    program_name: String,
}

impl ProgramSubscriber {
    pub fn new(program_id: Pubkey, program_name: String) -> Self {
        Self {
            program_id,
            program_name,
        }
    }

    /// 订阅并转发事件到Kafka
    pub async fn subscribe_and_forward(
        &self,
        kafka: Arc<super::KafkaProducer>,
        seen_events: Arc<DashMap<u64, std::time::Instant>>,
    ) -> Result<()> {
        tracing::info!(
            "Starting subscription for {} ({})",
            self.program_name,
            self.program_id
        );

        // TODO: 实际的WebSocket订阅实现
        // 这里使用模拟循环
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            // 模拟接收到事件
            // 实际应该从Solana WebSocket接收
        }

        Ok(())
    }

    /// 解析日志为事件
    fn parse_log(&self, log: &str) -> Option<Event> {
        // 检查是否是池子创建事件
        if log.contains("Initialize") || log.contains("CreatePool") {
            // TODO: 实际解析逻辑
            Some(Event::PoolCreated {
                pool: Pubkey::new_unique(),
                token: Pubkey::new_unique(),
                timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    /// 检查事件是否已处理（去重）
    fn is_duplicate(
        &self,
        event: &Event,
        seen: &DashMap<u64, std::time::Instant>,
    ) -> bool {
        let hash = self.compute_event_hash(event);

        // 检查是否在最近1小时内见过
        if let Some(entry) = seen.get(&hash) {
            if entry.elapsed().as_secs() < 3600 {
                return true;
            }
        }

        // 记录新事件
        seen.insert(hash, std::time::Instant::now());

        // 清理超过1小时的旧记录
        seen.retain(|_, v| v.elapsed().as_secs() < 3600);

        false
    }

    fn compute_event_hash(&self, event: &Event) -> u64 {
        // 简化的哈希实现
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        format!("{:?}", event).hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log() {
        let subscriber = ProgramSubscriber::new(
            Pubkey::new_unique(),
            "Raydium".to_string(),
        );

        let log = "Program log: Initialize pool with 10 SOL";
        let event = subscriber.parse_log(log);
        assert!(event.is_some());
    }
}
