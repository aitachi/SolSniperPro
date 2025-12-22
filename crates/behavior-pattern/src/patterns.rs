use solsniper_core::{BehaviorPattern, RiskLevel};
use crate::indicators::Indicator;

/// 预定义模式库
pub struct PatternLibrary;

impl PatternLibrary {
    /// 加载所有预定义模式
    pub fn load_all() -> Vec<Pattern> {
        vec![
            Self::fast_rug_pull(),
            Self::slow_rug_pull(),
            Self::coordinated_pump(),
            Self::organic_growth(),
            Self::wash_trading(),
        ]
    }

    /// 模式 1: 快速撤池 Rug
    fn fast_rug_pull() -> Pattern {
        Pattern {
            name: "快速撤池 Rug".to_string(),
            description: "项目方在短时间内撤除流动性".to_string(),
            indicators: vec![
                Indicator::SuddenLiquidityDrop {
                    threshold_pct: 80.0,
                },
                Indicator::CreatorSellOff {
                    threshold_pct: 50.0,
                },
            ],
            confidence_threshold: 0.8,
            risk_level: RiskLevel::Critical,
        }
    }

    /// 模式 2: 慢速撤池 Rug
    fn slow_rug_pull() -> Pattern {
        Pattern {
            name: "慢速撤池 Rug".to_string(),
            description: "项目方逐步撤除流动性，避免触发告警".to_string(),
            indicators: vec![
                Indicator::SuddenLiquidityDrop {
                    threshold_pct: 30.0,
                },
                Indicator::LpUnlock {
                    time_after_launch_hours: 24 * 7,
                },
                Indicator::CreatorSellOff {
                    threshold_pct: 30.0,
                },
            ],
            confidence_threshold: 0.7,
            risk_level: RiskLevel::High,
        }
    }

    /// 模式 3: 协同拉盘
    fn coordinated_pump() -> Pattern {
        Pattern {
            name: "协同拉盘".to_string(),
            description: "多个钱包短时间内大量买入".to_string(),
            indicators: vec![
                Indicator::CoordinatedBuying {
                    wallet_count: 20,
                    timeframe_seconds: 300,
                },
                Indicator::VolumeSpike {
                    multiplier: 5.0,
                },
                Indicator::PriceParabolicRise {
                    slope: 2.0,
                },
            ],
            confidence_threshold: 0.75,
            risk_level: RiskLevel::Medium,
        }
    }

    /// 模式 4: 有机增长
    fn organic_growth() -> Pattern {
        Pattern {
            name: "有机增长".to_string(),
            description: "持有者稳步增加，交易量稳定".to_string(),
            indicators: vec![
                Indicator::OrganicGrowth {
                    holder_increase_rate: 10.0,
                },
                Indicator::SteadyVolume {
                    variance: 0.3,
                },
                Indicator::DistributedHolding {
                    max_top10_ratio: 0.4,
                },
            ],
            confidence_threshold: 0.8,
            risk_level: RiskLevel::Low,
        }
    }

    /// 模式 5: 洗售交易
    fn wash_trading() -> Pattern {
        Pattern {
            name: "洗售交易".to_string(),
            description: "同一钱包反复买卖制造虚假交易量".to_string(),
            indicators: vec![
                Indicator::WashTrading {
                    same_wallet_ratio: 0.6,
                },
                Indicator::FakeVolume {
                    suspicious_tx_ratio: 0.7,
                },
            ],
            confidence_threshold: 0.75,
            risk_level: RiskLevel::High,
        }
    }
}

/// 模式定义
#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub indicators: Vec<Indicator>,
    pub confidence_threshold: f64,
    pub risk_level: RiskLevel,
}
