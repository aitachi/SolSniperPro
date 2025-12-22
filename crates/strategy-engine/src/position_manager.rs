use solsniper_core::{Error, Result, TokenInfo};
use serde::{Deserialize, Serialize};

/// 仓位管理策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionSizingStrategy {
    /// 固定金额
    FixedAmount,
    /// 固定比例
    FixedPercentage,
    /// 基于波动性（ATR）
    VolatilityBased,
    /// 凯利公式
    KellyCriterion,
    /// 风险平价
    RiskParity,
    /// 马丁格尔（加倍）
    Martingale,
    /// 反马丁格尔（减半）
    AntiMartingale,
}

/// 仓位管理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionManagerConfig {
    /// 仓位管理策略
    pub strategy: PositionSizingStrategy,

    /// 固定金额（SOL）- 用于FixedAmount策略
    pub fixed_amount_sol: f64,

    /// 固定比例（0-1）- 用于FixedPercentage策略
    pub fixed_percentage: f64,

    /// 最大单次仓位（SOL）
    pub max_position_size_sol: f64,

    /// 最小单次仓位（SOL）
    pub min_position_size_sol: f64,

    /// 目标波动率（用于VolatilityBased）
    pub target_volatility: f64,

    /// 凯利分数（0-1，建议0.25，即1/4凯利）
    pub kelly_fraction: f64,

    /// 马丁格尔倍数（默认2.0）
    pub martingale_multiplier: f64,

    /// 马丁格尔最大倍数
    pub max_martingale_steps: u32,

    /// 风险因子（用于风险调整）
    pub risk_factor: f64,
}

impl Default for PositionManagerConfig {
    fn default() -> Self {
        Self {
            strategy: PositionSizingStrategy::FixedPercentage,
            fixed_amount_sol: 1.0,
            fixed_percentage: 0.1, // 10%
            max_position_size_sol: 10.0,
            min_position_size_sol: 0.1,
            target_volatility: 0.02, // 2% daily volatility
            kelly_fraction: 0.25, // 1/4 Kelly
            martingale_multiplier: 2.0,
            max_martingale_steps: 3,
            risk_factor: 1.0,
        }
    }
}

/// 仓位计算结果
#[derive(Debug, Clone)]
pub struct PositionSize {
    /// 推荐仓位大小（SOL）
    pub amount_sol: f64,

    /// 使用的策略
    pub strategy_used: PositionSizingStrategy,

    /// 风险调整后的仓位
    pub risk_adjusted_amount: f64,

    /// 调整原因
    pub adjustment_reasons: Vec<String>,

    /// 置信度（0-1）
    pub confidence: f64,
}

/// 交易历史记录
#[derive(Debug, Clone)]
pub struct TradeHistory {
    pub wins: u32,
    pub losses: u32,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub consecutive_losses: u32,
}

impl TradeHistory {
    pub fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            avg_win: 0.0,
            avg_loss: 0.0,
            consecutive_losses: 0,
        }
    }

    pub fn win_rate(&self) -> f64 {
        let total = self.wins + self.losses;
        if total == 0 {
            return 0.0;
        }
        self.wins as f64 / total as f64
    }

    pub fn total_trades(&self) -> u32 {
        self.wins + self.losses
    }

    pub fn profit_factor(&self) -> f64 {
        if self.avg_loss == 0.0 {
            return 0.0;
        }
        (self.wins as f64 * self.avg_win) / (self.losses as f64 * self.avg_loss.abs())
    }
}

/// 动态仓位管理器
///
/// 根据账户状态、市场条件和历史表现动态调整仓位大小
///
/// # 策略
/// - FixedAmount: 固定金额
/// - FixedPercentage: 固定比例
/// - VolatilityBased: 基于波动性调整
/// - KellyCriterion: 凯利公式优化
/// - RiskParity: 风险平价
/// - Martingale: 马丁格尔（亏损加倍）
/// - AntiMartingale: 反马丁格尔（盈利加倍）
pub struct PositionManager {
    config: PositionManagerConfig,
    trade_history: TradeHistory,
}

impl PositionManager {
    /// 创建新的仓位管理器
    pub fn new(config: PositionManagerConfig) -> Self {
        Self {
            config,
            trade_history: TradeHistory::new(),
        }
    }

    /// 创建默认配置的管理器
    pub fn default() -> Self {
        Self::new(PositionManagerConfig::default())
    }

    /// 计算推荐仓位大小
    ///
    /// # 参数
    /// - `account_balance`: 账户总余额（SOL）
    /// - `token`: 代币信息
    /// - `risk_score`: 风险评分（0-100，越高越安全）
    pub fn calculate_position_size(
        &self,
        account_balance: f64,
        token: &TokenInfo,
        risk_score: f64,
    ) -> Result<PositionSize> {
        if account_balance <= 0.0 {
            return Err(Error::Internal("Account balance must be positive".to_string()));
        }

        let mut adjustment_reasons = Vec::new();

        // 1. 根据策略计算基础仓位
        let base_amount = match self.config.strategy {
            PositionSizingStrategy::FixedAmount => {
                self.calculate_fixed_amount()
            }
            PositionSizingStrategy::FixedPercentage => {
                self.calculate_fixed_percentage(account_balance)
            }
            PositionSizingStrategy::VolatilityBased => {
                self.calculate_volatility_based(account_balance, token)
            }
            PositionSizingStrategy::KellyCriterion => {
                self.calculate_kelly_criterion(account_balance)
            }
            PositionSizingStrategy::RiskParity => {
                self.calculate_risk_parity(account_balance, token)
            }
            PositionSizingStrategy::Martingale => {
                self.calculate_martingale(account_balance)
            }
            PositionSizingStrategy::AntiMartingale => {
                self.calculate_anti_martingale(account_balance)
            }
        };

        // 2. 应用风险调整
        let risk_adjusted = self.apply_risk_adjustment(base_amount, risk_score);
        if risk_adjusted < base_amount {
            adjustment_reasons.push(format!(
                "Risk adjusted: {:.4} -> {:.4} SOL (risk_score: {:.0})",
                base_amount, risk_adjusted, risk_score
            ));
        }

        // 3. 应用限制
        let final_amount = self.apply_limits(risk_adjusted);
        if final_amount != risk_adjusted {
            adjustment_reasons.push(format!(
                "Applied limits: {:.4} -> {:.4} SOL",
                risk_adjusted, final_amount
            ));
        }

        // 4. 计算置信度
        let confidence = self.calculate_confidence(risk_score);

        Ok(PositionSize {
            amount_sol: final_amount,
            strategy_used: self.config.strategy,
            risk_adjusted_amount: risk_adjusted,
            adjustment_reasons,
            confidence,
        })
    }

    /// 固定金额策略
    fn calculate_fixed_amount(&self) -> f64 {
        self.config.fixed_amount_sol
    }

    /// 固定比例策略
    fn calculate_fixed_percentage(&self, account_balance: f64) -> f64 {
        account_balance * self.config.fixed_percentage
    }

    /// 基于波动性的仓位
    ///
    /// 公式: position = (target_volatility / token_volatility) * account_balance
    fn calculate_volatility_based(&self, account_balance: f64, token: &TokenInfo) -> f64 {
        let token_volatility = token.volatility_1h.max(0.01); // 避免除零
        let volatility_ratio = self.config.target_volatility / token_volatility;

        // 限制比例在合理范围内
        let clamped_ratio = volatility_ratio.clamp(0.1, 2.0);

        account_balance * self.config.fixed_percentage * clamped_ratio
    }

    /// 凯利公式
    ///
    /// Kelly% = W - [(1 - W) / R]
    /// 其中 W = 胜率, R = 平均盈利/平均亏损比
    fn calculate_kelly_criterion(&self, account_balance: f64) -> f64 {
        if self.trade_history.total_trades() < 10 {
            // 交易历史不足，使用保守策略
            return self.calculate_fixed_percentage(account_balance);
        }

        let win_rate = self.trade_history.win_rate();
        let loss_rate = 1.0 - win_rate;

        if self.trade_history.avg_loss == 0.0 {
            return self.calculate_fixed_percentage(account_balance);
        }

        let win_loss_ratio = self.trade_history.avg_win / self.trade_history.avg_loss.abs();

        // Kelly公式
        let kelly_percentage = win_rate - (loss_rate / win_loss_ratio);

        // 使用分数凯利（更保守）
        let fractional_kelly = kelly_percentage * self.config.kelly_fraction;

        // 确保非负且合理
        let safe_kelly = fractional_kelly.max(0.0).min(0.5);

        account_balance * safe_kelly
    }

    /// 风险平价策略
    ///
    /// 根据波动性分配仓位，使每个仓位的风险贡献相等
    fn calculate_risk_parity(&self, account_balance: f64, token: &TokenInfo) -> f64 {
        let token_volatility = token.volatility_1h.max(0.01);

        // 假设基准波动率为2%
        let base_volatility = 0.02;
        let volatility_ratio = base_volatility / token_volatility;

        // 风险平价仓位
        let risk_parity_pct = self.config.fixed_percentage * volatility_ratio;

        // 限制范围
        let clamped_pct = risk_parity_pct.clamp(0.01, 0.3);

        account_balance * clamped_pct
    }

    /// 马丁格尔策略（亏损加倍）
    fn calculate_martingale(&self, account_balance: f64) -> f64 {
        let base_amount = self.calculate_fixed_percentage(account_balance);

        if self.trade_history.consecutive_losses == 0 {
            return base_amount;
        }

        // 限制最大步数
        let steps = self.trade_history.consecutive_losses.min(self.config.max_martingale_steps);

        // 加倍
        let multiplier = self.config.martingale_multiplier.powi(steps as i32);

        base_amount * multiplier
    }

    /// 反马丁格尔策略（盈利加倍）
    fn calculate_anti_martingale(&self, account_balance: f64) -> f64 {
        let base_amount = self.calculate_fixed_percentage(account_balance);

        // 如果有连续亏损，减小仓位
        if self.trade_history.consecutive_losses > 0 {
            let reduction_factor = 0.5_f64.powi(self.trade_history.consecutive_losses as i32);
            return base_amount * reduction_factor;
        }

        // 如果没有连续亏损，使用基础仓位
        base_amount
    }

    /// 应用风险调整
    ///
    /// 根据风险评分调整仓位大小
    /// risk_score: 0-100, 越高越安全
    fn apply_risk_adjustment(&self, base_amount: f64, risk_score: f64) -> f64 {
        // 将风险评分转换为调整因子 (0.5 - 1.5)
        let risk_multiplier = if risk_score >= 70.0 {
            // 高风险评分（安全）: 1.0 - 1.5
            1.0 + (risk_score - 70.0) / 60.0
        } else if risk_score >= 50.0 {
            // 中等风险评分: 0.8 - 1.0
            0.8 + (risk_score - 50.0) / 100.0
        } else {
            // 低风险评分（危险）: 0.3 - 0.8
            0.3 + risk_score / 100.0
        };

        base_amount * risk_multiplier * self.config.risk_factor
    }

    /// 应用限制
    fn apply_limits(&self, amount: f64) -> f64 {
        amount
            .max(self.config.min_position_size_sol)
            .min(self.config.max_position_size_sol)
    }

    /// 计算置信度
    fn calculate_confidence(&self, risk_score: f64) -> f64 {
        // 基于风险评分和交易历史计算置信度
        let risk_confidence = risk_score / 100.0;

        let history_confidence = if self.trade_history.total_trades() >= 20 {
            let win_rate = self.trade_history.win_rate();
            let profit_factor = self.trade_history.profit_factor();

            // 综合胜率和盈亏比
            (win_rate * 0.6 + profit_factor.min(2.0) / 2.0 * 0.4).min(1.0)
        } else {
            0.5 // 历史不足，中等置信度
        };

        // 综合置信度
        (risk_confidence * 0.7 + history_confidence * 0.3).min(1.0)
    }

    /// 记录交易结果
    pub fn record_trade(&mut self, profit_loss: f64) {
        if profit_loss > 0.0 {
            // 盈利
            self.trade_history.wins += 1;

            // 更新平均盈利（移动平均）
            if self.trade_history.wins == 1 {
                self.trade_history.avg_win = profit_loss;
            } else {
                self.trade_history.avg_win =
                    (self.trade_history.avg_win * (self.trade_history.wins - 1) as f64
                    + profit_loss) / self.trade_history.wins as f64;
            }

            // 重置连续亏损
            self.trade_history.consecutive_losses = 0;
        } else {
            // 亏损
            self.trade_history.losses += 1;

            // 更新平均亏损
            if self.trade_history.losses == 1 {
                self.trade_history.avg_loss = profit_loss;
            } else {
                self.trade_history.avg_loss =
                    (self.trade_history.avg_loss * (self.trade_history.losses - 1) as f64
                    + profit_loss) / self.trade_history.losses as f64;
            }

            // 增加连续亏损
            self.trade_history.consecutive_losses += 1;
        }

        tracing::info!(
            "📊 Trade recorded: PnL={:.4} SOL, Win rate: {:.1}%, Consecutive losses: {}",
            profit_loss,
            self.trade_history.win_rate() * 100.0,
            self.trade_history.consecutive_losses
        );
    }

    /// 获取交易统计
    pub fn get_statistics(&self) -> TradeHistory {
        self.trade_history.clone()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: PositionManagerConfig) {
        self.config = config;
        tracing::info!("⚙️ Position manager config updated: strategy={:?}", self.config.strategy);
    }

    /// 重置交易历史
    pub fn reset_history(&mut self) {
        self.trade_history = TradeHistory::new();
        tracing::info!("🔄 Trade history reset");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_token() -> TokenInfo {
        TokenInfo {
            mint: solana_sdk::pubkey::Pubkey::new_unique(),
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
            pool_address: Some(solana_sdk::pubkey::Pubkey::new_unique()),
            creator: Some(solana_sdk::pubkey::Pubkey::new_unique()),
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
            discovered_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_fixed_amount_strategy() {
        let config = PositionManagerConfig {
            strategy: PositionSizingStrategy::FixedAmount,
            fixed_amount_sol: 2.0,
            ..Default::default()
        };

        let manager = PositionManager::new(config);
        let token = create_test_token();

        let position = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        assert_eq!(position.strategy_used, PositionSizingStrategy::FixedAmount);
        assert!(position.amount_sol >= 2.0); // May be adjusted by risk
    }

    #[test]
    fn test_fixed_percentage_strategy() {
        let config = PositionManagerConfig {
            strategy: PositionSizingStrategy::FixedPercentage,
            fixed_percentage: 0.1, // 10%
            ..Default::default()
        };

        let manager = PositionManager::new(config);
        let token = create_test_token();

        let position = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        assert_eq!(position.strategy_used, PositionSizingStrategy::FixedPercentage);
        // Should be around 10 SOL (10% of 100), possibly adjusted
        assert!(position.amount_sol >= 8.0 && position.amount_sol <= 12.0);
    }

    #[test]
    fn test_volatility_based_strategy() {
        let config = PositionManagerConfig {
            strategy: PositionSizingStrategy::VolatilityBased,
            target_volatility: 0.02,
            fixed_percentage: 0.1,
            ..Default::default()
        };

        let manager = PositionManager::new(config);
        let mut token = create_test_token();

        // High volatility token
        token.volatility_1h = 0.10;
        let position_high_vol = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // Low volatility token
        token.volatility_1h = 0.01;
        let position_low_vol = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // Low volatility should get larger position
        assert!(position_low_vol.amount_sol > position_high_vol.amount_sol);
    }

    #[test]
    fn test_risk_adjustment() {
        let manager = PositionManager::default();
        let token = create_test_token();

        // High risk score (safe token)
        let position_safe = manager.calculate_position_size(100.0, &token, 90.0).unwrap();

        // Low risk score (risky token)
        let position_risky = manager.calculate_position_size(100.0, &token, 30.0).unwrap();

        // Safe token should get larger position
        assert!(position_safe.amount_sol > position_risky.amount_sol);
        assert!(position_safe.confidence > position_risky.confidence);
    }

    #[test]
    fn test_position_limits() {
        let config = PositionManagerConfig {
            strategy: PositionSizingStrategy::FixedAmount,
            fixed_amount_sol: 100.0, // Very large
            max_position_size_sol: 10.0,
            min_position_size_sol: 1.0,
            ..Default::default()
        };

        let manager = PositionManager::new(config);
        let token = create_test_token();

        let position = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // Should be capped at max
        assert!(position.amount_sol <= 10.0);
    }

    #[test]
    fn test_trade_history_recording() {
        let mut manager = PositionManager::default();

        // Record some trades
        manager.record_trade(1.0); // Win
        manager.record_trade(-0.5); // Loss
        manager.record_trade(1.5); // Win
        manager.record_trade(-0.3); // Loss
        manager.record_trade(2.0); // Win

        let stats = manager.get_statistics();

        assert_eq!(stats.wins, 3);
        assert_eq!(stats.losses, 2);
        assert_eq!(stats.win_rate(), 0.6);
        assert!(stats.avg_win > 0.0);
        assert!(stats.avg_loss < 0.0);
    }

    #[test]
    fn test_martingale_strategy() {
        let config = PositionManagerConfig {
            strategy: PositionSizingStrategy::Martingale,
            fixed_percentage: 0.1,
            martingale_multiplier: 2.0,
            ..Default::default()
        };

        let mut manager = PositionManager::new(config);
        let token = create_test_token();

        // Initial position
        let pos1 = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // After one loss
        manager.record_trade(-1.0);
        let pos2 = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // Position should roughly double after loss
        assert!(pos2.amount_sol > pos1.amount_sol * 1.5);
    }

    #[test]
    fn test_anti_martingale_strategy() {
        let config = PositionManagerConfig {
            strategy: PositionSizingStrategy::AntiMartingale,
            fixed_percentage: 0.1,
            ..Default::default()
        };

        let mut manager = PositionManager::new(config);
        let token = create_test_token();

        // Initial position
        let pos1 = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // After one loss
        manager.record_trade(-1.0);
        let pos2 = manager.calculate_position_size(100.0, &token, 70.0).unwrap();

        // Position should reduce after loss
        assert!(pos2.amount_sol < pos1.amount_sol);
    }

    #[test]
    fn test_confidence_calculation() {
        let manager = PositionManager::default();
        let token = create_test_token();

        let high_risk = manager.calculate_position_size(100.0, &token, 90.0).unwrap();
        let low_risk = manager.calculate_position_size(100.0, &token, 30.0).unwrap();

        assert!(high_risk.confidence > low_risk.confidence);
    }
}
