pub mod identifier;
pub mod follower;
pub mod analyzer;

use solsniper_core::{SmartWallet, Result};
use solana_sdk::pubkey::Pubkey;
use dashmap::DashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};

pub use identifier::SmartWalletIdentifier;
pub use follower::SmartMoneyFollower;
pub use analyzer::TradeAnalyzer;

/// 聪明钱追踪系统
pub struct SmartMoneyTracker {
    /// 聪明钱钱包数据库
    smart_wallets: Arc<DashMap<Pubkey, SmartWallet>>,

    /// 钱包识别器
    identifier: SmartWalletIdentifier,

    /// 跟单执行器
    follower: SmartMoneyFollower,

    /// 交易分析器
    analyzer: TradeAnalyzer,
}

impl SmartMoneyTracker {
    pub fn new(db_url: &str) -> Result<Self> {
        let smart_wallets = Arc::new(DashMap::new());

        Ok(Self {
            smart_wallets: Arc::clone(&smart_wallets),
            identifier: SmartWalletIdentifier::new(db_url)?,
            follower: SmartMoneyFollower::new(Arc::clone(&smart_wallets)),
            analyzer: TradeAnalyzer::new(db_url)?,
        })
    }

    /// 启动聪明钱识别
    pub async fn identify_smart_wallets(&mut self) -> Result<()> {
        tracing::info!("Starting smart wallet identification...");

        let identified = self.identifier.identify_smart_wallets().await?;

        for wallet in identified {
            self.smart_wallets.insert(wallet.address, wallet);
        }

        tracing::info!(
            "Identified {} smart wallets",
            self.smart_wallets.len()
        );

        Ok(())
    }

    /// 启动实时跟单
    pub async fn start_following(&self) -> Result<()> {
        tracing::info!("Starting smart money following...");
        self.follower.start_following().await
    }

    /// 获取所有聪明钱钱包
    pub fn get_smart_wallets(&self) -> Vec<SmartWallet> {
        self.smart_wallets
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// 检查是否为聪明钱钱包
    pub fn is_smart_wallet(&self, address: &Pubkey) -> bool {
        self.smart_wallets.contains_key(address)
    }

    /// 更新钱包统计
    pub async fn update_wallet_stats(&self, address: &Pubkey) -> Result<()> {
        if let Some(mut entry) = self.smart_wallets.get_mut(address) {
            let updated = self.analyzer.fetch_wallet_stats(address).await?;
            *entry = updated;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_smart_money_tracker() {
        // 测试需要真实数据库连接
        // let tracker = SmartMoneyTracker::new("postgresql://...").unwrap();
        // let wallets = tracker.get_smart_wallets();
        // assert!(wallets.len() > 0);
    }
}
