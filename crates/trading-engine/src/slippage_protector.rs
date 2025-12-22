use solsniper_core::Result;
use std::cmp::min;

/// 滑点保护器
///
/// 保护交易免受过高滑点影响，提供：
/// - 价格影响计算（基于AMM公式）
/// - 动态滑点调整（根据流动性）
/// - 滑点验证
/// - 最小输出金额计算
#[derive(Clone)]
pub struct SlippageProtector {
    /// 最大滑点（basis points, 100 = 1%）
    pub(crate) max_slippage_bps: u16,

    /// 是否启用动态调整
    pub(crate) dynamic_adjustment: bool,

    /// 动态调整因子
    pub(crate) dynamic_factor: f64,
}

/// Swap报价
#[derive(Debug, Clone)]
pub struct SwapQuote {
    /// 输入金额
    pub amount_in: u64,

    /// 预期输出金额
    pub expected_out: u64,

    /// 最小输出金额（考虑滑点）
    pub min_amount_out: u64,

    /// 价格影响（basis points）
    pub price_impact_bps: u16,

    /// 实际滑点（basis points）
    pub slippage_bps: u16,

    /// 流动性池储备（输入代币）
    pub reserve_in: u64,

    /// 流动性池储备（输出代币）
    pub reserve_out: u64,
}

impl SlippageProtector {
    /// 创建新的滑点保护器
    ///
    /// # 参数
    /// - `max_slippage_bps`: 最大允许滑点（100 = 1%）
    /// - `dynamic_adjustment`: 是否启用动态调整
    pub fn new(max_slippage_bps: u16, dynamic_adjustment: bool) -> Self {
        Self {
            max_slippage_bps,
            dynamic_adjustment,
            dynamic_factor: 1.0,
        }
    }

    /// 创建默认保护器（3%最大滑点，启用动态调整）
    pub fn default() -> Self {
        Self::new(300, true)
    }

    /// 计算价格影响
    ///
    /// 使用恒定乘积做市商（CPMM）公式: x * y = k
    ///
    /// # 公式
    /// price_impact = (amount_in / reserve_in) * 100%
    ///
    /// # 参数
    /// - `amount_in`: 输入金额
    /// - `reserve_in`: 输入代币储备
    /// - `reserve_out`: 输出代币储备
    ///
    /// # 返回
    /// 价格影响（basis points, 100 = 1%）
    pub fn calculate_price_impact(
        &self,
        amount_in: u64,
        reserve_in: u64,
        reserve_out: u64,
    ) -> u16 {
        if reserve_in == 0 || reserve_out == 0 {
            return u16::MAX; // 无效池子，返回最大值
        }

        // 计算价格影响百分比
        let impact_ratio = amount_in as f64 / reserve_in as f64;
        let price_impact_bps = (impact_ratio * 10000.0) as u16;

        // 限制最大值
        min(price_impact_bps, 10000) // 最多100%
    }

    /// 计算实际输出金额（考虑AMM公式）
    ///
    /// # 公式
    /// amount_out = (reserve_out * amount_in) / (reserve_in + amount_in)
    ///
    /// 考虑交易费用（通常为0.3%）
    ///
    /// # 参数
    /// - `amount_in`: 输入金额
    /// - `reserve_in`: 输入代币储备
    /// - `reserve_out`: 输出代币储备
    /// - `fee_bps`: 交易费用（basis points, 30 = 0.3%）
    pub fn calculate_output_amount(
        &self,
        amount_in: u64,
        reserve_in: u64,
        reserve_out: u64,
        fee_bps: u16,
    ) -> u64 {
        if reserve_in == 0 || reserve_out == 0 {
            return 0;
        }

        // 扣除交易费用
        let amount_in_after_fee = amount_in as f64 * (1.0 - fee_bps as f64 / 10000.0);

        // AMM公式: amount_out = (reserve_out * amount_in_after_fee) / (reserve_in + amount_in_after_fee)
        let numerator = reserve_out as f64 * amount_in_after_fee;
        let denominator = reserve_in as f64 + amount_in_after_fee;

        (numerator / denominator) as u64
    }

    /// 动态调整滑点（根据流动性）
    ///
    /// 流动性越低，允许的滑点越高（因为价格影响必然更大）
    ///
    /// # 规则
    /// - 流动性 >= 100 SOL: 使用基准滑点
    /// - 流动性 50-100 SOL: 基准滑点 * 1.2
    /// - 流动性 20-50 SOL: 基准滑点 * 1.5
    /// - 流动性 < 20 SOL: 基准滑点 * 2.0
    ///
    /// # 参数
    /// - `liquidity_sol`: 池子流动性（SOL）
    pub fn adjust_slippage_for_liquidity(&self, liquidity_sol: f64) -> u16 {
        if !self.dynamic_adjustment {
            return self.max_slippage_bps;
        }

        let multiplier = if liquidity_sol >= 100.0 {
            1.0
        } else if liquidity_sol >= 50.0 {
            1.2
        } else if liquidity_sol >= 20.0 {
            1.5
        } else {
            2.0
        };

        let adjusted_slippage = (self.max_slippage_bps as f64 * multiplier) as u16;

        // 限制最大值为10%（1000 bps）
        min(adjusted_slippage, 1000)
    }

    /// 验证swap报价
    ///
    /// 检查价格影响和滑点是否在可接受范围内
    ///
    /// # 参数
    /// - `amount_in`: 输入金额
    /// - `reserve_in`: 输入代币储备
    /// - `reserve_out`: 输出代币储备
    /// - `liquidity_sol`: 池子流动性（用于动态调整）
    /// - `fee_bps`: 交易费用
    pub fn validate_swap_quote(
        &self,
        amount_in: u64,
        reserve_in: u64,
        reserve_out: u64,
        liquidity_sol: f64,
        fee_bps: u16,
    ) -> Result<SwapQuote> {
        // 1. 计算价格影响
        let price_impact_bps = self.calculate_price_impact(amount_in, reserve_in, reserve_out);

        // 2. 动态调整滑点
        let effective_slippage_bps = self.adjust_slippage_for_liquidity(liquidity_sol);

        // 3. 计算预期输出
        let expected_out = self.calculate_output_amount(amount_in, reserve_in, reserve_out, fee_bps);

        // 4. 计算最小输出（考虑滑点）
        let slippage_multiplier = 1.0 - (effective_slippage_bps as f64 / 10000.0);
        let min_amount_out = (expected_out as f64 * slippage_multiplier) as u64;

        // 5. 检查价格影响是否过高
        if price_impact_bps > 1000 {
            // 价格影响 > 10%，警告
            tracing::warn!(
                "High price impact: {:.2}% (amount_in={}, reserve_in={})",
                price_impact_bps as f64 / 100.0,
                amount_in,
                reserve_in
            );
        }

        // 6. 检查价格影响是否超过滑点限制
        if price_impact_bps > effective_slippage_bps {
            return Err(solsniper_core::Error::Internal(format!(
                "Price impact ({:.2}%) exceeds max slippage ({:.2}%)",
                price_impact_bps as f64 / 100.0,
                effective_slippage_bps as f64 / 100.0
            )));
        }

        Ok(SwapQuote {
            amount_in,
            expected_out,
            min_amount_out,
            price_impact_bps,
            slippage_bps: effective_slippage_bps,
            reserve_in,
            reserve_out,
        })
    }

    /// 检查实际滑点
    ///
    /// 在交易执行后，验证实际输出是否符合预期
    ///
    /// # 参数
    /// - `expected`: 预期输出金额
    /// - `actual`: 实际输出金额
    pub fn check_slippage(&self, expected: u64, actual: u64) -> Result<()> {
        if actual >= expected {
            // 实际输出 >= 预期，无滑点或正滑点
            return Ok(());
        }

        let slippage = expected - actual;
        let slippage_pct = (slippage as f64 / expected as f64) * 100.0;

        if slippage_pct > (self.max_slippage_bps as f64 / 100.0) {
            return Err(solsniper_core::Error::Internal(format!(
                "Actual slippage ({:.2}%) exceeds max allowed ({:.2}%)",
                slippage_pct,
                self.max_slippage_bps as f64 / 100.0
            )));
        }

        Ok(())
    }

    /// 计算最优交易金额
    ///
    /// 找到使价格影响在可接受范围内的最大交易金额
    ///
    /// # 参数
    /// - `desired_amount`: 期望交易金额
    /// - `reserve_in`: 输入代币储备
    /// - `reserve_out`: 输出代币储备
    /// - `max_impact_bps`: 最大允许价格影响
    ///
    /// # 返回
    /// 最优交易金额（可能小于desired_amount）
    pub fn calculate_optimal_amount(
        &self,
        desired_amount: u64,
        reserve_in: u64,
        reserve_out: u64,
        max_impact_bps: u16,
    ) -> u64 {
        // 计算当前价格影响
        let current_impact = self.calculate_price_impact(desired_amount, reserve_in, reserve_out);

        if current_impact <= max_impact_bps {
            // 当前金额可接受
            return desired_amount;
        }

        // 二分搜索最优金额
        let mut low = 0u64;
        let mut high = desired_amount;
        let mut optimal = desired_amount;

        while low <= high {
            let mid = (low + high) / 2;
            let impact = self.calculate_price_impact(mid, reserve_in, reserve_out);

            if impact <= max_impact_bps {
                optimal = mid;
                low = mid + 1;
            } else {
                high = mid.saturating_sub(1);
            }
        }

        optimal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_impact_calculation() {
        let protector = SlippageProtector::default();

        // 场景1: 小额交易，低价格影响
        let impact = protector.calculate_price_impact(
            1_000_000_000, // 1 SOL
            100_000_000_000, // 100 SOL reserve
            1_000_000_000_000, // 1000 Token reserve
        );
        assert_eq!(impact, 100); // 1% price impact

        // 场景2: 大额交易，高价格影响
        let impact = protector.calculate_price_impact(
            10_000_000_000, // 10 SOL
            100_000_000_000, // 100 SOL reserve
            1_000_000_000_000,
        );
        assert_eq!(impact, 1000); // 10% price impact
    }

    #[test]
    fn test_output_amount_calculation() {
        let protector = SlippageProtector::default();

        let output = protector.calculate_output_amount(
            1_000_000_000, // 1 SOL in
            100_000_000_000, // 100 SOL reserve
            1_000_000_000_000, // 1000 Token reserve
            30, // 0.3% fee
        );

        // 预期: (1000 * 1 * 0.997) / (100 + 1 * 0.997) ≈ 9.87 tokens
        assert!(output > 9_800_000_000 && output < 9_900_000_000);
    }

    #[test]
    fn test_dynamic_slippage_adjustment() {
        let protector = SlippageProtector::new(300, true); // 3% base

        // 高流动性: 不调整
        assert_eq!(protector.adjust_slippage_for_liquidity(100.0), 300);

        // 中流动性: 1.2x
        assert_eq!(protector.adjust_slippage_for_liquidity(60.0), 360);

        // 低流动性: 1.5x
        assert_eq!(protector.adjust_slippage_for_liquidity(30.0), 450);

        // 极低流动性: 2.0x
        assert_eq!(protector.adjust_slippage_for_liquidity(10.0), 600);
    }

    #[test]
    fn test_validate_swap_quote() {
        let protector = SlippageProtector::default();

        let quote = protector
            .validate_swap_quote(
                1_000_000_000,     // 1 SOL
                100_000_000_000,   // 100 SOL reserve
                1_000_000_000_000, // 1000 Token reserve
                50.0,              // 50 SOL liquidity
                30,                // 0.3% fee
            )
            .unwrap();

        assert_eq!(quote.amount_in, 1_000_000_000);
        assert!(quote.expected_out > 0);
        assert!(quote.min_amount_out < quote.expected_out);
        assert_eq!(quote.price_impact_bps, 100); // 1%
        assert_eq!(quote.slippage_bps, 360); // 3% * 1.2 for medium liquidity
    }

    #[test]
    fn test_validate_swap_quote_high_impact() {
        let protector = SlippageProtector::default();

        // 尝试交易30 SOL（占池子30%）
        let result = protector.validate_swap_quote(
            30_000_000_000,    // 30 SOL
            100_000_000_000,   // 100 SOL reserve
            1_000_000_000_000, // 1000 Token reserve
            50.0,              // 50 SOL liquidity
            30,
        );

        // 应该失败，因为价格影响(30%)超过最大滑点(3.6%)
        assert!(result.is_err());
    }

    #[test]
    fn test_check_slippage() {
        let protector = SlippageProtector::new(300, false); // 3% max

        // 场景1: 实际 >= 预期，通过
        assert!(protector.check_slippage(100, 100).is_ok());
        assert!(protector.check_slippage(100, 105).is_ok());

        // 场景2: 滑点在范围内，通过
        assert!(protector.check_slippage(100, 98).is_ok()); // 2% slippage

        // 场景3: 滑点超过限制，失败
        assert!(protector.check_slippage(100, 95).is_err()); // 5% slippage
    }

    #[test]
    fn test_calculate_optimal_amount() {
        let protector = SlippageProtector::default();

        // 期望交易50 SOL，但最大允许5%价格影响
        let optimal = protector.calculate_optimal_amount(
            50_000_000_000,    // 50 SOL
            100_000_000_000,   // 100 SOL reserve
            1_000_000_000_000, // 1000 Token reserve
            500,               // 5% max impact
        );

        // 最优金额应该 <= 50 SOL
        assert!(optimal <= 50_000_000_000);

        // 验证最优金额的价格影响 <= 5%
        let impact = protector.calculate_price_impact(optimal, 100_000_000_000, 1_000_000_000_000);
        assert!(impact <= 500);
    }
}
