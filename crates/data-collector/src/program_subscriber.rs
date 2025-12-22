use solsniper_core::{Event, Result};
use solana_sdk::pubkey::Pubkey;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use futures::stream::StreamExt;
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
    ///
    /// 实现完整的WebSocket订阅流程:
    /// 1. 连接到Solana WebSocket RPC
    /// 2. 订阅指定program的日志
    /// 3. 解析日志为事件
    /// 4. 去重检查
    /// 5. 转发到Kafka
    pub async fn subscribe_and_forward(
        &self,
        kafka: Arc<super::KafkaProducer>,
        seen_events: Arc<DashMap<u64, std::time::Instant>>,
    ) -> Result<()> {
        tracing::info!(
            "🔌 Starting WebSocket subscription for {} ({})",
            self.program_name,
            self.program_id
        );

        // 1. 创建WebSocket订阅客户端
        let ws_url = self.get_websocket_url();
        let pubsub_client = PubsubClient::new(&ws_url)
            .await
            .map_err(|e| solsniper_core::Error::Internal(
                format!("Failed to connect to WebSocket {}: {}", ws_url, e)
            ))?;

        tracing::info!("✅ Connected to WebSocket: {}", ws_url);

        // 2. 订阅程序日志
        let (mut notifications, unsubscribe) = pubsub_client
            .logs_subscribe(
                RpcTransactionLogsFilter::Mentions(vec![self.program_id.to_string()]),
                Some(RpcTransactionLogsConfig {
                    commitment: Some(CommitmentConfig::confirmed()),
                }),
            )
            .await
            .map_err(|e| solsniper_core::Error::Internal(
                format!("Failed to subscribe to {} logs: {}", self.program_name, e)
            ))?;

        tracing::info!(
            "✅ Subscribed to {} program logs, waiting for events...",
            self.program_name
        );

        let mut event_count = 0u64;
        let mut error_count = 0u64;

        // 3. 持续接收和处理事件
        while let Some(log_notification) = notifications.next().await {
            let signature = log_notification.value.signature;
            let logs = log_notification.value.logs;
            let err = log_notification.value.err;

            // 跳过失败的交易
            if err.is_some() {
                tracing::debug!(
                    "Skipping failed transaction {} for {}",
                    signature,
                    self.program_name
                );
                continue;
            }

            // 4. 解析每条日志
            for log in logs {
                if let Some(event) = self.parse_log(&log) {
                    // 5. 去重检查
                    if self.is_duplicate(&event, &seen_events) {
                        tracing::trace!(
                            "Duplicate event detected for {}, skipping",
                            self.program_name
                        );
                        continue;
                    }

                    // 6. 发送到Kafka
                    match kafka.send_event(&event).await {
                        Ok(_) => {
                            event_count += 1;
                            tracing::debug!(
                                "✅ Event #{} forwarded: {} - {} (sig: {})",
                                event_count,
                                self.program_name,
                                self.describe_event(&event),
                                signature
                            );

                            // 每100个事件输出统计
                            if event_count % 100 == 0 {
                                tracing::info!(
                                    "📊 {} statistics: {} events forwarded, {} errors",
                                    self.program_name,
                                    event_count,
                                    error_count
                                );
                            }
                        }
                        Err(e) => {
                            error_count += 1;
                            tracing::error!(
                                "❌ Failed to send event to Kafka for {}: {}",
                                self.program_name,
                                e
                            );

                            // 如果错误过多，可能需要重连
                            if error_count > 50 {
                                tracing::warn!(
                                    "⚠️ Too many Kafka errors ({}), connection may be broken",
                                    error_count
                                );
                            }
                        }
                    }
                }
            }
        }

        // 7. 清理订阅（正常情况下不会到这里，因为上面是无限循环）
        tracing::warn!(
            "WebSocket stream ended for {}, unsubscribing...",
            self.program_name
        );
        unsubscribe().await;

        Err(solsniper_core::Error::Internal(
            format!("WebSocket subscription ended unexpectedly for {}", self.program_name)
        ))
    }

    /// 获取WebSocket URL
    ///
    /// 优先使用环境变量配置，否则使用默认的公共端点
    fn get_websocket_url(&self) -> String {
        std::env::var("SOLANA_WS_URL")
            .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string())
    }

    /// 描述事件类型（用于日志）
    fn describe_event(&self, event: &Event) -> String {
        match event {
            Event::PoolCreated { token, .. } => format!("PoolCreated(token={})", token),
            Event::TokenLaunched { token, .. } => format!("TokenLaunched(token={})", token),
            Event::LiquidityAdded { pool, amount, .. } => {
                format!("LiquidityAdded(pool={}, amount={})", pool, amount)
            }
            Event::LargeSwap { pool, amount, .. } => {
                format!("LargeSwap(pool={}, amount={})", pool, amount)
            }
        }
    }

    /// 解析日志为事件
    ///
    /// 支持的事件类型:
    /// - PoolCreated: 新池子创建
    /// - TokenLaunched: 新代币发行
    /// - LiquidityAdded: 添加流动性
    /// - LargeSwap: 大额交易
    fn parse_log(&self, log: &str) -> Option<Event> {
        // 根据不同的program解析不同的日志格式

        // Raydium AMM池子创建
        if log.contains("Program log: initialize2") || log.contains("Program log: ray_log:") && log.contains("init_pc_amount") {
            return self.parse_raydium_pool_creation(log);
        }

        // Raydium/Orca池子初始化
        if log.contains("Program log: Instruction: Initialize") || log.contains("Program log: InitializePool") {
            return self.parse_pool_initialization(log);
        }

        // Pump.fun代币发行
        if log.contains("Program log: create") && self.program_name.contains("Pump") {
            return self.parse_pumpfun_launch(log);
        }

        // 大额swap检测
        if log.contains("Program log: Swap") || log.contains("Program log: SwapEvent") {
            return self.parse_swap_event(log);
        }

        // 流动性添加
        if log.contains("Program log: AddLiquidity") || log.contains("Program log: deposit") {
            return self.parse_liquidity_add(log);
        }

        None
    }

    /// 解析Raydium池子创建事件
    fn parse_raydium_pool_creation(&self, log: &str) -> Option<Event> {
        // Raydium日志格式: "Program log: ray_log: <data>"
        // 尝试提取代币mint地址（通常在日志中）

        // 简化实现：创建占位符事件
        // 实际实现需要解析具体的账户和金额数据
        tracing::debug!("Detected Raydium pool creation: {}", log);

        Some(Event::PoolCreated {
            pool: Pubkey::new_unique(), // TODO: 从日志中提取实际pool地址
            token: Pubkey::new_unique(), // TODO: 从日志中提取实际token mint
            timestamp: Utc::now(),
        })
    }

    /// 解析通用池子初始化事件
    fn parse_pool_initialization(&self, log: &str) -> Option<Event> {
        tracing::debug!("Detected pool initialization: {}", log);

        Some(Event::PoolCreated {
            pool: Pubkey::new_unique(),
            token: Pubkey::new_unique(),
            timestamp: Utc::now(),
        })
    }

    /// 解析Pump.fun代币发行事件
    fn parse_pumpfun_launch(&self, log: &str) -> Option<Event> {
        tracing::debug!("Detected Pump.fun token launch: {}", log);

        Some(Event::TokenLaunched {
            token: Pubkey::new_unique(),
            creator: Pubkey::new_unique(),
            timestamp: Utc::now(),
        })
    }

    /// 解析swap事件
    fn parse_swap_event(&self, log: &str) -> Option<Event> {
        // 尝试从日志中提取交易金额
        // Raydium日志示例: "Program log: SwapEvent: amount_in=1000000000, amount_out=..."

        let amount = self.extract_amount_from_log(log).unwrap_or(0);

        // 只关注大额交易（> 10 SOL）
        if amount > 10_000_000_000 {
            tracing::debug!("Detected large swap: {} lamports", amount);

            return Some(Event::LargeSwap {
                pool: Pubkey::new_unique(),
                amount,
                timestamp: Utc::now(),
            });
        }

        None
    }

    /// 解析流动性添加事件
    fn parse_liquidity_add(&self, log: &str) -> Option<Event> {
        let amount = self.extract_amount_from_log(log).unwrap_or(0);

        tracing::debug!("Detected liquidity addition: {} lamports", amount);

        Some(Event::LiquidityAdded {
            pool: Pubkey::new_unique(),
            amount,
            timestamp: Utc::now(),
        })
    }

    /// 从日志中提取金额
    ///
    /// 尝试匹配常见的金额模式:
    /// - "amount=1000000000"
    /// - "amount_in=1000000000"
    /// - "pc_amount=1000000000"
    fn extract_amount_from_log(&self, log: &str) -> Option<u64> {
        // 使用正则表达式或字符串解析
        // 简化实现：查找数字模式

        for pattern in &["amount=", "amount_in=", "pc_amount=", "coin_amount="] {
            if let Some(pos) = log.find(pattern) {
                let after_pattern = &log[pos + pattern.len()..];

                // 提取数字部分
                let number_str: String = after_pattern
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect();

                if let Ok(amount) = number_str.parse::<u64>() {
                    return Some(amount);
                }
            }
        }

        None
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
