use crate::{TokenInfo, Result, Error};

/// TokenInfo验证器
///
/// 验证并修复TokenInfo数据，防止:
/// - 除零错误
/// - NaN值
/// - 逻辑不一致
/// - 异常值
pub struct TokenInfoValidator;

impl TokenInfoValidator {
    /// 验证并修复TokenInfo数据
    ///
    /// # 验证项
    /// 1. 基本字段非零检查
    /// 2. 比例字段范围检查 (0-1)
    /// 3. 逻辑一致性检查
    /// 4. NaN值检查
    /// 5. 异常值检测和修正
    pub fn validate_and_sanitize(token: &mut TokenInfo) -> Result<()> {
        // 1. 基本字段验证
        if token.total_supply == 0 {
            return Err(Error::InvalidInput(
                "total_supply cannot be 0".to_string()
            ));
        }

        if token.decimals > 18 {
            tracing::warn!(
                "Token {} has unusual decimals: {}, capping to 18",
                token.symbol,
                token.decimals
            );
            token.decimals = 18;
        }

        // 2. 修正负值
        if token.liquidity_sol < 0.0 {
            tracing::warn!(
                "Token {} has negative liquidity_sol: {}, setting to 0",
                token.symbol,
                token.liquidity_sol
            );
            token.liquidity_sol = 0.0;
        }

        if token.liquidity_usd < 0.0 {
            token.liquidity_usd = 0.0;
        }

        if token.price_usd < 0.0 {
            token.price_usd = 0.0;
        }

        // 3. 比例字段范围检查 (0-1)
        token.top10_ratio = token.top10_ratio.clamp(0.0, 1.0);
        token.top20_ratio = token.top20_ratio.clamp(0.0, 1.0);
        token.top50_ratio = token.top50_ratio.clamp(0.0, 1.0);
        token.sentiment_score = token.sentiment_score.clamp(-1.0, 1.0);

        // 4. 逻辑一致性检查
        if token.circulating_supply > token.total_supply {
            tracing::warn!(
                "Token {} has circulating > total supply ({} > {}), adjusting",
                token.symbol,
                token.circulating_supply,
                token.total_supply
            );
            token.circulating_supply = token.total_supply;
        }

        // Top持有比例应该递增: top10 <= top20 <= top50
        if token.top10_ratio > token.top20_ratio {
            tracing::warn!(
                "Token {} has top10_ratio > top20_ratio, adjusting",
                token.symbol
            );
            token.top20_ratio = token.top10_ratio;
        }

        if token.top20_ratio > token.top50_ratio {
            tracing::warn!(
                "Token {} has top20_ratio > top50_ratio, adjusting",
                token.symbol
            );
            token.top50_ratio = token.top20_ratio;
        }

        // 交易数据逻辑性
        if token.txns_1h_buys + token.txns_1h_sells > token.txns_1h_total {
            tracing::warn!(
                "Token {} has buys+sells > total txns, adjusting",
                token.symbol
            );
            token.txns_1h_total = token.txns_1h_buys + token.txns_1h_sells;
        }

        // 交易量逻辑性: 1h <= 6h <= 24h
        if token.volume_1h > token.volume_6h {
            token.volume_6h = token.volume_1h;
        }
        if token.volume_6h > token.volume_24h {
            token.volume_24h = token.volume_6h;
        }

        // 5. 异常值检测和修正
        if token.price_change_1h.abs() > 10000.0 {
            tracing::warn!(
                "Token {} has extreme price change: {}%, capping to ±1000%",
                token.symbol,
                token.price_change_1h
            );
            token.price_change_1h = token.price_change_1h.clamp(-1000.0, 10000.0);
        }

        if token.price_change_6h.abs() > 10000.0 {
            token.price_change_6h = token.price_change_6h.clamp(-1000.0, 10000.0);
        }

        if token.price_change_24h.abs() > 10000.0 {
            token.price_change_24h = token.price_change_24h.clamp(-1000.0, 10000.0);
        }

        // 6. NaN值检查
        if token.liquidity_sol.is_nan() {
            tracing::warn!("Token {} has NaN liquidity_sol, setting to 0", token.symbol);
            token.liquidity_sol = 0.0;
        }

        if token.liquidity_usd.is_nan() {
            token.liquidity_usd = 0.0;
        }

        if token.price_usd.is_nan() {
            token.price_usd = 0.0;
        }

        if token.volatility_1h.is_nan() {
            token.volatility_1h = 0.0;
        }

        if token.price_change_1h.is_nan() {
            token.price_change_1h = 0.0;
        }

        if token.price_change_6h.is_nan() {
            token.price_change_6h = 0.0;
        }

        if token.price_change_24h.is_nan() {
            token.price_change_24h = 0.0;
        }

        // 7. Infinity检查
        if token.price_usd.is_infinite() {
            tracing::error!("Token {} has infinite price_usd, setting to 0", token.symbol);
            token.price_usd = 0.0;
        }

        if token.liquidity_usd.is_infinite() {
            token.liquidity_usd = 0.0;
        }

        Ok(())
    }

    /// 计算数据质量分数 (0-1)
    ///
    /// 评估TokenInfo数据的完整性和可靠性
    ///
    /// # 扣分项
    /// - 流动性过低
    /// - 持有者过少
    /// - 代币年龄过小
    /// - 交易量过低
    /// - 无社交媒体存在
    pub fn calculate_quality_score(token: &TokenInfo) -> f64 {
        let mut score = 1.0;

        // 流动性评分
        if token.liquidity_sol < 1.0 {
            score -= 0.2;
        } else if token.liquidity_sol < 5.0 {
            score -= 0.1;
        }

        // 持有者评分
        if token.holders_count < 50 {
            score -= 0.15;
        } else if token.holders_count < 100 {
            score -= 0.05;
        }

        // 年龄评分 (太新的代币数据可能不完整)
        if token.age_hours < 0.1 {
            score -= 0.2;
        } else if token.age_hours < 1.0 {
            score -= 0.1;
        }

        // 交易量评分
        if token.txns_1h_total < 10 {
            score -= 0.15;
        } else if token.txns_1h_total < 30 {
            score -= 0.05;
        }

        // 社交媒体评分
        if token.twitter_mentions == 0 && token.telegram_members == 0 {
            score -= 0.1;
        }

        // 价格数据完整性
        if token.price_usd == 0.0 {
            score -= 0.1;
        }

        score.max(0.0)
    }

    /// 检查TokenInfo是否适合ML预测
    ///
    /// ML模型需要完整的数据，缺失字段会导致预测错误
    pub fn is_suitable_for_ml(token: &TokenInfo) -> bool {
        // 基本字段必须有效
        if token.total_supply == 0 || token.price_usd == 0.0 {
            return false;
        }

        // 流动性数据必须完整
        if token.liquidity_sol == 0.0 || token.liquidity_usd == 0.0 {
            return false;
        }

        // 持有者数据必须合理
        if token.holders_count == 0 {
            return false;
        }

        // 交易数据必须存在
        if token.txns_1h_total == 0 {
            return false;
        }

        // 年龄必须足够（至少1分钟）
        if token.age_minutes < 1 {
            return false;
        }

        // 数据质量必须达标
        let quality = Self::calculate_quality_score(token);
        quality >= 0.5
    }

    /// 检测可能的数据源问题
    ///
    /// 返回警告信息列表
    pub fn detect_data_issues(token: &TokenInfo) -> Vec<String> {
        let mut issues = Vec::new();

        // 流动性与价格不匹配
        if token.liquidity_usd > 0.0 && token.liquidity_sol > 0.0 {
            let implied_sol_price = token.liquidity_usd / token.liquidity_sol;
            if implied_sol_price < 50.0 || implied_sol_price > 300.0 {
                issues.push(format!(
                    "Suspicious SOL price implied from liquidity: ${:.2}",
                    implied_sol_price
                ));
            }
        }

        // 持有者与交易数不匹配
        if token.holders_count > 1000 && token.txns_1h_total < 10 {
            issues.push("High holder count but very low transaction volume".to_string());
        }

        // Top10持有比例异常
        if token.top10_ratio > 0.95 {
            issues.push("Extremely concentrated holding (top10 > 95%)".to_string());
        }

        // 买卖税异常
        if token.buy_tax > 50.0 || token.sell_tax > 50.0 {
            issues.push(format!(
                "Unusually high taxes: buy={}%, sell={}%",
                token.buy_tax, token.sell_tax
            ));
        }

        // 价格变化与波动性不匹配
        if token.price_change_1h.abs() > 100.0 && token.volatility_1h < 0.1 {
            issues.push("Large price change but low volatility - data inconsistency".to_string());
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use chrono::Utc;

    fn create_test_token() -> TokenInfo {
        TokenInfo {
            mint: Pubkey::new_unique(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            liquidity_sol: 10.0,
            liquidity_usd: 1500.0,
            lp_locked: true,
            lp_burned: false,
            total_supply: 1_000_000_000,
            circulating_supply: 1_000_000_000,
            holders_count: 100,
            top10_ratio: 0.5,
            top20_ratio: 0.7,
            top50_ratio: 0.85,
            mint_authority_revoked: true,
            freeze_authority_revoked: true,
            buy_tax: 0.0,
            sell_tax: 0.0,
            created_at: Utc::now(),
            age_minutes: 60,
            age_hours: 1.0,
            txns_1h_total: 50,
            txns_1h_buys: 30,
            txns_1h_sells: 20,
            volume_1h: 100.0,
            volume_6h: 500.0,
            volume_24h: 1000.0,
            price_usd: 0.00001,
            price_change_1h: 10.0,
            price_change_6h: 20.0,
            price_change_24h: 50.0,
            volatility_1h: 0.3,
            twitter_mentions: 10,
            telegram_members: 50,
            discord_members: Some(20),
            sentiment_score: 0.5,
            is_verified: false,
            pool_address: Some(Pubkey::new_unique()),
            dex: "Raydium".to_string(),
        }
    }

    #[test]
    fn test_validate_and_sanitize_valid_token() {
        let mut token = create_test_token();
        assert!(TokenInfoValidator::validate_and_sanitize(&mut token).is_ok());
    }

    #[test]
    fn test_validate_zero_total_supply() {
        let mut token = create_test_token();
        token.total_supply = 0;
        assert!(TokenInfoValidator::validate_and_sanitize(&mut token).is_err());
    }

    #[test]
    fn test_sanitize_negative_liquidity() {
        let mut token = create_test_token();
        token.liquidity_sol = -10.0;
        TokenInfoValidator::validate_and_sanitize(&mut token).unwrap();
        assert_eq!(token.liquidity_sol, 0.0);
    }

    #[test]
    fn test_sanitize_inverted_holder_ratios() {
        let mut token = create_test_token();
        token.top10_ratio = 0.8;
        token.top20_ratio = 0.6; // Invalid: should be >= top10
        token.top50_ratio = 0.5; // Invalid: should be >= top20

        TokenInfoValidator::validate_and_sanitize(&mut token).unwrap();

        assert_eq!(token.top10_ratio, 0.8);
        assert_eq!(token.top20_ratio, 0.8); // Adjusted
        assert_eq!(token.top50_ratio, 0.8); // Adjusted
    }

    #[test]
    fn test_sanitize_nan_values() {
        let mut token = create_test_token();
        token.volatility_1h = f64::NAN;
        token.price_change_1h = f64::NAN;

        TokenInfoValidator::validate_and_sanitize(&mut token).unwrap();

        assert_eq!(token.volatility_1h, 0.0);
        assert_eq!(token.price_change_1h, 0.0);
    }

    #[test]
    fn test_sanitize_extreme_price_change() {
        let mut token = create_test_token();
        token.price_change_1h = 50000.0; // 50000% change

        TokenInfoValidator::validate_and_sanitize(&mut token).unwrap();

        assert_eq!(token.price_change_1h, 1000.0); // Capped to 1000%
    }

    #[test]
    fn test_quality_score() {
        let token = create_test_token();
        let score = TokenInfoValidator::calculate_quality_score(&token);

        // Good quality token should score high
        assert!(score >= 0.7);
    }

    #[test]
    fn test_quality_score_low_quality() {
        let mut token = create_test_token();
        token.liquidity_sol = 0.5; // Low liquidity
        token.holders_count = 20; // Few holders
        token.age_hours = 0.05; // Very new
        token.txns_1h_total = 5; // Low activity

        let score = TokenInfoValidator::calculate_quality_score(&token);

        // Poor quality token should score low
        assert!(score < 0.5);
    }

    #[test]
    fn test_is_suitable_for_ml() {
        let token = create_test_token();
        assert!(TokenInfoValidator::is_suitable_for_ml(&token));
    }

    #[test]
    fn test_not_suitable_for_ml_zero_price() {
        let mut token = create_test_token();
        token.price_usd = 0.0;
        assert!(!TokenInfoValidator::is_suitable_for_ml(&token));
    }

    #[test]
    fn test_detect_data_issues() {
        let mut token = create_test_token();
        token.top10_ratio = 0.98; // Extremely concentrated

        let issues = TokenInfoValidator::detect_data_issues(&token);

        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| i.contains("concentrated")));
    }
}
