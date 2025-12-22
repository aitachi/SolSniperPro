use solsniper_core::{TokenInfo, RiskScore};
use async_trait::async_trait;

/// 策略特征
#[async_trait]
pub trait Strategy: Send + Sync {
    fn name(&self) -> &str;
    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool;
    async fn calculate_position_size(&self, token: &TokenInfo, risk_score: &RiskScore) -> f64;
    async fn estimate_expected_profit(&self, token: &TokenInfo, risk_score: &RiskScore) -> f64;
    async fn calculate_risk_reward(&self, token: &TokenInfo, risk_score: &RiskScore) -> f64;
}

/// 策略1: 早鸟极速狙击
pub struct EarlyBirdStrategy;

impl EarlyBirdStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Strategy for EarlyBirdStrategy {
    fn name(&self) -> &str {
        "早鸟极速狙击"
    }

    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool {
        token.age_minutes <= 10
            && token.liquidity_sol >= 15.0
            && risk_score.total >= 85.0
            && token.mint_authority_revoked
            && token.freeze_authority_revoked
            && token.lp_burned
    }

    async fn calculate_position_size(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        1.0 // 固定 1 SOL
    }

    async fn estimate_expected_profit(&self, _token: &TokenInfo, risk_score: &RiskScore) -> f64 {
        30.0 * risk_score.confidence // 预期30%收益
    }

    async fn calculate_risk_reward(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        2.9 // 盈亏比 2.9:1
    }
}

/// 策略2: 流动性追踪策略
pub struct LiquidityHunterStrategy;

impl LiquidityHunterStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Strategy for LiquidityHunterStrategy {
    fn name(&self) -> &str {
        "流动性追踪"
    }

    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool {
        token.liquidity_sol >= 20.0
            && token.lp_locked
            && token.age_minutes >= 15 && token.age_minutes <= 360
            && risk_score.total >= 75.0
            && token.volume_1h >= token.liquidity_usd * 0.3
            && token.top10_ratio <= 0.6
    }

    async fn calculate_position_size(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        2.0 // 2 SOL
    }

    async fn estimate_expected_profit(&self, _token: &TokenInfo, risk_score: &RiskScore) -> f64 {
        68.0 * risk_score.confidence
    }

    async fn calculate_risk_reward(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        3.8
    }
}

/// 策略3: 交易量爆发策略
pub struct VolumeExplosionStrategy;

impl VolumeExplosionStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Strategy for VolumeExplosionStrategy {
    fn name(&self) -> &str {
        "交易量爆发"
    }

    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool {
        let avg_volume = token.volume_6h / 6.0;
        token.volume_1h >= avg_volume * 3.0
            && token.txns_1h_total >= 100
            && (token.txns_1h_buys as f64 / token.txns_1h_sells.max(1) as f64) >= 1.5
            && token.price_change_1h > 0.0
            && token.age_minutes >= 30
            && risk_score.total >= 70.0
    }

    async fn calculate_position_size(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        0.5 // 0.5 SOL (小仓位快进快出)
    }

    async fn estimate_expected_profit(&self, _token: &TokenInfo, risk_score: &RiskScore) -> f64 {
        28.0 * risk_score.confidence
    }

    async fn calculate_risk_reward(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        3.1
    }
}

/// 策略4: 稳健价值投资
pub struct ValueInvestingStrategy;

impl ValueInvestingStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Strategy for ValueInvestingStrategy {
    fn name(&self) -> &str {
        "稳健价值投资"
    }

    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool {
        risk_score.total >= 85.0
            && token.age_hours >= 1.0 && token.age_hours <= 48.0
            && token.liquidity_sol >= 30.0
            && token.lp_locked
            && token.holders_count >= 200
            && token.top10_ratio <= 0.5
            && token.top20_ratio <= 0.7
            && token.mint_authority_revoked
            && token.freeze_authority_revoked
            && token.lp_burned
            && token.buy_tax <= 5.0
            && token.sell_tax <= 5.0
    }

    async fn calculate_position_size(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        5.0 // 5 SOL (大仓位长持)
    }

    async fn estimate_expected_profit(&self, _token: &TokenInfo, risk_score: &RiskScore) -> f64 {
        125.0 * risk_score.confidence
    }

    async fn calculate_risk_reward(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        5.7
    }
}

/// 策略5: 反向套利策略
pub struct ContrarianArbitrageStrategy;

impl ContrarianArbitrageStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Strategy for ContrarianArbitrageStrategy {
    fn name(&self) -> &str {
        "反向套利"
    }

    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool {
        token.price_change_1h <= -30.0
            && token.liquidity_sol >= 10.0
            && token.txns_1h_total >= 30
            && token.age_hours >= 2.0
            && risk_score.total >= 65.0
            && token.lp_burned
    }

    async fn calculate_position_size(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        0.3 // 0.3 SOL (极小仓位,高风险)
    }

    async fn estimate_expected_profit(&self, _token: &TokenInfo, risk_score: &RiskScore) -> f64 {
        22.0 * risk_score.confidence
    }

    async fn calculate_risk_reward(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        1.8
    }
}

/// 策略6: 时间套利策略
pub struct TimeBasedArbitrageStrategy;

impl TimeBasedArbitrageStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Strategy for TimeBasedArbitrageStrategy {
    fn name(&self) -> &str {
        "时间套利"
    }

    async fn matches(&self, token: &TokenInfo, risk_score: &RiskScore) -> bool {
        // 场景A: 6-12小时窗口
        let scenario_a = token.age_hours >= 6.0 && token.age_hours <= 12.0
            && token.price_change_6h <= -20.0
            && token.price_change_24h > 0.0
            && token.volume_6h >= token.liquidity_usd
            && risk_score.total >= 70.0;

        // 场景B: 24-72小时窗口
        let scenario_b = token.age_hours >= 24.0 && token.age_hours <= 72.0
            && token.price_change_24h > 0.0
            && token.holders_count >= 300
            && token.liquidity_sol >= 50.0
            && risk_score.total >= 80.0;

        scenario_a || scenario_b
    }

    async fn calculate_position_size(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        1.5 // 1.5 SOL
    }

    async fn estimate_expected_profit(&self, _token: &TokenInfo, risk_score: &RiskScore) -> f64 {
        52.0 * risk_score.confidence
    }

    async fn calculate_risk_reward(&self, _token: &TokenInfo, _risk_score: &RiskScore) -> f64 {
        2.9
    }
}
