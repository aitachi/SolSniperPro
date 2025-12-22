pub mod contract_analyzer;
pub mod liquidity_analyzer;
pub mod holder_analyzer;

use solsniper_core::{TokenInfo, RiskScore, Score, ScoreBreakdown, Recommendation, Result};
use solsniper_ml_model::MLEnhancedStrategy;
use solsniper_behavior_pattern::BehaviorPatternRecognizer;
use std::sync::Arc;
use chrono::Utc;

/// 综合风险评估引擎
pub struct RiskAssessmentEngine {
    /// 合约安全分析器
    contract_analyzer: contract_analyzer::ContractAnalyzer,

    /// 流动性风险分析器
    liquidity_analyzer: liquidity_analyzer::LiquidityAnalyzer,

    /// 持有者分布分析器
    holder_analyzer: holder_analyzer::HolderAnalyzer,

    /// ML模型（可选）
    ml_strategy: Option<Arc<MLEnhancedStrategy>>,

    /// 行为模式识别器
    pattern_recognizer: Arc<BehaviorPatternRecognizer>,
}

impl RiskAssessmentEngine {
    pub fn new() -> Self {
        Self {
            contract_analyzer: contract_analyzer::ContractAnalyzer::new(),
            liquidity_analyzer: liquidity_analyzer::LiquidityAnalyzer::new(10.0, 50.0),
            holder_analyzer: holder_analyzer::HolderAnalyzer::new(0.6, 0.8),
            ml_strategy: None,
            pattern_recognizer: Arc::new(BehaviorPatternRecognizer::new()),
        }
    }

    pub fn with_ml(mut self, ml_path: &str) -> Result<Self> {
        self.ml_strategy = Some(Arc::new(MLEnhancedStrategy::new(ml_path)?));
        Ok(self)
    }

    /// 综合评估代币风险
    pub async fn assess(&self, token: &TokenInfo) -> Result<RiskScore> {
        // 并行执行所有分析器
        let (
            contract_score,
            liquidity_score,
            holder_score,
        ) = tokio::join!(
            self.contract_analyzer.analyze(token),
            self.liquidity_analyzer.analyze(token),
            self.holder_analyzer.analyze(token),
        );

        // ML预测
        let ml_adjustment = if let Some(ml) = &self.ml_strategy {
            let prediction = ml.predict_outcome(token).await?;
            // 如果ML预测是Rug,降低总分
            if prediction.is_rug {
                -20.0 * prediction.confidence
            } else {
                5.0 * prediction.confidence
            }
        } else {
            0.0
        };

        // 行为模式检测
        let pattern_matches = self.pattern_recognizer.match_patterns(token).await?;
        let pattern_penalty = pattern_matches.iter()
            .filter(|m| matches!(m.pattern.risk_level, solsniper_core::RiskLevel::High | solsniper_core::RiskLevel::Critical))
            .map(|m| m.confidence * 15.0)
            .sum::<f64>();

        // 加权计算总分
        let base_score = contract_score.value * 0.35
            + liquidity_score.value * 0.30
            + holder_score.value * 0.25
            + 50.0 * 0.10; // 社交情绪基准分

        let total = (base_score + ml_adjustment - pattern_penalty)
            .max(0.0)
            .min(100.0);

        // 生成推荐
        let recommendation = if total >= 85.0 {
            Recommendation::StrongBuy
        } else if total >= 75.0 {
            Recommendation::Buy
        } else if total >= 60.0 {
            Recommendation::Hold
        } else if total >= 40.0 {
            Recommendation::Avoid
        } else {
            Recommendation::StrongAvoid
        };

        // 计算置信度
        let confidence = self.calculate_confidence(token, total);

        Ok(RiskScore {
            total,
            breakdown: ScoreBreakdown {
                contract: contract_score,
                liquidity: liquidity_score,
                holder: holder_score,
                sentiment: Score { value: 50.0, issues: vec![] },
                similarity: Score { value: 50.0, issues: vec![] },
                behavior: Score {
                    value: 100.0 - pattern_penalty,
                    issues: pattern_matches.iter()
                        .map(|m| format!("{} ({:.1}%)", m.pattern.name, m.confidence * 100.0))
                        .collect(),
                },
            },
            confidence,
            recommendation,
            timestamp: Utc::now(),
        })
    }

    fn calculate_confidence(&self, token: &TokenInfo, score: f64) -> f64 {
        let mut confidence = 0.5; // 基准50%

        // 数据完整性
        if token.holders_count > 100 {
            confidence += 0.1;
        }
        if token.age_hours >= 1.0 {
            confidence += 0.1;
        }
        if token.txns_1h_total >= 50 {
            confidence += 0.1;
        }

        // 评分极端性
        if score > 90.0 || score < 20.0 {
            confidence += 0.15;
        }

        // ML模型置信度
        if self.ml_strategy.is_some() {
            confidence += 0.05;
        }

        confidence.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use chrono::Utc;

    #[tokio::test]
    async fn test_risk_assessment() {
        let engine = RiskAssessmentEngine::new();

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

        let risk_score = engine.assess(&token).await.unwrap();

        println!("Total Risk Score: {:.2}", risk_score.total);
        println!("Recommendation: {:?}", risk_score.recommendation);
        println!("Confidence: {:.1}%", risk_score.confidence * 100.0);

        assert!(risk_score.total >= 0.0 && risk_score.total <= 100.0);
        assert!(risk_score.confidence >= 0.0 && risk_score.confidence <= 1.0);
    }
}
