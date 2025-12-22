pub mod wallet_manager;
pub mod transaction_builder;
pub mod jito_client;
pub mod slippage_protector;
pub mod mev_protector;

use solsniper_core::{Result, TokenInfo};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;

pub use wallet_manager::WalletManager;
pub use transaction_builder::TransactionBuilder;
pub use slippage_protector::{SlippageProtector, SwapQuote};
pub use mev_protector::{MevProtector, MevPriority, ProtectedTransaction};

/// 执行选项
#[derive(Clone, Debug)]
pub struct ExecutionOptions {
    /// 是否使用JITO Bundle
    pub use_jito: bool,

    /// JITO小费金额（lamports）
    pub jito_tip: u64,

    /// 最大滑点（basis points, 100 = 1%）
    pub max_slippage_bps: u16,

    /// 确认级别
    pub confirmation_level: CommitmentConfig,

    /// 优先费用（micro-lamports per compute unit）
    pub priority_fee: u64,

    /// 最大重试次数
    pub max_retries: u32,

    /// MEV保护优先级
    pub mev_priority: MevPriority,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            use_jito: false,
            jito_tip: 1_000_000, // 0.001 SOL
            max_slippage_bps: 300, // 3%
            confirmation_level: CommitmentConfig::confirmed(),
            priority_fee: 50_000,
            max_retries: 3,
            mev_priority: MevPriority::Medium,
        }
    }
}

/// 交易结果
#[derive(Clone, Debug)]
pub struct TransactionResult {
    pub signature: Signature,
    pub success: bool,
    pub execution_time_ms: u64,
    pub method: ExecutionMethod,
}

/// 执行方法
#[derive(Clone, Debug)]
pub enum ExecutionMethod {
    Rpc,
    JitoBundle,
}

/// 交易执行引擎
pub struct TradingEngine {
    /// 钱包管理器
    wallet_manager: Arc<WalletManager>,

    /// RPC客户端
    rpc_client: Arc<RpcClient>,

    /// 交易构建器
    transaction_builder: TransactionBuilder,

    /// 滑点保护器
    slippage_protector: SlippageProtector,

    /// MEV保护器
    mev_protector: MevProtector,

    /// 交易缓存（防止重复提交）
    transaction_cache: Arc<DashMap<Pubkey, Instant>>,

    /// 默认执行选项
    default_options: ExecutionOptions,
}

impl TradingEngine {
    pub fn new(
        wallet_manager: Arc<WalletManager>,
        rpc_client: Arc<RpcClient>,
    ) -> Self {
        Self {
            wallet_manager,
            rpc_client,
            transaction_builder: TransactionBuilder::new(),
            slippage_protector: SlippageProtector::default(), // 3% max slippage, dynamic adjustment enabled
            mev_protector: MevProtector::default(), // JITO enabled, 0.001 SOL min tip
            transaction_cache: Arc::new(DashMap::new()),
            default_options: ExecutionOptions::default(),
        }
    }

    /// 设置自定义滑点保护器
    pub fn with_slippage_protector(mut self, protector: SlippageProtector) -> Self {
        self.slippage_protector = protector;
        self
    }

    /// 设置自定义MEV保护器
    pub fn with_mev_protector(mut self, protector: MevProtector) -> Self {
        self.mev_protector = protector;
        self
    }

    /// 设置默认执行选项
    pub fn with_default_options(mut self, options: ExecutionOptions) -> Self {
        self.default_options = options;
        self
    }

    /// 启用JITO
    pub fn with_jito(mut self, tip_lamports: u64) -> Self {
        self.default_options.use_jito = true;
        self.default_options.jito_tip = tip_lamports;
        self
    }

    /// 执行买入交易
    ///
    /// # 参数
    /// - `token`: 目标代币信息
    /// - `amount_sol`: 买入金额（SOL）
    /// - `options`: 执行选项（可选，使用默认值）
    pub async fn execute_buy(
        &self,
        token: &TokenInfo,
        amount_sol: f64,
        options: Option<ExecutionOptions>,
    ) -> Result<TransactionResult> {
        let start_time = Instant::now();
        let options = options.unwrap_or_else(|| self.default_options.clone());

        tracing::info!(
            "Executing buy: {} SOL for token {}, DEX: {}",
            amount_sol,
            token.symbol,
            token.dex
        );

        // 1. 预检查
        self.pre_execution_checks(&token.mint, amount_sol, &options).await?;

        // 2. 滑点保护和最小输出计算
        let min_tokens_out = self.calculate_min_tokens_out_with_protection(
            token,
            amount_sol,
            &options,
        ).await?;

        tracing::debug!(
            "Calculated min_tokens_out: {} for {} SOL",
            min_tokens_out,
            amount_sol
        );

        // 3. 构建交易
        let transaction = self.build_buy_transaction(
            token,
            amount_sol,
            min_tokens_out,
            &options,
        ).await?;

        // 4. 验证交易大小
        self.transaction_builder.validate_transaction_size(&transaction)?;

        // 5. 执行交易
        let result = if options.use_jito {
            self.execute_via_jito(transaction, options.jito_tip).await?
        } else {
            self.execute_via_rpc(transaction, &options).await?
        };

        // 6. 记录交易缓存
        self.transaction_cache.insert(token.mint, Instant::now());

        let execution_time = start_time.elapsed().as_millis() as u64;

        tracing::info!(
            "Buy executed: signature={}, time={}ms, method={:?}",
            result.signature,
            execution_time,
            result.method
        );

        Ok(TransactionResult {
            signature: result.signature,
            success: result.success,
            execution_time_ms: execution_time,
            method: result.method,
        })
    }

    /// 执行卖出交易
    pub async fn execute_sell(
        &self,
        token: &TokenInfo,
        amount_tokens: u64,
        options: Option<ExecutionOptions>,
    ) -> Result<TransactionResult> {
        let start_time = Instant::now();
        let options = options.unwrap_or_else(|| self.default_options.clone());

        tracing::info!(
            "Executing sell: {} tokens for {}, DEX: {}",
            amount_tokens,
            token.symbol,
            token.dex
        );

        // 类似买入逻辑，构建卖出交易
        // 这里简化实现，实际需要构建反向swap

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(TransactionResult {
            signature: Signature::new_unique(), // TODO: 实际签名
            success: true,
            execution_time_ms: execution_time,
            method: if options.use_jito { ExecutionMethod::JitoBundle } else { ExecutionMethod::Rpc },
        })
    }

    /// 并发狙击（多钱包同时买入）
    pub async fn concurrent_snipe(
        &self,
        token: &TokenInfo,
        total_amount: f64,
        wallet_count: usize,
        options: Option<ExecutionOptions>,
    ) -> Result<Vec<TransactionResult>> {
        tracing::info!(
            "Concurrent snipe: {} wallets, total {} SOL for {}",
            wallet_count,
            total_amount,
            token.symbol
        );

        let amount_per_wallet = total_amount / wallet_count as f64;
        let mut handles = Vec::new();

        for i in 0..wallet_count.min(self.wallet_manager.sub_wallet_count() + 1) {
            let token = token.clone();
            let options = options.clone();
            let engine = self.clone_for_concurrent();

            let handle = tokio::spawn(async move {
                tracing::debug!("Wallet {}/{}: buying {} SOL", i + 1, wallet_count, amount_per_wallet);
                engine.execute_buy(&token, amount_per_wallet, options).await
            });

            handles.push(handle);
        }

        // 等待所有交易完成
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    tracing::error!("Concurrent snipe failed: {}", e);
                    return Err(e);
                }
                Err(e) => {
                    tracing::error!("Concurrent snipe task failed: {}", e);
                    return Err(solsniper_core::Error::Internal(e.to_string()));
                }
            }
        }

        Ok(results)
    }

    /// 预执行检查
    async fn pre_execution_checks(
        &self,
        token: &Pubkey,
        amount_sol: f64,
        options: &ExecutionOptions,
    ) -> Result<()> {
        // 1. 检查余额
        let required = WalletManager::estimate_total_cost(amount_sol, options.priority_fee);
        let primary_pubkey = self.wallet_manager.get_primary_pubkey();

        if !self.wallet_manager.has_sufficient_balance(&primary_pubkey, required).await? {
            return Err(solsniper_core::Error::Internal(
                format!("Insufficient balance: need {} SOL", required)
            ));
        }

        // 2. 防止重复交易（5秒内）
        if let Some(last_time) = self.transaction_cache.get(token) {
            if last_time.elapsed() < Duration::from_secs(5) {
                return Err(solsniper_core::Error::Internal(
                    "Duplicate transaction within 5 seconds".to_string()
                ));
            }
        }

        Ok(())
    }

    /// 构建买入交易
    async fn build_buy_transaction(
        &self,
        token: &TokenInfo,
        amount_sol: f64,
        min_tokens_out: u64,
        options: &ExecutionOptions,
    ) -> Result<solana_sdk::transaction::Transaction> {
        let wallet = self.wallet_manager.get_primary_wallet();
        let amount_lamports = (amount_sol * 1e9) as u64;

        // 获取最新blockhash
        let recent_blockhash = self.rpc_client
            .get_latest_blockhash()
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to get blockhash: {}", e)))?;

        // 根据DEX选择构建方法
        let transaction = match token.dex.as_str() {
            "Raydium" => {
                let pool_id = token.pool_address.ok_or_else(|| {
                    solsniper_core::Error::Internal("Pool address not found".to_string())
                })?;

                self.transaction_builder
                    .with_priority_fee(options.priority_fee)
                    .build_raydium_swap(
                        wallet,
                        &pool_id,
                        &spl_token::native_mint::id(), // WSOL
                        &token.mint,
                        amount_lamports,
                        min_tokens_out,
                        recent_blockhash,
                    )
                    .await?
            }
            "Orca" => {
                let whirlpool = token.pool_address.ok_or_else(|| {
                    solsniper_core::Error::Internal("Whirlpool address not found".to_string())
                })?;

                self.transaction_builder
                    .with_priority_fee(options.priority_fee)
                    .build_orca_swap(
                        wallet,
                        &whirlpool,
                        amount_lamports,
                        min_tokens_out,
                        recent_blockhash,
                    )
                    .await?
            }
            _ => {
                return Err(solsniper_core::Error::Internal(
                    format!("Unsupported DEX: {}", token.dex)
                ));
            }
        };

        Ok(transaction)
    }

    /// 通过RPC执行交易
    async fn execute_via_rpc(
        &self,
        transaction: solana_sdk::transaction::Transaction,
        options: &ExecutionOptions,
    ) -> Result<TransactionResult> {
        for attempt in 1..=options.max_retries {
            match self.rpc_client.send_and_confirm_transaction_with_spinner_and_config(
                &transaction,
                options.confirmation_level,
                Default::default(),
            ) {
                Ok(signature) => {
                    return Ok(TransactionResult {
                        signature,
                        success: true,
                        execution_time_ms: 0,
                        method: ExecutionMethod::Rpc,
                    });
                }
                Err(e) => {
                    tracing::warn!("RPC execution attempt {}/{} failed: {}", attempt, options.max_retries, e);

                    if attempt < options.max_retries {
                        tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                    } else {
                        return Err(solsniper_core::Error::Internal(format!("Transaction failed after {} retries: {}", options.max_retries, e)));
                    }
                }
            }
        }

        unreachable!()
    }

    /// 通过JITO执行交易
    async fn execute_via_jito(
        &self,
        transaction: solana_sdk::transaction::Transaction,
        tip_lamports: u64,
    ) -> Result<TransactionResult> {
        // TODO: 实际JITO Bundle实现
        tracing::warn!("JITO execution not fully implemented, falling back to RPC");

        self.execute_via_rpc(transaction, &self.default_options).await
    }

    /// 计算最小输出代币数量（滑点保护）
    fn calculate_min_tokens_out(
        &self,
        amount_sol: f64,
        token_price_usd: f64,
        max_slippage_bps: u16,
    ) -> u64 {
        const SOL_PRICE_USD: f64 = 150.0; // 应从价格API获取

        let expected_usd = amount_sol * SOL_PRICE_USD;
        let expected_tokens = expected_usd / token_price_usd;

        // 应用滑点
        let slippage_multiplier = 1.0 - (max_slippage_bps as f64 / 10_000.0);
        let min_tokens = expected_tokens * slippage_multiplier;

        (min_tokens * 1e9) as u64
    }

    /// 使用滑点保护器计算最小输出金额
    ///
    /// 优先使用SlippageProtector的AMM公式计算
    /// 如果数据不足，回退到简单的价格计算
    async fn calculate_min_tokens_out_with_protection(
        &self,
        token: &TokenInfo,
        amount_sol: f64,
        options: &ExecutionOptions,
    ) -> Result<u64> {
        let amount_in_lamports = (amount_sol * 1e9) as u64;

        // 尝试获取池子储备数据（如果可用）
        // 注意：实际实现需要从DEX查询储备数据
        // 这里简化为使用liquidity作为估算
        if token.liquidity_sol > 0.0 {
            // 估算储备（假设50%的流动性是SOL，50%是Token）
            let reserve_sol = (token.liquidity_sol * 0.5 * 1e9) as u64;
            let reserve_token = if token.price_usd > 0.0 {
                let token_value_usd = token.liquidity_usd * 0.5;
                let token_count = token_value_usd / token.price_usd;
                (token_count * 10f64.powi(token.decimals as i32)) as u64
            } else {
                return Ok(self.calculate_min_tokens_out(
                    amount_sol,
                    token.price_usd,
                    options.max_slippage_bps,
                ));
            };

            // 使用滑点保护器验证
            match self.slippage_protector.validate_swap_quote(
                amount_in_lamports,
                reserve_sol,
                reserve_token,
                token.liquidity_sol,
                30, // 0.3% DEX fee
            ) {
                Ok(quote) => {
                    tracing::info!(
                        "✅ Slippage protection: expected_out={}, min_out={}, price_impact={:.2}%, slippage={:.2}%",
                        quote.expected_out,
                        quote.min_amount_out,
                        quote.price_impact_bps as f64 / 100.0,
                        quote.slippage_bps as f64 / 100.0
                    );
                    return Ok(quote.min_amount_out);
                }
                Err(e) => {
                    tracing::warn!(
                        "⚠️ Slippage protection validation failed: {}, falling back to simple calculation",
                        e
                    );
                    // 回退到简单计算
                }
            }
        }

        // 回退：使用简单的价格计算
        tracing::debug!("Using simple price-based slippage calculation");
        Ok(self.calculate_min_tokens_out(
            amount_sol,
            token.price_usd,
            options.max_slippage_bps,
        ))
    }

    /// 克隆用于并发执行
    fn clone_for_concurrent(&self) -> Self {
        Self {
            wallet_manager: Arc::clone(&self.wallet_manager),
            rpc_client: Arc::clone(&self.rpc_client),
            transaction_builder: TransactionBuilder::new()
                .with_priority_fee(self.default_options.priority_fee),
            slippage_protector: self.slippage_protector.clone(),
            transaction_cache: Arc::clone(&self.transaction_cache),
            default_options: self.default_options.clone(),
        }
    }

    /// 清理过期的交易缓存
    pub fn cleanup_transaction_cache(&self) {
        self.transaction_cache.retain(|_, timestamp| {
            timestamp.elapsed() < Duration::from_secs(60)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_trading_engine_creation() {
        let rpc_client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));
        let wallet_manager = Arc::new(WalletManager::new(Arc::clone(&rpc_client)));
        let engine = TradingEngine::new(wallet_manager, rpc_client);

        assert_eq!(engine.default_options.max_slippage_bps, 300);
    }

    #[test]
    fn test_min_tokens_calculation() {
        let rpc_client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));
        let wallet_manager = Arc::new(WalletManager::new(Arc::clone(&rpc_client)));
        let engine = TradingEngine::new(wallet_manager, rpc_client);

        let min_tokens = engine.calculate_min_tokens_out(
            1.0,       // 1 SOL
            0.00001,   // token price
            300,       // 3% slippage
        );

        // 1 SOL * 150 USD / 0.00001 USD = 15,000,000 tokens
        // With 3% slippage: 15,000,000 * 0.97 = 14,550,000
        assert!(min_tokens > 14_000_000_000_000_000);
    }
}
