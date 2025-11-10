use solsniper_core::{TokenInfo, BehaviorPattern, Result};
use crate::{Pattern, PatternMatch, Indicator, PatternLibrary};
use sqlx::PgPool;
use solana_sdk::pubkey::Pubkey;

/// 行为模式识别器
pub struct BehaviorPatternRecognizer {
    /// 已知模式库
    known_patterns: Vec<Pattern>,

    /// 数据库连接(用于查询历史数据)
    db: Option<PgPool>,
}

impl BehaviorPatternRecognizer {
    pub fn new() -> Self {
        Self {
            known_patterns: PatternLibrary::load_all(),
            db: None,
        }
    }

    pub fn with_database(db_url: &str) -> Result<Self> {
        Ok(Self {
            known_patterns: PatternLibrary::load_all(),
            db: Some(
                PgPool::connect_lazy(db_url)
                    .map_err(|e| solsniper_core::Error::Database(e.to_string()))?
            ),
        })
    }

    /// 匹配所有模式
    pub async fn match_patterns(&self, token: &TokenInfo) -> Result<Vec<PatternMatch>> {
        let mut matches = Vec::new();

        for pattern in &self.known_patterns {
            let confidence = self.calculate_pattern_confidence(token, pattern).await?;

            if confidence > pattern.confidence_threshold {
                let matched_indicators = self.get_matched_indicators(token, pattern).await?;

                matches.push(PatternMatch {
                    pattern: BehaviorPattern {
                        name: pattern.name.clone(),
                        description: pattern.description.clone(),
                        risk_level: pattern.risk_level.clone(),
                        confidence,
                        indicators: matched_indicators.clone(),
                    },
                    confidence,
                    matched_indicators,
                });
            }
        }

        // 按置信度排序
        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        Ok(matches)
    }

    /// 计算模式置信度
    async fn calculate_pattern_confidence(
        &self,
        token: &TokenInfo,
        pattern: &Pattern,
    ) -> Result<f64> {
        let mut total_weight = 0.0;
        let mut matched_weight = 0.0;

        for indicator in &pattern.indicators {
            let weight = indicator.weight();
            total_weight += weight;

            if self.matches_indicator(token, indicator).await? {
                matched_weight += weight;
            }
        }

        Ok(if total_weight > 0.0 {
            matched_weight / total_weight
        } else {
            0.0
        })
    }

    /// 检查是否匹配指标
    async fn matches_indicator(
        &self,
        token: &TokenInfo,
        indicator: &Indicator,
    ) -> Result<bool> {
        match indicator {
            Indicator::SuddenLiquidityDrop { threshold_pct } => {
                // 简化: 检查流动性变化
                // 实际应该查询历史数据
                let current_liq = token.liquidity_sol;
                // TODO: 查询初始流动性
                let initial_liq = current_liq * 2.0; // 假设
                let drop_pct = (initial_liq - current_liq) / initial_liq * 100.0;
                Ok(drop_pct > *threshold_pct)
            }

            Indicator::CreatorSellOff { threshold_pct } => {
                // TODO: 识别创建者地址并检查其卖出行为
                Ok(false)
            }

            Indicator::LpUnlock { time_after_launch_hours } => {
                Ok(!token.lp_locked && token.age_hours >= *time_after_launch_hours as f64)
            }

            Indicator::CoordinatedBuying { wallet_count, timeframe_seconds } => {
                // TODO: 查询最近的买入交易
                Ok(false)
            }

            Indicator::VolumeSpike { multiplier } => {
                let avg_volume = token.volume_6h / 6.0;
                Ok(token.volume_1h >= avg_volume * multiplier)
            }

            Indicator::PriceParabolicRise { slope } => {
                Ok(token.price_change_1h > *slope * 100.0)
            }

            Indicator::OrganicGrowth { holder_increase_rate } => {
                // 假设初始100个持有者
                let initial_holders = 100.0;
                let growth_rate = (token.holders_count as f64 - initial_holders)
                    / token.age_hours.max(0.1);
                Ok(growth_rate >= *holder_increase_rate)
            }

            Indicator::SteadyVolume { variance } => {
                // TODO: 计算交易量方差
                Ok(token.volatility_1h <= *variance)
            }

            Indicator::DistributedHolding { max_top10_ratio } => {
                Ok(token.top10_ratio <= *max_top10_ratio)
            }

            Indicator::WashTrading { same_wallet_ratio } => {
                // TODO: 检测同一钱包反复交易
                Ok(false)
            }

            Indicator::FakeVolume { suspicious_tx_ratio } => {
                // TODO: 识别可疑交易
                Ok(false)
            }
        }
    }

    /// 获取已匹配的指标
    async fn get_matched_indicators(
        &self,
        token: &TokenInfo,
        pattern: &Pattern,
    ) -> Result<Vec<String>> {
        let mut matched = Vec::new();

        for indicator in &pattern.indicators {
            if self.matches_indicator(token, indicator).await? {
                matched.push(format!("{:?}", indicator));
            }
        }

        Ok(matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_pattern_recognition() {
        let recognizer = BehaviorPatternRecognizer::new();

        let token = TokenInfo {
            mint: Pubkey::new_unique(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            liquidity_sol: 10.0, // 低流动性
            liquidity_usd: 1000.0,
            lp_locked: false, // 未锁定
            lp_burned: false,
            total_supply: 1_000_000_000,
            circulating_supply: 1_000_000_000,
            holders_count: 50,
            top10_ratio: 0.8, // 高度集中
            top20_ratio: 0.9,
            top50_ratio: 0.95,
            mint_authority_revoked: false, // 未撤销权限
            freeze_authority_revoked: false,
            buy_tax: 5.0,
            sell_tax: 10.0, // 高卖出税
            created_at: Utc::now(),
            age_minutes: 200,
            age_hours: 3.3,
            txns_1h_total: 20,
            txns_1h_buys: 5,
            txns_1h_sells: 15, // 卖压大
            volume_1h: 500.0,
            volume_6h: 2000.0,
            volume_24h: 5000.0,
            price_usd: 0.00001,
            price_change_1h: -30.0, // 价格下跌
            price_change_6h: -50.0,
            price_change_24h: -70.0,
            volatility_1h: 0.8,
            twitter_mentions: 5,
            telegram_members: 20,
            discord_members: Some(10),
            sentiment_score: 0.2, // 负面情绪
            is_verified: false,
            pool_address: Some(Pubkey::new_unique()),
            dex: "Raydium".to_string(),
        };

        let matches = recognizer.match_patterns(&token).await.unwrap();

        println!("Found {} pattern matches", matches.len());
        for m in &matches {
            println!(
                "Pattern: {} (confidence: {:.2}%)",
                m.pattern.name,
                m.confidence * 100.0
            );
        }

        // 这个代币特征应该匹配 Rug Pull 模式
        assert!(matches.iter().any(|m| m.pattern.name.contains("Rug")));
    }
}
