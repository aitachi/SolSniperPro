use solsniper_core::{Result, TokenInfo};
use solana_sdk::{
    signature::{Keypair, Signer, Signature},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
    instruction::Instruction,
    message::Message,
    hash::Hash,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// JITO MEV 捆绑狙击器
///
/// 核心原理:
/// 通过支付高额小费，将交易打包成"捆绑包"发送给验证者，
/// 获得绝对的优先执行权。这是Solana上最主流、最有效的狙击方式。
///
/// 关键技术:
/// - Jito Bundle API
/// - Jito-Solana RPC端点
/// - Bundle提交和确认机制
pub struct JitoMevSniper {
    /// Jito Block Engine端点
    block_engine_url: String,

    /// HTTP客户端
    client: Client,

    /// 钱包
    wallet: Arc<Keypair>,

    /// Jito tip账户列表
    tip_accounts: Vec<Pubkey>,
}

impl JitoMevSniper {
    pub fn new(block_engine_url: String, wallet: Arc<Keypair>) -> Result<Self> {
        Ok(Self {
            block_engine_url,
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| solsniper_core::Error::Internal(format!("Failed to build HTTP client: {}", e)))?,
            wallet,
            tip_accounts: Self::get_jito_tip_accounts(),
        })
    }

    /// 获取JITO官方tip账户列表
    fn get_jito_tip_accounts() -> Vec<Pubkey> {
        vec![
            "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5",
            "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe",
            "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY",
            "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49",
            "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh",
            "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt",
            "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL",
            "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT",
        ]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect()
    }

    /// 执行JITO捆绑狙击
    ///
    /// # 参数
    /// - `token`: 目标代币信息
    /// - `amount_sol`: 买入金额(SOL)
    /// - `tip_lamports`: 给验证者的小费(lamports)
    /// - `swap_transaction`: 已构建好的swap交易
    ///
    /// # 流程
    /// 1. 构建小费交易(tip transaction)
    /// 2. 打包成Bundle
    /// 3. 提交到Jito Block Engine
    /// 4. 等待上链确认
    pub async fn execute_bundle_snipe(
        &self,
        token: &TokenInfo,
        amount_sol: f64,
        tip_lamports: u64,
        swap_transaction: Transaction,
        recent_blockhash: Hash,
    ) -> Result<String> {
        tracing::info!(
            "🎯 JITO捆绑狙击: 代币={}, 金额={} SOL, 小费={} lamports",
            token.symbol, amount_sol, tip_lamports
        );

        // 1. 构建小费交易
        let tip_tx = self.build_tip_transaction(tip_lamports, recent_blockhash).await?;

        // 2. 创建Bundle（先小费，后swap）
        let bundle = self.create_bundle(vec![tip_tx, swap_transaction]).await?;

        // 3. 提交Bundle
        let bundle_id = self.submit_bundle(bundle).await?;

        tracing::info!("✅ Bundle已提交: {}", bundle_id);

        // 4. 等待确认
        self.wait_for_bundle_confirmation(&bundle_id).await?;

        Ok(bundle_id)
    }

    /// 构建小费交易
    ///
    /// 小费交易是一笔简单的SOL转账，发送到Jito指定的tip账户
    async fn build_tip_transaction(&self, tip_lamports: u64, recent_blockhash: Hash) -> Result<Transaction> {
        // 随机选择一个tip账户（负载均衡）
        use rand::Rng;
        let tip_account_index = rand::thread_rng().gen_range(0..self.tip_accounts.len());
        let tip_account = &self.tip_accounts[tip_account_index];

        tracing::debug!(
            "构建小费交易: {} lamports ({:.6} SOL) -> {}",
            tip_lamports,
            tip_lamports as f64 / 1e9,
            tip_account
        );

        // 创建转账指令
        let transfer_ix = system_instruction::transfer(
            &self.wallet.pubkey(),
            tip_account,
            tip_lamports,
        );

        // 构建消息
        let message = Message::new_with_blockhash(
            &[transfer_ix],
            Some(&self.wallet.pubkey()),
            &recent_blockhash,
        );

        // 创建并签名交易
        let mut transaction = Transaction::new_unsigned(message);
        transaction.sign(&[&*self.wallet], recent_blockhash);

        Ok(transaction)
    }

    /// 创建Bundle
    async fn create_bundle(&self, transactions: Vec<Transaction>) -> Result<JitoBundle> {
        let bundle = JitoBundle {
            transactions: transactions
                .iter()
                .map(|tx| {
                    // 序列化交易为base58
                    let serialized = bincode::serialize(tx)
                        .map_err(|e| solsniper_core::Error::Internal(format!("Failed to serialize transaction: {}", e)))?;
                    Ok(bs58::encode(serialized).into_string())
                })
                .collect::<Result<Vec<String>>>()?,
        };

        tracing::debug!("创建Bundle: {} 笔交易", bundle.transactions.len());

        Ok(bundle)
    }

    /// 提交Bundle到Jito Block Engine
    async fn submit_bundle(&self, bundle: JitoBundle) -> Result<String> {
        let url = format!("{}/api/v1/bundles", self.block_engine_url);

        tracing::debug!("提交Bundle到: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&bundle)
            .send()
            .await
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to send bundle: {}", e)))?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to read response: {}", e)))?;

        if !status.is_success() {
            return Err(solsniper_core::Error::Internal(format!(
                "Bundle submission failed with status {}: {}",
                status, response_text
            )));
        }

        // 解析响应
        let result: JitoBundleResponse = serde_json::from_str(&response_text)
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to parse response: {}. Response: {}", e, response_text)))?;

        if let Some(bundle_id) = result.bundle_id {
            Ok(bundle_id)
        } else {
            Err(solsniper_core::Error::Internal(format!(
                "Bundle提交失败: {:?}",
                result.error
            )))
        }
    }

    /// 等待Bundle确认
    async fn wait_for_bundle_confirmation(&self, bundle_id: &str) -> Result<()> {
        tracing::info!("⏳ 等待Bundle确认: {}", bundle_id);

        const MAX_RETRIES: u32 = 30;
        const RETRY_INTERVAL: Duration = Duration::from_millis(500);

        for attempt in 1..=MAX_RETRIES {
            // 查询Bundle状态
            let status = self.query_bundle_status(bundle_id).await?;

            match status {
                BundleStatus::Landed => {
                    tracing::info!("✅ Bundle已确认并上链");
                    return Ok(());
                }
                BundleStatus::Failed => {
                    return Err(solsniper_core::Error::Internal(
                        "Bundle执行失败".to_string()
                    ));
                }
                BundleStatus::Pending => {
                    tracing::debug!("Bundle pending, attempt {}/{}", attempt, MAX_RETRIES);
                }
            }

            tokio::time::sleep(RETRY_INTERVAL).await;
        }

        Err(solsniper_core::Error::Internal(format!(
            "Bundle confirmation timeout after {} attempts",
            MAX_RETRIES
        )))
    }

    /// 查询Bundle状态
    async fn query_bundle_status(&self, bundle_id: &str) -> Result<BundleStatus> {
        let url = format!("{}/api/v1/bundles/status/{}", self.block_engine_url, bundle_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to query bundle status: {}", e)))?;

        if !response.status().is_success() {
            // 如果状态查询失败，假设pending
            return Ok(BundleStatus::Pending);
        }

        let status_response: BundleStatusResponse = response
            .json()
            .await
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to parse status response: {}", e)))?;

        Ok(status_response.status)
    }

    /// 计算推荐的小费金额
    ///
    /// 根据网络拥堵情况和竞争激烈程度动态计算
    pub async fn calculate_optimal_tip(&self, priority: TipPriority) -> u64 {
        // 基准小费
        let base_tip = match priority {
            TipPriority::Low => 500_000,       // 0.0005 SOL
            TipPriority::Medium => 1_000_000,  // 0.001 SOL
            TipPriority::High => 2_000_000,    // 0.002 SOL
            TipPriority::Critical => 5_000_000, // 0.005 SOL
        };

        // TODO: 查询最近成功Bundle的小费，动态调整
        // 可以调用 Jito API 获取最近的tip统计

        base_tip
    }

    /// 获取最近成功Bundle的统计信息
    pub async fn get_recent_bundle_stats(&self) -> Result<BundleStats> {
        let url = format!("{}/api/v1/bundles/stats", self.block_engine_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to get bundle stats: {}", e)))?;

        if !response.status().is_success() {
            return Ok(BundleStats::default());
        }

        let stats = response
            .json()
            .await
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to parse stats: {}", e)))?;

        Ok(stats)
    }
}

/// Tip优先级
#[derive(Debug, Clone, Copy)]
pub enum TipPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Bundle状态
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BundleStatus {
    Pending,
    Landed,
    Failed,
}

/// Jito Bundle结构
#[derive(Debug, Serialize, Deserialize)]
struct JitoBundle {
    /// Base58编码的交易列表
    transactions: Vec<String>,
}

/// Jito Bundle响应
#[derive(Debug, Deserialize)]
struct JitoBundleResponse {
    /// Bundle ID
    #[serde(rename = "bundleId")]
    bundle_id: Option<String>,

    /// 错误信息
    error: Option<String>,
}

/// Bundle状态响应
#[derive(Debug, Deserialize)]
struct BundleStatusResponse {
    status: BundleStatus,
}

/// Bundle统计信息
#[derive(Debug, Clone, Deserialize)]
pub struct BundleStats {
    pub total_bundles: u64,
    pub successful_bundles: u64,
    pub failed_bundles: u64,
    pub avg_tip_lamports: u64,
    pub median_tip_lamports: u64,
}

impl Default for BundleStats {
    fn default() -> Self {
        Self {
            total_bundles: 0,
            successful_bundles: 0,
            failed_bundles: 0,
            avg_tip_lamports: 1_000_000,
            median_tip_lamports: 1_000_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tip_accounts() {
        let accounts = JitoMevSniper::get_jito_tip_accounts();
        assert_eq!(accounts.len(), 8);
    }

    #[tokio::test]
    async fn test_optimal_tip_calculation() {
        let wallet = Arc::new(Keypair::new());
        let sniper = JitoMevSniper::new(
            "https://mainnet.block-engine.jito.wtf".to_string(),
            wallet,
        )
        .unwrap();

        let low_tip = sniper.calculate_optimal_tip(TipPriority::Low).await;
        let high_tip = sniper.calculate_optimal_tip(TipPriority::High).await;

        assert!(low_tip < high_tip);
        assert_eq!(low_tip, 500_000);
        assert_eq!(high_tip, 2_000_000);

        println!("Low priority tip: {} lamports ({} SOL)", low_tip, low_tip as f64 / 1e9);
        println!("High priority tip: {} lamports ({} SOL)", high_tip, high_tip as f64 / 1e9);
    }
}

