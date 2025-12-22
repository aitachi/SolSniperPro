use solsniper_core::Result;
use solana_sdk::transaction::Transaction;
use std::sync::Arc;

/// MEV保护优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MevPriority {
    /// 低优先级（标准交易）
    Low,
    /// 中优先级（竞争性交易）
    Medium,
    /// 高优先级（时间敏感）
    High,
    /// 关键优先级（必须优先执行）
    Critical,
}

/// 受保护的交易
#[derive(Debug, Clone)]
pub enum ProtectedTransaction {
    /// JITO Bundle保护
    JitoBundle {
        transaction: Transaction,
        tip_lamports: u64,
    },
    /// 优先费用保护
    PriorityFee {
        transaction: Transaction,
        fee_micro_lamports: u64,
    },
    /// 标准交易（无保护）
    Standard { transaction: Transaction },
}

/// MEV保护器
///
/// 提供MEV（Maximal Extractable Value）保护，防止三明治攻击和抢跑
///
/// # 保护策略
/// 1. JITO Bundle: 通过支付tip获得优先执行权
/// 2. Priority Fee: 通过设置高优先费用提升交易优先级
/// 3. 动态调整: 根据网络状况和优先级自动调整费用
pub struct MevProtector {
    /// 是否启用JITO Bundle
    jito_enabled: bool,

    /// 最小tip金额（lamports）
    min_tip_lamports: u64,

    /// 最大tip金额（lamports）
    max_tip_lamports: u64,

    /// 基础priority fee（micro-lamports per compute unit）
    base_priority_fee: u64,

    /// 是否启用动态费用调整
    dynamic_adjustment: bool,
}

impl MevProtector {
    /// 创建新的MEV保护器
    ///
    /// # 参数
    /// - `jito_enabled`: 是否启用JITO Bundle
    /// - `min_tip_lamports`: 最小tip金额
    /// - `dynamic_adjustment`: 是否启用动态费用调整
    pub fn new(jito_enabled: bool, min_tip_lamports: u64, dynamic_adjustment: bool) -> Self {
        Self {
            jito_enabled,
            min_tip_lamports,
            max_tip_lamports: min_tip_lamports * 10, // 最大tip为最小tip的10倍
            base_priority_fee: 50_000, // 默认50,000 micro-lamports
            dynamic_adjustment,
        }
    }

    /// 创建默认保护器（启用JITO，最小tip 0.001 SOL）
    pub fn default() -> Self {
        Self::new(true, 1_000_000, true) // 0.001 SOL
    }

    /// 创建仅使用priority fee的保护器
    pub fn priority_fee_only() -> Self {
        Self::new(false, 0, true)
    }

    /// 保护交易
    ///
    /// 根据优先级和配置选择最佳保护策略
    ///
    /// # 策略选择
    /// - JITO启用 + High/Critical优先级 → JITO Bundle
    /// - JITO禁用或Low/Medium优先级 → Priority Fee
    ///
    /// # 参数
    /// - `transaction`: 待保护的交易
    /// - `priority`: MEV优先级
    pub fn protect_transaction(
        &self,
        transaction: Transaction,
        priority: MevPriority,
    ) -> Result<ProtectedTransaction> {
        // 策略1: 使用JITO Bundle（高优先级或关键交易）
        if self.jito_enabled && matches!(priority, MevPriority::High | MevPriority::Critical) {
            let tip = self.calculate_dynamic_tip(priority);

            tracing::info!(
                "🛡️ Protecting transaction with JITO Bundle (priority={:?}, tip={} lamports / {:.6} SOL)",
                priority,
                tip,
                tip as f64 / 1e9
            );

            return Ok(ProtectedTransaction::JitoBundle {
                transaction,
                tip_lamports: tip,
            });
        }

        // 策略2: 使用Priority Fee
        if matches!(priority, MevPriority::Medium | MevPriority::High) {
            let priority_fee = self.calculate_priority_fee(priority);

            tracing::info!(
                "🛡️ Protecting transaction with Priority Fee (priority={:?}, fee={} micro-lamports)",
                priority,
                priority_fee
            );

            return Ok(ProtectedTransaction::PriorityFee {
                transaction,
                fee_micro_lamports: priority_fee,
            });
        }

        // 策略3: 标准交易（低优先级）
        tracing::debug!("Transaction sent without MEV protection (priority={:?})", priority);

        Ok(ProtectedTransaction::Standard { transaction })
    }

    /// 计算动态tip金额
    ///
    /// 根据优先级动态调整tip
    ///
    /// # 优先级映射
    /// - Low: min_tip * 1.0
    /// - Medium: min_tip * 2.0
    /// - High: min_tip * 4.0
    /// - Critical: min_tip * 8.0
    pub fn calculate_dynamic_tip(&self, priority: MevPriority) -> u64 {
        if !self.dynamic_adjustment {
            return self.min_tip_lamports;
        }

        let multiplier = match priority {
            MevPriority::Low => 1.0,
            MevPriority::Medium => 2.0,
            MevPriority::High => 4.0,
            MevPriority::Critical => 8.0,
        };

        let tip = (self.min_tip_lamports as f64 * multiplier) as u64;

        // 限制最大值
        tip.min(self.max_tip_lamports)
    }

    /// 计算priority fee
    ///
    /// 根据优先级动态调整priority fee
    ///
    /// # 优先级映射
    /// - Low: base * 1.0
    /// - Medium: base * 2.0
    /// - High: base * 5.0
    /// - Critical: base * 10.0
    pub fn calculate_priority_fee(&self, priority: MevPriority) -> u64 {
        if !self.dynamic_adjustment {
            return self.base_priority_fee;
        }

        let multiplier = match priority {
            MevPriority::Low => 1.0,
            MevPriority::Medium => 2.0,
            MevPriority::High => 5.0,
            MevPriority::Critical => 10.0,
        };

        (self.base_priority_fee as f64 * multiplier) as u64
    }

    /// 估算总MEV保护成本
    ///
    /// # 参数
    /// - `priority`: MEV优先级
    /// - `compute_units`: 计算单元数量
    ///
    /// # 返回
    /// 总成本（lamports）
    pub fn estimate_protection_cost(&self, priority: MevPriority, compute_units: u64) -> u64 {
        if self.jito_enabled && matches!(priority, MevPriority::High | MevPriority::Critical) {
            // JITO成本 = tip + 基础priority fee
            let tip = self.calculate_dynamic_tip(priority);
            let base_fee = (self.base_priority_fee * compute_units) / 1_000_000; // micro-lamports to lamports
            tip + base_fee
        } else {
            // Priority Fee成本
            let priority_fee = self.calculate_priority_fee(priority);
            (priority_fee * compute_units) / 1_000_000 // micro-lamports to lamports
        }
    }

    /// 设置JITO启用状态
    pub fn set_jito_enabled(&mut self, enabled: bool) {
        self.jito_enabled = enabled;
    }

    /// 设置最小tip
    pub fn set_min_tip(&mut self, tip_lamports: u64) {
        self.min_tip_lamports = tip_lamports;
        self.max_tip_lamports = tip_lamports * 10;
    }

    /// 设置基础priority fee
    pub fn set_base_priority_fee(&mut self, fee_micro_lamports: u64) {
        self.base_priority_fee = fee_micro_lamports;
    }

    /// 获取推荐的优先级
    ///
    /// 根据交易特征推荐合适的MEV优先级
    ///
    /// # 参数
    /// - `is_time_sensitive`: 是否时间敏感（如新币狙击）
    /// - `amount_sol`: 交易金额
    /// - `pool_liquidity`: 池子流动性
    pub fn recommend_priority(
        &self,
        is_time_sensitive: bool,
        amount_sol: f64,
        pool_liquidity: f64,
    ) -> MevPriority {
        // 新币狙击或大额交易 → Critical
        if is_time_sensitive && amount_sol > 5.0 {
            return MevPriority::Critical;
        }

        // 时间敏感或中大额交易 → High
        if is_time_sensitive || amount_sol > 2.0 {
            return MevPriority::High;
        }

        // 占池子流动性比例较大 → High
        let pool_impact = amount_sol / pool_liquidity.max(0.1);
        if pool_impact > 0.05 {
            // 超过5%流动性
            return MevPriority::High;
        }

        // 普通交易 → Medium
        if amount_sol > 0.5 {
            return MevPriority::Medium;
        }

        // 小额交易 → Low
        MevPriority::Low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_protector() {
        let protector = MevProtector::default();
        assert_eq!(protector.jito_enabled, true);
        assert_eq!(protector.min_tip_lamports, 1_000_000);
        assert_eq!(protector.dynamic_adjustment, true);
    }

    #[test]
    fn test_calculate_dynamic_tip() {
        let protector = MevProtector::default();

        assert_eq!(
            protector.calculate_dynamic_tip(MevPriority::Low),
            1_000_000
        );
        assert_eq!(
            protector.calculate_dynamic_tip(MevPriority::Medium),
            2_000_000
        );
        assert_eq!(
            protector.calculate_dynamic_tip(MevPriority::High),
            4_000_000
        );
        assert_eq!(
            protector.calculate_dynamic_tip(MevPriority::Critical),
            8_000_000
        );
    }

    #[test]
    fn test_calculate_priority_fee() {
        let protector = MevProtector::default();

        assert_eq!(
            protector.calculate_priority_fee(MevPriority::Low),
            50_000
        );
        assert_eq!(
            protector.calculate_priority_fee(MevPriority::Medium),
            100_000
        );
        assert_eq!(
            protector.calculate_priority_fee(MevPriority::High),
            250_000
        );
        assert_eq!(
            protector.calculate_priority_fee(MevPriority::Critical),
            500_000
        );
    }

    #[test]
    fn test_recommend_priority() {
        let protector = MevProtector::default();

        // 新币狙击 + 大额 → Critical
        assert_eq!(
            protector.recommend_priority(true, 10.0, 100.0),
            MevPriority::Critical
        );

        // 新币狙击 + 中额 → High
        assert_eq!(
            protector.recommend_priority(true, 3.0, 100.0),
            MevPriority::High
        );

        // 普通交易 + 大额 → High
        assert_eq!(
            protector.recommend_priority(false, 5.0, 100.0),
            MevPriority::High
        );

        // 普通交易 + 小额 → Medium
        assert_eq!(
            protector.recommend_priority(false, 1.0, 100.0),
            MevPriority::Medium
        );

        // 极小额 → Low
        assert_eq!(
            protector.recommend_priority(false, 0.1, 100.0),
            MevPriority::Low
        );
    }

    #[test]
    fn test_estimate_protection_cost() {
        let protector = MevProtector::default();
        let compute_units = 200_000;

        // JITO Bundle（High priority）
        let cost_high = protector.estimate_protection_cost(MevPriority::High, compute_units);
        assert!(cost_high > 4_000_000); // > 4M lamports tip

        // Priority Fee（Medium priority）
        let cost_medium =
            protector.estimate_protection_cost(MevPriority::Medium, compute_units);
        assert!(cost_medium < 1_000_000); // < 1M lamports
    }
}
