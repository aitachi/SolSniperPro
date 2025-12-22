use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use uuid::Uuid;

/// Token 基础信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,

    // 流动性信息
    pub liquidity_sol: f64,
    pub liquidity_usd: f64,
    pub lp_locked: bool,
    pub lp_burned: bool,

    // 供应量信息
    pub total_supply: u64,
    pub circulating_supply: u64,

    // 持有者信息
    pub holders_count: u64,
    pub top10_ratio: f64,
    pub top20_ratio: f64,
    pub top50_ratio: f64,

    // 权限信息
    pub mint_authority_revoked: bool,
    pub freeze_authority_revoked: bool,

    // 交易税费
    pub buy_tax: f64,
    pub sell_tax: f64,

    // 时间信息
    pub created_at: DateTime<Utc>,
    pub age_minutes: u64,
    pub age_hours: f64,

    // 交易数据
    pub txns_1h_total: u64,
    pub txns_1h_buys: u64,
    pub txns_1h_sells: u64,
    pub volume_1h: f64,
    pub volume_6h: f64,
    pub volume_24h: f64,

    // 价格数据
    pub price_usd: f64,
    pub price_change_1h: f64,
    pub price_change_6h: f64,
    pub price_change_24h: f64,
    pub volatility_1h: f64,

    // 社交数据
    pub twitter_mentions: u64,
    pub telegram_members: u64,
    pub discord_members: Option<u64>,
    pub sentiment_score: f64,

    // 验证状态
    pub is_verified: bool,

    // 池子信息
    pub pool_address: Option<Pubkey>,
    pub dex: String, // "Raydium", "Orca", "Meteora", "PumpFun"
}

/// 风险评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub total: f64,
    pub breakdown: ScoreBreakdown,
    pub confidence: f64,
    pub recommendation: Recommendation,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub contract: Score,
    pub liquidity: Score,
    pub holder: Score,
    pub sentiment: Score,
    pub similarity: Score,
    pub behavior: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub value: f64,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recommendation {
    StrongBuy,
    Buy,
    Hold,
    Avoid,
    StrongAvoid,
}

/// 策略匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyMatch {
    pub strategy_name: String,
    pub position_size: f64,
    pub expected_profit: f64,
    pub risk_reward_ratio: f64,
    pub confidence: f64,
}

/// 交易信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnipeSignal {
    pub id: Uuid,
    pub token: TokenInfo,
    pub risk_score: RiskScore,
    pub strategy_matches: Vec<StrategyMatch>,
    pub recommended_amount: f64,
    pub timestamp: DateTime<Utc>,
}

/// 事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    PoolCreated {
        pool: Pubkey,
        token: Pubkey,
        timestamp: DateTime<Utc>,
    },
    TokenAnalyzed {
        token: TokenInfo,
        risk_score: RiskScore,
        timestamp: DateTime<Utc>,
    },
    SnipeSignal {
        signal: SnipeSignal,
        timestamp: DateTime<Utc>,
    },
    TradeExecuted {
        signature: String,
        token: Pubkey,
        amount: f64,
        success: bool,
        timestamp: DateTime<Utc>,
    },
    RiskAlert {
        token: Pubkey,
        alert_type: AlertType,
        message: String,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    LiquidityDrop,
    RugPullDetected,
    PriceManipulation,
    SuspiciousActivity,
}

/// 聪明钱钱包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartWallet {
    pub address: Pubkey,
    pub total_trades: u64,
    pub profitable_trades: u64,
    pub total_profit_sol: f64,
    pub win_rate: f64,
    pub average_holding_time_hours: f64,
    pub last_active: DateTime<Utc>,
    pub rank: u32,
}

/// 行为模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub name: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub confidence: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// ML 预测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPrediction {
    pub is_rug: bool,
    pub rug_probability: f64,
    pub expected_gain_pct: f64,
    pub confidence: f64,
    pub model_version: String,
}

/// 持有者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holder {
    pub address: Pubkey,
    pub amount: u64,
    pub percentage: f64,
    pub is_team: bool,
    pub is_whale: bool,
}
