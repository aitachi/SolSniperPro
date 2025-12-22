use ndarray::Array1;
use solsniper_core::TokenInfo;

/// 特征提取器 - 将 TokenInfo 转换为 ML 可用的特征向量
pub struct FeatureExtractor;

impl FeatureExtractor {
    pub fn new() -> Self {
        Self
    }

    /// 提取特征向量 (50维特征)
    pub fn extract(&self, token: &TokenInfo) -> Array1<f64> {
        let mut features = Vec::with_capacity(50);

        // === 基础特征 (4个) ===
        features.push(token.liquidity_sol);
        features.push((token.total_supply as f64) / 1e9);
        features.push(token.holders_count as f64);
        features.push(token.age_hours);

        // === 持有者分布特征 (3个) ===
        features.push(token.top10_ratio);
        features.push(token.top20_ratio);
        features.push(token.top50_ratio);

        // === 流动性特征 (3个) ===
        features.push(if token.lp_locked { 1.0 } else { 0.0 });
        features.push(if token.lp_burned { 1.0 } else { 0.0 });
        features.push(token.liquidity_usd / token.liquidity_sol.max(0.001)); // SOL价格估算

        // === 合约安全特征 (4个) ===
        features.push(if token.mint_authority_revoked { 1.0 } else { 0.0 });
        features.push(if token.freeze_authority_revoked { 1.0 } else { 0.0 });
        features.push(token.buy_tax);
        features.push(token.sell_tax);

        // === 交易活动特征 (7个) ===
        features.push(token.txns_1h_total as f64);
        features.push(token.txns_1h_buys as f64);
        features.push(token.txns_1h_sells as f64);
        let buy_sell_ratio = if token.txns_1h_sells > 0 {
            token.txns_1h_buys as f64 / token.txns_1h_sells as f64
        } else {
            10.0 // 假设无卖单时比例很高
        };
        features.push(buy_sell_ratio);
        features.push(token.volume_1h);
        features.push(token.volume_6h);
        features.push(token.volume_24h);

        // === 价格特征 (5个) ===
        features.push(token.price_usd);
        features.push(token.price_change_1h);
        features.push(token.price_change_6h);
        features.push(token.price_change_24h);
        features.push(token.volatility_1h);

        // === 社交特征 (4个) ===
        features.push(token.twitter_mentions as f64);
        features.push(token.telegram_members as f64);
        features.push(token.discord_members.unwrap_or(0) as f64);
        features.push(token.sentiment_score);

        // === 衍生特征 (15个) ===

        // 流动性与交易量比率
        let liquidity_volume_ratio = if token.volume_1h > 0.0 {
            token.liquidity_usd / token.volume_1h
        } else {
            100.0
        };
        features.push(liquidity_volume_ratio);

        // 每笔交易平均金额
        let avg_tx_amount = if token.txns_1h_total > 0 {
            token.volume_1h / token.txns_1h_total as f64
        } else {
            0.0
        };
        features.push(avg_tx_amount);

        // 持有者增长速度 (假设初始100人)
        let holder_growth_rate = (token.holders_count as f64 - 100.0) / token.age_hours.max(0.1);
        features.push(holder_growth_rate);

        // 价格动量
        let price_momentum = token.price_change_1h * 0.5 + token.price_change_6h * 0.3 + token.price_change_24h * 0.2;
        features.push(price_momentum);

        // 交易量趋势
        let volume_trend = if token.volume_6h > 0.0 {
            token.volume_1h / (token.volume_6h / 6.0)
        } else {
            1.0
        };
        features.push(volume_trend);

        // 社交热度综合分数
        let social_score = (token.twitter_mentions as f64).ln_1p() * 0.4
            + (token.telegram_members as f64).ln_1p() * 0.3
            + token.sentiment_score * 100.0 * 0.3;
        features.push(social_score);

        // 风险指标: top10持有者数量
        let top10_holders = (token.holders_count as f64 * token.top10_ratio).ceil();
        features.push(top10_holders);

        // 流动性充足性 (每SOL流动性对应的持有者数)
        let liquidity_per_holder = if token.holders_count > 0 {
            token.liquidity_sol / token.holders_count as f64
        } else {
            0.0
        };
        features.push(liquidity_per_holder);

        // 时间衰减因子 (越早风险越高)
        let time_decay = (-token.age_hours / 24.0).exp();
        features.push(time_decay);

        // 完整性分数 (权限撤销 + LP处理)
        let safety_score = (if token.mint_authority_revoked { 1.0 } else { 0.0 })
            + (if token.freeze_authority_revoked { 1.0 } else { 0.0 })
            + (if token.lp_locked { 0.5 } else { 0.0 })
            + (if token.lp_burned { 1.0 } else { 0.0 });
        features.push(safety_score);

        // 验证状态
        features.push(if token.is_verified { 1.0 } else { 0.0 });

        // DEX 类型编码 (独热编码简化版)
        let dex_score = match token.dex.as_str() {
            "Raydium" => 1.0,
            "Orca" => 0.8,
            "Meteora" => 0.6,
            "PumpFun" => 0.4,
            _ => 0.0,
        };
        features.push(dex_score);

        // 交易税费总和
        let total_tax = token.buy_tax + token.sell_tax;
        features.push(total_tax);

        // 供应量集中度 (循环供应/总供应)
        let supply_ratio = token.circulating_supply as f64 / token.total_supply.max(1) as f64;
        features.push(supply_ratio);

        // 活跃度指标 (交易笔数/持有者数)
        let activity_ratio = if token.holders_count > 0 {
            token.txns_1h_total as f64 / token.holders_count as f64
        } else {
            0.0
        };
        features.push(activity_ratio);

        // 确保特征向量长度为50
        assert_eq!(features.len(), 50, "Feature vector must have exactly 50 dimensions");

        Array1::from(features)
    }

    /// 获取特征名称 (用于模型可解释性)
    pub fn get_feature_names(&self) -> Vec<String> {
        vec![
            // 基础特征
            "liquidity_sol".to_string(),
            "total_supply_b".to_string(),
            "holders_count".to_string(),
            "age_hours".to_string(),

            // 持有者分布
            "top10_ratio".to_string(),
            "top20_ratio".to_string(),
            "top50_ratio".to_string(),

            // 流动性特征
            "lp_locked".to_string(),
            "lp_burned".to_string(),
            "sol_price_estimate".to_string(),

            // 合约安全
            "mint_authority_revoked".to_string(),
            "freeze_authority_revoked".to_string(),
            "buy_tax".to_string(),
            "sell_tax".to_string(),

            // 交易活动
            "txns_1h_total".to_string(),
            "txns_1h_buys".to_string(),
            "txns_1h_sells".to_string(),
            "buy_sell_ratio".to_string(),
            "volume_1h".to_string(),
            "volume_6h".to_string(),
            "volume_24h".to_string(),

            // 价格特征
            "price_usd".to_string(),
            "price_change_1h".to_string(),
            "price_change_6h".to_string(),
            "price_change_24h".to_string(),
            "volatility_1h".to_string(),

            // 社交特征
            "twitter_mentions".to_string(),
            "telegram_members".to_string(),
            "discord_members".to_string(),
            "sentiment_score".to_string(),

            // 衍生特征
            "liquidity_volume_ratio".to_string(),
            "avg_tx_amount".to_string(),
            "holder_growth_rate".to_string(),
            "price_momentum".to_string(),
            "volume_trend".to_string(),
            "social_score".to_string(),
            "top10_holders_abs".to_string(),
            "liquidity_per_holder".to_string(),
            "time_decay".to_string(),
            "safety_score".to_string(),
            "is_verified".to_string(),
            "dex_score".to_string(),
            "total_tax".to_string(),
            "supply_ratio".to_string(),
            "activity_ratio".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use chrono::Utc;

    #[test]
    fn test_feature_extraction() {
        let extractor = FeatureExtractor::new();

        let token = TokenInfo {
            mint: Pubkey::new_unique(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            liquidity_sol: 50.0,
            liquidity_usd: 5000.0,
            lp_locked: true,
            lp_burned: true,
            total_supply: 1_000_000_000,
            circulating_supply: 1_000_000_000,
            holders_count: 500,
            top10_ratio: 0.35,
            top20_ratio: 0.55,
            top50_ratio: 0.75,
            mint_authority_revoked: true,
            freeze_authority_revoked: true,
            buy_tax: 0.0,
            sell_tax: 0.0,
            created_at: Utc::now(),
            age_minutes: 30,
            age_hours: 0.5,
            txns_1h_total: 150,
            txns_1h_buys: 100,
            txns_1h_sells: 50,
            volume_1h: 2500.0,
            volume_6h: 8000.0,
            volume_24h: 15000.0,
            price_usd: 0.00001,
            price_change_1h: 25.0,
            price_change_6h: 50.0,
            price_change_24h: 100.0,
            volatility_1h: 0.15,
            twitter_mentions: 50,
            telegram_members: 200,
            discord_members: Some(100),
            sentiment_score: 0.75,
            is_verified: false,
            pool_address: Some(Pubkey::new_unique()),
            dex: "Raydium".to_string(),
        };

        let features = extractor.extract(&token);

        assert_eq!(features.len(), 50);
        assert_eq!(features[0], 50.0); // liquidity_sol
        assert_eq!(features[10], 1.0); // mint_authority_revoked

        let names = extractor.get_feature_names();
        assert_eq!(names.len(), 50);
    }
}
