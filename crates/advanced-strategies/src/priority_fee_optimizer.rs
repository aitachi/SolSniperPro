use solsniper_core::Result;
use dashmap::DashMap;
use std::sync::Arc;

/// Priority Fee优化器
///
/// 核心原理:
/// 动态调整交易的priorityFee或computeUnitPrice参数，
/// 确保交易能够快速被验证者处理。
pub struct PriorityFeeOptimizer {
    /// 历史Fee数据缓存
    historical_fees: Arc<DashMap<u64, Vec<u64>>>,
}

impl PriorityFeeOptimizer {
    pub fn new() -> Self {
        Self {
            historical_fees: Arc::new(DashMap::new()),
        }
    }

    /// 计算推荐的Priority Fee
    ///
    /// 基于:
    /// 1. 最近20个slot的Fee数据
    /// 2. P75分位数(75%交易能确认)
    /// 3. 网络拥堵情况
    pub async fn calculate_recommended_fee(&self) -> u64 {
        // 基准Fee: 5000 microlamports
        let base_fee = 5000_u64;

        // TODO: 实际实现
        // 1. 查询最近的Fee数据
        // 2. 计算分位数
        // 3. 根据拥堵调整

        let congestion_multiplier = 1.5;

        (base_fee as f64 * congestion_multiplier) as u64
    }

    /// 根据交易重要性调整Fee
    pub fn adjust_for_urgency(&self, base_fee: u64, urgency: UrgencyLevel) -> u64 {
        match urgency {
            UrgencyLevel::Low => base_fee,
            UrgencyLevel::Medium => (base_fee as f64 * 1.5) as u64,
            UrgencyLevel::High => (base_fee as f64 * 2.5) as u64,
            UrgencyLevel::Critical => (base_fee as f64 * 5.0) as u64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_priority_fee_optimizer() {
        let optimizer = PriorityFeeOptimizer::new();

        let base_fee = optimizer.calculate_recommended_fee().await;
        assert!(base_fee > 0);

        let critical_fee = optimizer.adjust_for_urgency(base_fee, UrgencyLevel::Critical);
        assert!(critical_fee > base_fee * 4);

        println!("基准Fee: {} microlamports", base_fee);
        println!("紧急Fee: {} microlamports", critical_fee);
    }
}
