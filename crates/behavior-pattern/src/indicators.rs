use solsniper_core::{BehaviorPattern, RiskLevel};
use serde::{Deserialize, Serialize};
use chrono::Duration;

/// 行为指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Indicator {
    // === Rug Pull 模式 ===
    /// 流动性突然下降
    SuddenLiquidityDrop {
        threshold_pct: f64,
    },

    /// 创建者大量卖出
    CreatorSellOff {
        threshold_pct: f64,
    },

    /// LP 解锁后撤池
    LpUnlock {
        time_after_launch_hours: u64,
    },

    // === Pump 模式 ===
    /// 协同买入
    CoordinatedBuying {
        wallet_count: usize,
        timeframe_seconds: u64,
    },

    /// 交易量激增
    VolumeSpike {
        multiplier: f64,
    },

    /// 价格抛物线上涨
    PriceParabolicRise {
        slope: f64,
    },

    // === 健康模式 ===
    /// 有机增长
    OrganicGrowth {
        holder_increase_rate: f64,
    },

    /// 稳定交易量
    SteadyVolume {
        variance: f64,
    },

    /// 分布式持有
    DistributedHolding {
        max_top10_ratio: f64,
    },

    // === 操纵模式 ===
    /// 洗售交易
    WashTrading {
        same_wallet_ratio: f64,
    },

    /// 假交易量
    FakeVolume {
        suspicious_tx_ratio: f64,
    },
}

impl Indicator {
    /// 获取指标权重
    pub fn weight(&self) -> f64 {
        match self {
            Self::SuddenLiquidityDrop { .. } => 1.0,
            Self::CreatorSellOff { .. } => 0.9,
            Self::LpUnlock { .. } => 0.8,
            Self::CoordinatedBuying { .. } => 0.7,
            Self::VolumeSpike { .. } => 0.6,
            Self::PriceParabolicRise { .. } => 0.6,
            Self::OrganicGrowth { .. } => 0.8,
            Self::SteadyVolume { .. } => 0.5,
            Self::DistributedHolding { .. } => 0.7,
            Self::WashTrading { .. } => 0.9,
            Self::FakeVolume { .. } => 0.8,
        }
    }
}
