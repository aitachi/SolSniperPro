use crate::{Error, Result, TokenInfo};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// 风险控制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskControlConfig {
    /// 单次最大交易金额（SOL）
    pub max_position_size_sol: f64,

    /// 单日最大交易金额（SOL）
    pub max_daily_volume_sol: f64,

    /// 单日最大交易次数
    pub max_daily_trades: u32,

    /// 单日最大亏损（SOL）
    pub max_daily_loss_sol: f64,

    /// 最大持仓数量
    pub max_concurrent_positions: u32,

    /// 单个代币最大仓位（占总资金比例）
    pub max_position_pct: f64,

    /// 冷却期（秒）- 交易失败后等待时间
    pub cooldown_after_loss_secs: u64,

    /// 最小代币流动性（SOL）
    pub min_token_liquidity_sol: f64,

    /// 最大代币集中度（top10持有比例）
    pub max_token_concentration: f64,

    /// 最小代币持有人数
    pub min_token_holders: u32,

    /// 启用黑名单检查
    pub enable_blacklist: bool,

    /// 启用白名单模式（仅允许白名单代币）
    pub enable_whitelist_only: bool,
}

impl Default for RiskControlConfig {
    fn default() -> Self {
        Self {
            max_position_size_sol: 10.0,
            max_daily_volume_sol: 100.0,
            max_daily_trades: 50,
            max_daily_loss_sol: 20.0,
            max_concurrent_positions: 10,
            max_position_pct: 0.2, // 20%
            cooldown_after_loss_secs: 300, // 5分钟
            min_token_liquidity_sol: 10.0,
            max_token_concentration: 0.6, // 60%
            min_token_holders: 100,
            enable_blacklist: true,
            enable_whitelist_only: false,
        }
    }
}

/// 交易记录
#[derive(Debug, Clone)]
struct TradeRecord {
    timestamp: Instant,
    token: Pubkey,
    amount_sol: f64,
    is_buy: bool,
    profit_loss: Option<f64>, // 仅卖出时有值
}

/// 持仓信息
#[derive(Debug, Clone)]
pub struct Position {
    pub token: Pubkey,
    pub amount_tokens: u64,
    pub cost_sol: f64,
    pub entry_price: f64,
    pub entry_time: Instant,
}

/// 风险检查结果
#[derive(Debug, Clone)]
pub struct RiskCheckResult {
    pub approved: bool,
    pub reasons: Vec<String>,
    pub warnings: Vec<String>,
}

impl RiskCheckResult {
    fn approved() -> Self {
        Self {
            approved: true,
            reasons: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn rejected(reason: String) -> Self {
        Self {
            approved: false,
            reasons: vec![reason],
            warnings: Vec::new(),
        }
    }

    fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// 风险控制器
///
/// 提供交易前风险检查，防止过度交易和高风险操作
///
/// # 功能
/// - 仓位限制检查
/// - 日交易量限制
/// - 日交易次数限制
/// - 日亏损限制
/// - 冷却期管理
/// - 代币黑白名单
/// - 代币质量检查
pub struct RiskController {
    config: Arc<RwLock<RiskControlConfig>>,

    /// 当前持仓
    positions: Arc<DashMap<Pubkey, Position>>,

    /// 今日交易记录
    today_trades: Arc<RwLock<Vec<TradeRecord>>>,

    /// 黑名单
    blacklist: Arc<RwLock<HashSet<Pubkey>>>,

    /// 白名单
    whitelist: Arc<RwLock<HashSet<Pubkey>>>,

    /// 上次交易失败时间
    last_loss_time: Arc<RwLock<Option<Instant>>>,

    /// 统计信息
    stats: Arc<RwLock<RiskStats>>,
}

/// 风险统计
#[derive(Debug, Clone, Default)]
pub struct RiskStats {
    pub total_checks: u64,
    pub approved_checks: u64,
    pub rejected_checks: u64,
    pub blacklist_rejections: u64,
    pub position_limit_rejections: u64,
    pub daily_limit_rejections: u64,
    pub quality_rejections: u64,
}

impl RiskStats {
    pub fn approval_rate(&self) -> f64 {
        if self.total_checks == 0 {
            return 0.0;
        }
        self.approved_checks as f64 / self.total_checks as f64
    }
}

impl RiskController {
    /// 创建新的风险控制器
    pub fn new(config: RiskControlConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            positions: Arc::new(DashMap::new()),
            today_trades: Arc::new(RwLock::new(Vec::new())),
            blacklist: Arc::new(RwLock::new(HashSet::new())),
            whitelist: Arc::new(RwLock::new(HashSet::new())),
            last_loss_time: Arc::new(RwLock::new(None)),
            stats: Arc::new(RwLock::new(RiskStats::default())),
        }
    }

    /// 创建默认配置的控制器
    pub fn default() -> Self {
        Self::new(RiskControlConfig::default())
    }

    /// 检查买入交易
    ///
    /// # 参数
    /// - `token`: 代币信息
    /// - `amount_sol`: 买入金额
    /// - `total_balance_sol`: 总资金（用于计算仓位比例）
    pub async fn check_buy(
        &self,
        token: &TokenInfo,
        amount_sol: f64,
        total_balance_sol: f64,
    ) -> Result<RiskCheckResult> {
        let config = self.config.read().await;
        let mut stats = self.stats.write().await;
        stats.total_checks += 1;

        // 1. 检查冷却期
        if let Some(reason) = self.check_cooldown(&config).await {
            stats.rejected_checks += 1;
            return Ok(RiskCheckResult::rejected(reason));
        }

        // 2. 检查黑白名单
        if let Some(reason) = self.check_blacklist_whitelist(token, &config).await {
            stats.rejected_checks += 1;
            stats.blacklist_rejections += 1;
            return Ok(RiskCheckResult::rejected(reason));
        }

        // 3. 检查单次仓位限制
        if amount_sol > config.max_position_size_sol {
            stats.rejected_checks += 1;
            stats.position_limit_rejections += 1;
            return Ok(RiskCheckResult::rejected(format!(
                "Position size {:.2} SOL exceeds max {:.2} SOL",
                amount_sol, config.max_position_size_sol
            )));
        }

        // 4. 检查仓位比例
        let position_pct = amount_sol / total_balance_sol;
        if position_pct > config.max_position_pct {
            stats.rejected_checks += 1;
            stats.position_limit_rejections += 1;
            return Ok(RiskCheckResult::rejected(format!(
                "Position percentage {:.1}% exceeds max {:.1}%",
                position_pct * 100.0,
                config.max_position_pct * 100.0
            )));
        }

        // 5. 检查持仓数量
        if self.positions.len() >= config.max_concurrent_positions as usize {
            stats.rejected_checks += 1;
            stats.position_limit_rejections += 1;
            return Ok(RiskCheckResult::rejected(format!(
                "Max concurrent positions {} reached",
                config.max_concurrent_positions
            )));
        }

        // 6. 检查日交易量限制
        if let Some(reason) = self.check_daily_limits(&config, amount_sol).await {
            stats.rejected_checks += 1;
            stats.daily_limit_rejections += 1;
            return Ok(RiskCheckResult::rejected(reason));
        }

        // 7. 检查代币质量
        let mut result = RiskCheckResult::approved();
        if let Some(warning) = self.check_token_quality(token, &config) {
            // 质量问题作为警告，不直接拒绝
            result = result.with_warning(warning);
        }

        stats.approved_checks += 1;
        Ok(result)
    }

    /// 检查卖出交易
    pub async fn check_sell(&self, token: &Pubkey, amount_tokens: u64) -> Result<RiskCheckResult> {
        let mut stats = self.stats.write().await;
        stats.total_checks += 1;

        // 检查是否持有该仓位
        if !self.positions.contains_key(token) {
            stats.rejected_checks += 1;
            return Ok(RiskCheckResult::rejected(format!(
                "No position found for token {}",
                token
            )));
        }

        let position = self.positions.get(token).unwrap();
        if amount_tokens > position.amount_tokens {
            stats.rejected_checks += 1;
            return Ok(RiskCheckResult::rejected(format!(
                "Sell amount {} exceeds position {}",
                amount_tokens, position.amount_tokens
            )));
        }

        stats.approved_checks += 1;
        Ok(RiskCheckResult::approved())
    }

    /// 记录买入交易
    pub async fn record_buy(&self, token: Pubkey, amount_sol: f64, amount_tokens: u64, price: f64) {
        // 记录交易
        let trade = TradeRecord {
            timestamp: Instant::now(),
            token,
            amount_sol,
            is_buy: true,
            profit_loss: None,
        };
        self.today_trades.write().await.push(trade);

        // 更新持仓
        let position = Position {
            token,
            amount_tokens,
            cost_sol: amount_sol,
            entry_price: price,
            entry_time: Instant::now(),
        };
        self.positions.insert(token, position);

        tracing::info!(
            "📈 Position opened: {} tokens of {} for {:.4} SOL @ {:.8}",
            amount_tokens,
            token,
            amount_sol,
            price
        );
    }

    /// 记录卖出交易
    pub async fn record_sell(
        &self,
        token: Pubkey,
        amount_sol: f64,
        amount_tokens: u64,
        current_price: f64,
    ) {
        let profit_loss = if let Some(position) = self.positions.get(&token) {
            // 计算盈亏
            let cost_basis = (position.cost_sol / position.amount_tokens as f64) * amount_tokens as f64;
            Some(amount_sol - cost_basis)
        } else {
            None
        };

        // 记录交易
        let trade = TradeRecord {
            timestamp: Instant::now(),
            token,
            amount_sol,
            is_buy: false,
            profit_loss,
        };
        self.today_trades.write().await.push(trade);

        // 更新持仓
        if let Some(mut position) = self.positions.get_mut(&token) {
            if amount_tokens >= position.amount_tokens {
                // 全部卖出，移除持仓
                drop(position);
                self.positions.remove(&token);
                tracing::info!("📉 Position closed: {} for {:.4} SOL", token, amount_sol);
            } else {
                // 部分卖出，更新持仓
                let remaining_tokens = position.amount_tokens - amount_tokens;
                let remaining_cost = position.cost_sol * (remaining_tokens as f64 / position.amount_tokens as f64);
                position.amount_tokens = remaining_tokens;
                position.cost_sol = remaining_cost;
                tracing::info!(
                    "📉 Position reduced: {} tokens sold for {:.4} SOL ({} remaining)",
                    amount_tokens,
                    amount_sol,
                    remaining_tokens
                );
            }
        }

        // 如果亏损，更新冷却时间
        if let Some(pnl) = profit_loss {
            if pnl < 0.0 {
                *self.last_loss_time.write().await = Some(Instant::now());
                tracing::warn!("📉 Loss recorded: {:.4} SOL", pnl.abs());
            } else {
                tracing::info!("📈 Profit recorded: {:.4} SOL", pnl);
            }
        }
    }

    /// 检查冷却期
    async fn check_cooldown(&self, config: &RiskControlConfig) -> Option<String> {
        if let Some(last_loss) = *self.last_loss_time.read().await {
            let elapsed = last_loss.elapsed().as_secs();
            if elapsed < config.cooldown_after_loss_secs {
                let remaining = config.cooldown_after_loss_secs - elapsed;
                return Some(format!(
                    "Cooldown period active: {} seconds remaining",
                    remaining
                ));
            }
        }
        None
    }

    /// 检查黑白名单
    async fn check_blacklist_whitelist(
        &self,
        token: &TokenInfo,
        config: &RiskControlConfig,
    ) -> Option<String> {
        // 黑名单检查
        if config.enable_blacklist && self.blacklist.read().await.contains(&token.mint) {
            return Some(format!("Token {} is blacklisted", token.mint));
        }

        // 白名单检查
        if config.enable_whitelist_only && !self.whitelist.read().await.contains(&token.mint) {
            return Some(format!("Token {} is not whitelisted", token.mint));
        }

        None
    }

    /// 检查日交易限制
    async fn check_daily_limits(
        &self,
        config: &RiskControlConfig,
        new_amount_sol: f64,
    ) -> Option<String> {
        let trades = self.today_trades.read().await;

        // 检查日交易次数
        if trades.len() >= config.max_daily_trades as usize {
            return Some(format!(
                "Daily trade limit {} reached",
                config.max_daily_trades
            ));
        }

        // 检查日交易量
        let today_volume: f64 = trades.iter().filter(|t| t.is_buy).map(|t| t.amount_sol).sum();
        if today_volume + new_amount_sol > config.max_daily_volume_sol {
            return Some(format!(
                "Daily volume limit {:.2} SOL would be exceeded (current: {:.2} SOL)",
                config.max_daily_volume_sol, today_volume
            ));
        }

        // 检查日亏损
        let today_loss: f64 = trades
            .iter()
            .filter_map(|t| t.profit_loss)
            .filter(|&pnl| pnl < 0.0)
            .map(|pnl| pnl.abs())
            .sum();

        if today_loss >= config.max_daily_loss_sol {
            return Some(format!(
                "Daily loss limit {:.2} SOL reached (current loss: {:.2} SOL)",
                config.max_daily_loss_sol, today_loss
            ));
        }

        None
    }

    /// 检查代币质量
    fn check_token_quality(
        &self,
        token: &TokenInfo,
        config: &RiskControlConfig,
    ) -> Option<String> {
        let mut issues = Vec::new();

        // 流动性检查
        if token.liquidity_sol < config.min_token_liquidity_sol {
            issues.push(format!(
                "Low liquidity: {:.2} SOL (min: {:.2} SOL)",
                token.liquidity_sol, config.min_token_liquidity_sol
            ));
        }

        // 集中度检查
        if token.top10_ratio > config.max_token_concentration {
            issues.push(format!(
                "High concentration: {:.1}% top10 (max: {:.1}%)",
                token.top10_ratio * 100.0,
                config.max_token_concentration * 100.0
            ));
        }

        // 持有人数检查
        if token.holders_count < config.min_token_holders {
            issues.push(format!(
                "Low holder count: {} (min: {})",
                token.holders_count, config.min_token_holders
            ));
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues.join("; "))
        }
    }

    /// 添加到黑名单
    pub async fn add_to_blacklist(&self, token: Pubkey) {
        self.blacklist.write().await.insert(token);
        tracing::warn!("🚫 Token {} added to blacklist", token);
    }

    /// 从黑名单移除
    pub async fn remove_from_blacklist(&self, token: &Pubkey) {
        self.blacklist.write().await.remove(token);
        tracing::info!("✅ Token {} removed from blacklist", token);
    }

    /// 添加到白名单
    pub async fn add_to_whitelist(&self, token: Pubkey) {
        self.whitelist.write().await.insert(token);
        tracing::info!("✅ Token {} added to whitelist", token);
    }

    /// 获取当前持仓
    pub fn get_positions(&self) -> Vec<Position> {
        self.positions.iter().map(|entry| entry.value().clone()).collect()
    }

    /// 获取今日统计
    pub async fn get_daily_stats(&self) -> DailyStats {
        let trades = self.today_trades.read().await;

        let total_trades = trades.len();
        let buy_count = trades.iter().filter(|t| t.is_buy).count();
        let sell_count = total_trades - buy_count;

        let total_volume: f64 = trades.iter().filter(|t| t.is_buy).map(|t| t.amount_sol).sum();

        let (total_profit, total_loss) = trades.iter().filter_map(|t| t.profit_loss).fold(
            (0.0, 0.0),
            |(profit, loss), pnl| {
                if pnl > 0.0 {
                    (profit + pnl, loss)
                } else {
                    (profit, loss + pnl.abs())
                }
            },
        );

        DailyStats {
            total_trades,
            buy_count,
            sell_count,
            total_volume_sol: total_volume,
            total_profit_sol: total_profit,
            total_loss_sol: total_loss,
            net_pnl_sol: total_profit - total_loss,
        }
    }

    /// 获取风险统计
    pub async fn get_risk_stats(&self) -> RiskStats {
        self.stats.read().await.clone()
    }

    /// 重置日统计（每日凌晨调用）
    pub async fn reset_daily_stats(&self) {
        self.today_trades.write().await.clear();
        tracing::info!("📊 Daily stats reset");
    }

    /// 更新配置
    pub async fn update_config(&self, config: RiskControlConfig) {
        *self.config.write().await = config;
        tracing::info!("⚙️ Risk control config updated");
    }

    /// 启动每日重置任务
    pub fn spawn_daily_reset_task(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                // 等待到下一个午夜
                let now = chrono::Local::now();
                let tomorrow = now.date_naive().succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap();
                let duration_until_midnight = (tomorrow - now.naive_local()).to_std().unwrap();

                tokio::time::sleep(duration_until_midnight).await;

                // 重置统计
                self.reset_daily_stats().await;
            }
        })
    }
}

/// 日统计
#[derive(Debug, Clone)]
pub struct DailyStats {
    pub total_trades: usize,
    pub buy_count: usize,
    pub sell_count: usize,
    pub total_volume_sol: f64,
    pub total_profit_sol: f64,
    pub total_loss_sol: f64,
    pub net_pnl_sol: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    fn create_test_token() -> TokenInfo {
        TokenInfo {
            mint: Pubkey::new_unique(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            total_supply: 1_000_000,
            circulating_supply: 800_000,
            price_usd: 0.001,
            market_cap_usd: 800.0,
            liquidity_sol: 50.0,
            liquidity_usd: 7500.0,
            volume_24h: 1000.0,
            volume_1h: 100.0,
            volume_6h: 500.0,
            price_change_1h: 5.0,
            price_change_6h: 10.0,
            price_change_24h: 20.0,
            holders_count: 500,
            top10_ratio: 0.3,
            top20_ratio: 0.45,
            top50_ratio: 0.65,
            dex: "Raydium".to_string(),
            pool_address: Some(Pubkey::new_unique()),
            creator: Some(Pubkey::new_unique()),
            age_minutes: 60.0,
            age_hours: 1.0,
            buy_tax: 0.0,
            sell_tax: 0.0,
            is_renounced: true,
            is_frozen: false,
            txns_1h_buys: 50,
            txns_1h_sells: 30,
            txns_1h_total: 80,
            txns_6h_buys: 200,
            txns_6h_sells: 150,
            txns_6h_total: 350,
            volatility_1h: 0.05,
            volatility_6h: 0.08,
            social_twitter: Some("@test".to_string()),
            social_telegram: None,
            social_website: None,
            discovered_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_risk_controller_creation() {
        let controller = RiskController::default();
        let config = controller.config.read().await;

        assert_eq!(config.max_position_size_sol, 10.0);
        assert_eq!(config.max_daily_trades, 50);
    }

    #[tokio::test]
    async fn test_buy_check_approved() {
        let controller = RiskController::default();
        let token = create_test_token();

        let result = controller.check_buy(&token, 5.0, 100.0).await.unwrap();
        assert!(result.approved);
    }

    #[tokio::test]
    async fn test_position_size_limit() {
        let controller = RiskController::default();
        let token = create_test_token();

        let result = controller.check_buy(&token, 15.0, 100.0).await.unwrap();
        assert!(!result.approved);
        assert!(result.reasons[0].contains("exceeds max"));
    }

    #[tokio::test]
    async fn test_position_percentage_limit() {
        let controller = RiskController::default();
        let token = create_test_token();

        // 25% of 100 SOL = 25 SOL, exceeds 20% limit
        let result = controller.check_buy(&token, 25.0, 100.0).await.unwrap();
        assert!(!result.approved);
    }

    #[tokio::test]
    async fn test_blacklist() {
        let controller = RiskController::default();
        let mut token = create_test_token();

        // Add to blacklist
        controller.add_to_blacklist(token.mint).await;

        let result = controller.check_buy(&token, 5.0, 100.0).await.unwrap();
        assert!(!result.approved);
        assert!(result.reasons[0].contains("blacklisted"));
    }

    #[tokio::test]
    async fn test_record_buy_and_sell() {
        let controller = RiskController::default();
        let token = Pubkey::new_unique();

        // Record buy
        controller.record_buy(token, 5.0, 1000, 0.005).await;
        assert_eq!(controller.positions.len(), 1);

        // Record partial sell
        controller.record_sell(token, 3.0, 600, 0.005).await;
        assert_eq!(controller.positions.len(), 1);
        assert_eq!(controller.positions.get(&token).unwrap().amount_tokens, 400);

        // Record full sell
        controller.record_sell(token, 2.0, 400, 0.005).await;
        assert_eq!(controller.positions.len(), 0);
    }

    #[tokio::test]
    async fn test_daily_stats() {
        let controller = RiskController::default();
        let token = create_test_token();

        // Simulate trades
        controller.record_buy(token.mint, 5.0, 1000, 0.005).await;
        controller.record_sell(token.mint, 6.0, 1000, 0.006).await;

        let stats = controller.get_daily_stats().await;
        assert_eq!(stats.total_trades, 2);
        assert_eq!(stats.buy_count, 1);
        assert_eq!(stats.sell_count, 1);
        assert_eq!(stats.total_volume_sol, 5.0);
        assert!(stats.net_pnl_sol > 0.0); // Profit
    }
}
