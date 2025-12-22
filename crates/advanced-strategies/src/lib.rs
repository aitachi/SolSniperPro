pub mod jito_bundle;
pub mod mempool_monitor;
pub mod pool_creation_monitor;
pub mod token_deployment_monitor;
pub mod priority_fee_optimizer;
pub mod sandwich_attack;

use solsniper_core::{Result, TokenInfo};
use solana_sdk::pubkey::Pubkey;

/// 高级狙击策略管理器
pub struct AdvancedStrategyManager {
    /// JITO MEV捆绑狙击
    jito_sniper: jito_bundle::JitoMevSniper,

    /// 内存池监听器
    mempool_monitor: mempool_monitor::MempoolMonitor,

    /// 池创建监听器
    pool_monitor: pool_creation_monitor::PoolCreationMonitor,

    /// 代币部署监听器
    deployment_monitor: token_deployment_monitor::TokenDeploymentMonitor,

    /// Priority Fee优化器
    priority_optimizer: priority_fee_optimizer::PriorityFeeOptimizer,

    /// 三明治攻击引擎
    sandwich_engine: sandwich_attack::SandwichAttackEngine,
}

impl AdvancedStrategyManager {
    pub fn new(
        jito_endpoint: String,
        helius_api_key: String,
        triton_endpoint: String,
    ) -> Result<Self> {
        Ok(Self {
            jito_sniper: jito_bundle::JitoMevSniper::new(jito_endpoint)?,
            mempool_monitor: mempool_monitor::MempoolMonitor::new(helius_api_key)?,
            pool_monitor: pool_creation_monitor::PoolCreationMonitor::new()?,
            deployment_monitor: token_deployment_monitor::TokenDeploymentMonitor::new()?,
            priority_optimizer: priority_fee_optimizer::PriorityFeeOptimizer::new(),
            sandwich_engine: sandwich_attack::SandwichAttackEngine::new()?,
        })
    }

    /// 启动所有监听器
    pub async fn start_all(&self) -> Result<()> {
        tracing::info!("🚀 启动高级狙击策略系统...");

        tokio::try_join!(
            self.mempool_monitor.start_monitoring(),
            self.pool_monitor.start_monitoring(),
            self.deployment_monitor.start_monitoring(),
        )?;

        tracing::info!("✅ 所有监听器已启动");
        Ok(())
    }

    /// 执行JITO捆绑狙击
    pub async fn execute_jito_snipe(
        &self,
        token: &TokenInfo,
        amount_sol: f64,
        tip_lamports: u64,
    ) -> Result<String> {
        self.jito_sniper.execute_bundle_snipe(token, amount_sol, tip_lamports).await
    }

    /// 执行三明治攻击
    pub async fn execute_sandwich_attack(
        &self,
        target_tx_signature: &str,
        target_amount: u64,
    ) -> Result<(String, String)> {
        self.sandwich_engine.execute_sandwich(target_tx_signature, target_amount).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_advanced_strategy_manager() {
        // 需要实际的API密钥
        // let manager = AdvancedStrategyManager::new(
        //     "https://mainnet.block-engine.jito.wtf".to_string(),
        //     "your_helius_key".to_string(),
        //     "your_triton_endpoint".to_string(),
        // ).unwrap();
    }
}
