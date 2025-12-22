use solsniper_core::{Error, Result, TokenInfo};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// 退出策略类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitStrategyType {
    /// 固定止盈
    FixedTakeProfit,
    /// 固定止损
    FixedStopLoss,
    /// 追踪止损
    TrailingStop,
    /// 时间退出
    TimeBased,
    /// 分批退出
    Scaled,
    /// 基于指标（RSI/MACD）
    IndicatorBased,
    /// 综合退出（多种条件）
    Composite,
}

/// 退出信号
#[derive(Debug, Clone)]
pub struct ExitSignal {
    /// 是否应该退出
    pub should_exit: bool,

    /// 建议退出比例（0-1）
    pub exit_percentage: f64,

    /// 触发的策略
    pub triggered_by: Vec<ExitStrategyType>,

    /// 原因说明
    pub reasons: Vec<String>,

    /// 紧急程度（0-1，越高越紧急）
    pub urgency: f64,

    /// 预期价格（如果有）
    pub target_price: Option<f64>,
}

impl ExitSignal {
    pub fn no_exit() -> Self {
        Self {
            should_exit: false,
            exit_percentage: 0.0,
            triggered_by: Vec::new(),
            reasons: Vec::new(),
            urgency: 0.0,
            target_price: None,
        }
    }

    pub fn full_exit(reason: String, urgency: f64) -> Self {
        Self {
            should_exit: true,
            exit_percentage: 1.0,
            triggered_by: Vec::new(),
            reasons: vec![reason],
            urgency,
            target_price: None,
        }
    }
}

/// 持仓跟踪信息
#[derive(Debug, Clone)]
pub struct PositionTracker {
    /// 入场价格
    pub entry_price: f64,

    /// 入场时间
    pub entry_time: Instant,

    /// 最高价格
    pub highest_price: f64,

    /// 最低价格（入场后）
    pub lowest_price: f64,

    /// 当前价格
    pub current_price: f64,

    /// 持仓数量
    pub position_size: f64,

    /// 已实现盈亏
    pub realized_pnl: f64,
}

impl PositionTracker {
    pub fn new(entry_price: f64, position_size: f64) -> Self {
        Self {
            entry_price,
            entry_time: Instant::now(),
            highest_price: entry_price,
            lowest_price: entry_price,
            current_price: entry_price,
            position_size,
            realized_pnl: 0.0,
        }
    }

    /// 更新当前价格
    pub fn update_price(&mut self, current_price: f64) {
        self.current_price = current_price;
        self.highest_price = self.highest_price.max(current_price);
        self.lowest_price = self.lowest_price.min(current_price);
    }

    /// 计算未实现盈亏（百分比）
    pub fn unrealized_pnl_pct(&self) -> f64 {
        (self.current_price - self.entry_price) / self.entry_price * 100.0
    }

    /// 计算未实现盈亏（绝对值）
    pub fn unrealized_pnl_abs(&self) -> f64 {
        (self.current_price - self.entry_price) * self.position_size
    }

    /// 从最高点回撤百分比
    pub fn drawdown_from_high(&self) -> f64 {
        if self.highest_price == 0.0 {
            return 0.0;
        }
        (self.highest_price - self.current_price) / self.highest_price * 100.0
    }

    /// 持仓时长（秒）
    pub fn holding_duration(&self) -> u64 {
        self.entry_time.elapsed().as_secs()
    }
}

/// 退出策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitStrategyConfig {
    // 止盈配置
    pub take_profit_pct: f64,        // 止盈百分比
    pub partial_take_profit_pct: f64, // 部分止盈百分比
    pub partial_exit_ratio: f64,     // 部分退出比例

    // 止损配置
    pub stop_loss_pct: f64,          // 止损百分比

    // 追踪止损配置
    pub trailing_stop_pct: f64,      // 追踪止损百分比
    pub trailing_activation_pct: f64, // 追踪激活百分比

    // 时间退出配置
    pub max_holding_minutes: u64,    // 最大持仓时间（分钟）
    pub min_holding_minutes: u64,    // 最小持仓时间（分钟）

    // 分批退出配置
    pub scale_out_levels: Vec<f64>,  // 分批退出价格水平
    pub scale_out_ratios: Vec<f64>,  // 各级退出比例

    // 动态调整
    pub dynamic_adjustment: bool,    // 是否启用动态调整
    pub volatility_multiplier: f64,  // 波动率乘数

    // 保护性止损
    pub breakeven_protection: bool,  // 盈利后移动止损到成本价
    pub breakeven_trigger_pct: f64,  // 触发保本止损的盈利百分比
}

impl Default for ExitStrategyConfig {
    fn default() -> Self {
        Self {
            take_profit_pct: 50.0,           // 50% 止盈
            partial_take_profit_pct: 25.0,   // 25% 部分止盈
            partial_exit_ratio: 0.5,         // 卖出50%
            stop_loss_pct: 20.0,             // 20% 止损
            trailing_stop_pct: 10.0,         // 10% 追踪止损
            trailing_activation_pct: 20.0,   // 20% 盈利后激活追踪
            max_holding_minutes: 240,        // 最多持有4小时
            min_holding_minutes: 5,          // 至少持有5分钟
            scale_out_levels: vec![20.0, 40.0, 60.0], // 20%, 40%, 60% 盈利时分批卖出
            scale_out_ratios: vec![0.3, 0.3, 0.4],    // 各卖出30%, 30%, 40%
            dynamic_adjustment: true,
            volatility_multiplier: 1.5,
            breakeven_protection: true,
            breakeven_trigger_pct: 10.0,     // 10% 盈利后启用保本
        }
    }
}

/// 退出策略管理器
pub struct ExitStrategyManager {
    config: ExitStrategyConfig,
    position: Option<PositionTracker>,
    trailing_stop_activated: bool,
    breakeven_activated: bool,
    scale_out_executed: Vec<bool>, // 跟踪哪些分批退出已执行
}

impl ExitStrategyManager {
    /// 创建新的退出策略管理器
    pub fn new(config: ExitStrategyConfig) -> Self {
        let scale_out_count = config.scale_out_levels.len();
        Self {
            config,
            position: None,
            trailing_stop_activated: false,
            breakeven_activated: false,
            scale_out_executed: vec![false; scale_out_count],
        }
    }

    /// 创建默认配置的管理器
    pub fn default() -> Self {
        Self::new(ExitStrategyConfig::default())
    }

    /// 开仓
    pub fn open_position(&mut self, entry_price: f64, position_size: f64) {
        self.position = Some(PositionTracker::new(entry_price, position_size));
        self.trailing_stop_activated = false;
        self.breakeven_activated = false;
        self.scale_out_executed = vec![false; self.config.scale_out_levels.len()];

        tracing::info!(
            "📈 Position opened: entry_price={:.8}, size={:.4}",
            entry_price,
            position_size
        );
    }

    /// 检查是否应该退出
    pub fn check_exit(&mut self, current_price: f64, token: &TokenInfo) -> Result<ExitSignal> {
        let position = self.position.as_mut().ok_or_else(|| {
            Error::Internal("No active position to check exit".to_string())
        })?;

        // 更新当前价格
        position.update_price(current_price);

        let mut signal = ExitSignal::no_exit();
        let pnl_pct = position.unrealized_pnl_pct();

        // 1. 检查固定止损
        if let Some(sl_signal) = self.check_stop_loss(position) {
            signal = Self::merge_signals(signal, sl_signal);
        }

        // 2. 检查固定止盈
        if let Some(tp_signal) = self.check_take_profit(position) {
            signal = Self::merge_signals(signal, tp_signal);
        }

        // 3. 检查追踪止损
        if let Some(trail_signal) = self.check_trailing_stop(position) {
            signal = Self::merge_signals(signal, trail_signal);
        }

        // 4. 检查时间退出
        if let Some(time_signal) = self.check_time_based_exit(position) {
            signal = Self::merge_signals(signal, time_signal);
        }

        // 5. 检查分批退出
        if let Some(scale_signal) = self.check_scaled_exit(position) {
            signal = Self::merge_signals(signal, scale_signal);
        }

        // 6. 检查保本止损
        if let Some(be_signal) = self.check_breakeven_protection(position) {
            signal = Self::merge_signals(signal, be_signal);
        }

        // 7. 检查基于指标的退出
        if let Some(indicator_signal) = self.check_indicator_exit(position, token) {
            signal = Self::merge_signals(signal, indicator_signal);
        }

        if signal.should_exit {
            tracing::info!(
                "🚪 Exit signal: exit_pct={:.0}%, urgency={:.2}, reasons={:?}",
                signal.exit_percentage * 100.0,
                signal.urgency,
                signal.reasons
            );
        }

        Ok(signal)
    }

    /// 检查固定止损
    fn check_stop_loss(&self, position: &PositionTracker) -> Option<ExitSignal> {
        let pnl_pct = position.unrealized_pnl_pct();

        if pnl_pct <= -self.config.stop_loss_pct {
            Some(ExitSignal {
                should_exit: true,
                exit_percentage: 1.0,
                triggered_by: vec![ExitStrategyType::FixedStopLoss],
                reasons: vec![format!(
                    "Stop loss triggered: {:.2}% loss (limit: {:.2}%)",
                    pnl_pct.abs(),
                    self.config.stop_loss_pct
                )],
                urgency: 1.0, // 最高紧急度
                target_price: Some(position.entry_price * (1.0 - self.config.stop_loss_pct / 100.0)),
            })
        } else {
            None
        }
    }

    /// 检查固定止盈
    fn check_take_profit(&self, position: &PositionTracker) -> Option<ExitSignal> {
        let pnl_pct = position.unrealized_pnl_pct();

        // 完全止盈
        if pnl_pct >= self.config.take_profit_pct {
            return Some(ExitSignal {
                should_exit: true,
                exit_percentage: 1.0,
                triggered_by: vec![ExitStrategyType::FixedTakeProfit],
                reasons: vec![format!(
                    "Take profit triggered: {:.2}% profit (target: {:.2}%)",
                    pnl_pct,
                    self.config.take_profit_pct
                )],
                urgency: 0.8,
                target_price: Some(position.entry_price * (1.0 + self.config.take_profit_pct / 100.0)),
            });
        }

        // 部分止盈
        if pnl_pct >= self.config.partial_take_profit_pct {
            return Some(ExitSignal {
                should_exit: true,
                exit_percentage: self.config.partial_exit_ratio,
                triggered_by: vec![ExitStrategyType::FixedTakeProfit],
                reasons: vec![format!(
                    "Partial take profit: {:.2}% profit, selling {:.0}%",
                    pnl_pct,
                    self.config.partial_exit_ratio * 100.0
                )],
                urgency: 0.6,
                target_price: Some(position.entry_price * (1.0 + self.config.partial_take_profit_pct / 100.0)),
            });
        }

        None
    }

    /// 检查追踪止损
    fn check_trailing_stop(&mut self, position: &PositionTracker) -> Option<ExitSignal> {
        let pnl_pct = position.unrealized_pnl_pct();

        // 检查是否激活追踪止损
        if !self.trailing_stop_activated && pnl_pct >= self.config.trailing_activation_pct {
            self.trailing_stop_activated = true;
            tracing::info!(
                "🎯 Trailing stop activated at {:.2}% profit",
                self.config.trailing_activation_pct
            );
        }

        // 如果已激活，检查是否触发
        if self.trailing_stop_activated {
            let drawdown = position.drawdown_from_high();

            if drawdown >= self.config.trailing_stop_pct {
                return Some(ExitSignal {
                    should_exit: true,
                    exit_percentage: 1.0,
                    triggered_by: vec![ExitStrategyType::TrailingStop],
                    reasons: vec![format!(
                        "Trailing stop triggered: {:.2}% drawdown from high (limit: {:.2}%)",
                        drawdown,
                        self.config.trailing_stop_pct
                    )],
                    urgency: 0.9,
                    target_price: Some(position.highest_price * (1.0 - self.config.trailing_stop_pct / 100.0)),
                });
            }
        }

        None
    }

    /// 检查时间退出
    fn check_time_based_exit(&self, position: &PositionTracker) -> Option<ExitSignal> {
        let holding_minutes = position.holding_duration() / 60;

        // 超过最大持仓时间
        if holding_minutes >= self.config.max_holding_minutes {
            return Some(ExitSignal {
                should_exit: true,
                exit_percentage: 1.0,
                triggered_by: vec![ExitStrategyType::TimeBased],
                reasons: vec![format!(
                    "Max holding time reached: {} minutes (limit: {})",
                    holding_minutes,
                    self.config.max_holding_minutes
                )],
                urgency: 0.7,
                target_price: None,
            });
        }

        None
    }

    /// 检查分批退出
    fn check_scaled_exit(&mut self, position: &PositionTracker) -> Option<ExitSignal> {
        let pnl_pct = position.unrealized_pnl_pct();

        for (i, &level) in self.config.scale_out_levels.iter().enumerate() {
            if !self.scale_out_executed[i] && pnl_pct >= level {
                self.scale_out_executed[i] = true;
                let exit_ratio = self.config.scale_out_ratios.get(i).copied().unwrap_or(0.33);

                return Some(ExitSignal {
                    should_exit: true,
                    exit_percentage: exit_ratio,
                    triggered_by: vec![ExitStrategyType::Scaled],
                    reasons: vec![format!(
                        "Scaled exit level {}: {:.2}% profit, selling {:.0}%",
                        i + 1,
                        pnl_pct,
                        exit_ratio * 100.0
                    )],
                    urgency: 0.5,
                    target_price: Some(position.entry_price * (1.0 + level / 100.0)),
                });
            }
        }

        None
    }

    /// 检查保本止损
    fn check_breakeven_protection(&mut self, position: &PositionTracker) -> Option<ExitSignal> {
        if !self.config.breakeven_protection {
            return None;
        }

        let pnl_pct = position.unrealized_pnl_pct();

        // 激活保本保护
        if !self.breakeven_activated && pnl_pct >= self.config.breakeven_trigger_pct {
            self.breakeven_activated = true;
            tracing::info!(
                "🛡️ Breakeven protection activated at {:.2}% profit",
                self.config.breakeven_trigger_pct
            );
        }

        // 如果已激活，检查是否回到成本价以下
        if self.breakeven_activated && pnl_pct <= 0.0 {
            return Some(ExitSignal {
                should_exit: true,
                exit_percentage: 1.0,
                triggered_by: vec![ExitStrategyType::FixedStopLoss],
                reasons: vec![format!(
                    "Breakeven protection triggered: price returned to entry level"
                )],
                urgency: 0.85,
                target_price: Some(position.entry_price),
            });
        }

        None
    }

    /// 检查基于指标的退出
    fn check_indicator_exit(&self, position: &PositionTracker, token: &TokenInfo) -> Option<ExitSignal> {
        let mut reasons = Vec::new();
        let mut should_exit = false;

        // 检查交易量骤降（可能是流动性枯竭）
        if token.volume_1h < token.volume_6h * 0.1 {
            reasons.push("Volume collapsed (1h < 10% of 6h average)".to_string());
            should_exit = true;
        }

        // 检查大额卖压
        if token.txns_1h_total > 0 {
            let sell_ratio = token.txns_1h_sells as f64 / token.txns_1h_total as f64;
            if sell_ratio > 0.8 {
                reasons.push(format!("High sell pressure: {:.1}% sells", sell_ratio * 100.0));
                should_exit = true;
            }
        }

        // 检查价格急跌
        if token.price_change_1h < -15.0 {
            reasons.push(format!("Sharp price drop: {:.1}% in 1h", token.price_change_1h));
            should_exit = true;
        }

        if should_exit {
            Some(ExitSignal {
                should_exit: true,
                exit_percentage: 1.0,
                triggered_by: vec![ExitStrategyType::IndicatorBased],
                reasons,
                urgency: 0.9,
                target_price: None,
            })
        } else {
            None
        }
    }

    /// 合并退出信号
    fn merge_signals(mut base: ExitSignal, new: ExitSignal) -> ExitSignal {
        if !new.should_exit {
            return base;
        }

        if !base.should_exit {
            return new;
        }

        // 两个都要退出，合并
        base.exit_percentage = base.exit_percentage.max(new.exit_percentage);
        base.urgency = base.urgency.max(new.urgency);
        base.triggered_by.extend(new.triggered_by);
        base.reasons.extend(new.reasons);

        base
    }

    /// 执行退出
    pub fn execute_exit(&mut self, exit_percentage: f64) -> Result<f64> {
        let position = self.position.as_mut().ok_or_else(|| {
            Error::Internal("No active position to exit".to_string())
        })?;

        let exit_amount = position.position_size * exit_percentage;
        let realized_pnl = (position.current_price - position.entry_price) * exit_amount;

        position.position_size -= exit_amount;
        position.realized_pnl += realized_pnl;

        tracing::info!(
            "💰 Position exited: {:.0}%, amount={:.4}, PnL={:.4} ({:.2}%)",
            exit_percentage * 100.0,
            exit_amount,
            realized_pnl,
            (realized_pnl / (position.entry_price * exit_amount)) * 100.0
        );

        // 如果完全退出，清空仓位
        if position.position_size < 0.0001 {
            tracing::info!(
                "📊 Position fully closed: Total realized PnL={:.4}",
                position.realized_pnl
            );
            self.position = None;
        }

        Ok(realized_pnl)
    }

    /// 获取当前仓位
    pub fn get_position(&self) -> Option<&PositionTracker> {
        self.position.as_ref()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: ExitStrategyConfig) {
        self.config = config;
        tracing::info!("⚙️ Exit strategy config updated");
    }

    /// 是否有持仓
    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_token() -> TokenInfo {
        use chrono::Utc;
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
    fn test_stop_loss_trigger() {
        let mut manager = ExitStrategyManager::default();
        let token = create_test_token();

        manager.open_position(1.0, 100.0);

        // Price drops 25% (exceeds 20% stop loss)
        let signal = manager.check_exit(0.75, &token).unwrap();

        assert!(signal.should_exit);
        assert_eq!(signal.exit_percentage, 1.0);
        assert!(signal.triggered_by.contains(&ExitStrategyType::FixedStopLoss));
    }

    #[test]
    fn test_take_profit_trigger() {
        let mut manager = ExitStrategyManager::default();
        let token = create_test_token();

        manager.open_position(1.0, 100.0);

        // Price rises 60% (exceeds 50% take profit)
        let signal = manager.check_exit(1.6, &token).unwrap();

        assert!(signal.should_exit);
        assert_eq!(signal.exit_percentage, 1.0);
        assert!(signal.triggered_by.contains(&ExitStrategyType::FixedTakeProfit));
    }

    #[test]
    fn test_partial_take_profit() {
        let config = ExitStrategyConfig {
            partial_take_profit_pct: 30.0,
            partial_exit_ratio: 0.5,
            take_profit_pct: 100.0, // Set high to not trigger full TP
            ..Default::default()
        };

        let mut manager = ExitStrategyManager::new(config);
        let token = create_test_token();

        manager.open_position(1.0, 100.0);

        // Price rises 35% (exceeds 30% partial TP)
        let signal = manager.check_exit(1.35, &token).unwrap();

        assert!(signal.should_exit);
        assert_eq!(signal.exit_percentage, 0.5); // Sell 50%
    }

    #[test]
    fn test_trailing_stop() {
        let config = ExitStrategyConfig {
            trailing_activation_pct: 20.0,
            trailing_stop_pct: 10.0,
            ..Default::default()
        };

        let mut manager = ExitStrategyManager::new(config);
        let token = create_test_token();

        manager.open_position(1.0, 100.0);

        // Price rises 25% (activates trailing stop)
        manager.check_exit(1.25, &token).unwrap();
        assert!(manager.trailing_stop_activated);

        // Price drops to 1.10 (15% from high, exceeds 10% trailing)
        let signal = manager.check_exit(1.10, &token).unwrap();

        assert!(signal.should_exit);
        assert!(signal.triggered_by.contains(&ExitStrategyType::TrailingStop));
    }

    #[test]
    fn test_position_tracker() {
        let mut tracker = PositionTracker::new(1.0, 100.0);

        tracker.update_price(1.2);
        assert_eq!(tracker.unrealized_pnl_pct(), 20.0);

        tracker.update_price(0.9);
        assert_eq!(tracker.unrealized_pnl_pct(), -10.0);
        assert_eq!(tracker.drawdown_from_high(), 25.0); // (1.2 - 0.9) / 1.2
    }

    #[test]
    fn test_execute_exit() {
        let mut manager = ExitStrategyManager::default();

        manager.open_position(1.0, 100.0);
        manager.position.as_mut().unwrap().update_price(1.5);

        // Exit 50%
        let pnl = manager.execute_exit(0.5).unwrap();

        assert_eq!(pnl, 25.0); // (1.5 - 1.0) * 50 = 25
        assert!(manager.has_position());
        assert_eq!(manager.position.as_ref().unwrap().position_size, 50.0);

        // Exit remaining 50%
        manager.execute_exit(1.0).unwrap();
        assert!(!manager.has_position());
    }

    #[test]
    fn test_breakeven_protection() {
        let config = ExitStrategyConfig {
            breakeven_protection: true,
            breakeven_trigger_pct: 10.0,
            ..Default::default()
        };

        let mut manager = ExitStrategyManager::new(config);
        let token = create_test_token();

        manager.open_position(1.0, 100.0);

        // Price rises 15% (activates breakeven)
        manager.check_exit(1.15, &token).unwrap();
        assert!(manager.breakeven_activated);

        // Price returns to entry
        let signal = manager.check_exit(0.99, &token).unwrap();

        assert!(signal.should_exit);
    }
}
