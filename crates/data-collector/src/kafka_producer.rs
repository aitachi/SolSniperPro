use solsniper_core::{Event, Result};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use serde_json;

/// Kafka生产者
pub struct KafkaProducer {
    producer: FutureProducer,
    raw_events_topic: String,
    pool_created_topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: Vec<String>) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers.join(","))
            .set("message.timeout.ms", "5000")
            .set("compression.type", "lz4")
            .set("batch.size", "16384")
            .set("linger.ms", "10")
            .create()
            .map_err(|e| solsniper_core::Error::Kafka(e.to_string()))?;

        Ok(Self {
            producer,
            raw_events_topic: "raw-events".to_string(),
            pool_created_topic: "pool-created".to_string(),
        })
    }

    /// 发送事件到Kafka
    pub async fn send_event(&self, event: &Event) -> Result<()> {
        let topic = match event {
            Event::PoolCreated { .. } => &self.pool_created_topic,
            _ => &self.raw_events_topic,
        };

        let payload = serde_json::to_string(event)
            .map_err(|e| solsniper_core::Error::Serialization(e.to_string()))?;

        let record = FutureRecord::to(topic)
            .payload(&payload)
            .key(&format!("{:?}", event));

        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .map_err(|(e, _)| solsniper_core::Error::Kafka(e.to_string()))?;

        Ok(())
    }

    /// 批量发送事件
    pub async fn send_batch(&self, events: Vec<Event>) -> Result<()> {
        let mut futures = Vec::new();

        for event in events {
            let topic = match event {
                Event::PoolCreated { .. } => &self.pool_created_topic,
                _ => &self.raw_events_topic,
            };

            let payload = serde_json::to_string(&event)
                .map_err(|e| solsniper_core::Error::Serialization(e.to_string()))?;

            let record = FutureRecord::to(topic)
                .payload(&payload)
                .key(&format!("{:?}", event));

            let future = self.producer.send(record, Duration::from_secs(0));
            futures.push(future);
        }

        // 等待所有发送完成
        for future in futures {
            future.await
                .map_err(|(e, _)| solsniper_core::Error::Kafka(e.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use solana_sdk::pubkey::Pubkey;

    #[tokio::test]
    async fn test_kafka_producer() {
        // 需要运行的Kafka实例
        // let producer = KafkaProducer::new(vec!["localhost:9092".to_string()]).unwrap();

        // let event = Event::PoolCreated {
        //     pool: Pubkey::new_unique(),
        //     token: Pubkey::new_unique(),
        //     timestamp: Utc::now(),
        // };

        // producer.send_event(&event).await.unwrap();
    }
}
