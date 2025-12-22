pub mod strategies;
pub mod matcher;
pub mod position_manager;
pub mod exit_strategy;
pub mod profit_predictor;
pub mod strategy_priority;

use solsniper_core::{TokenInfo, StrategyMatch, RiskScore, Result};
use strategies::*;

pub use position_manager::{PositionManager, PositionManagerConfig, PositionSizingStrategy, PositionSize, TradeHistory};
pub use exit_strategy::{ExitStrategyManager, ExitStrategyConfig, ExitStrategyType, ExitSignal, PositionTracker};
pub use profit_predictor::{ProfitPredictor, ProfitPrediction, HistoricalTrade, TokenFeatures};
pub use strategy_priority::{StrategyPriorityManager, StrategyPriority, FilteredStrategy, StrategySelection};

/// 策略引擎
pub struct StrategyEngine {
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategyEngine {
    pub fn new() -> Self {
        Self {
            strategies: vec![
                Box::new(EarlyBirdStrategy::new()),
                Box::new(LiquidityHunterStrategy::new()),
                Box::new(VolumeExplosionStrategy::new()),
                Box::new(ValueInvestingStrategy::new()),
                Box::new(ContrarianArbitrageStrategy::new()),
                Box::new(TimeBasedArbitrageStrategy::new()),
            ],
        }
    }

    /// 评估代币是否匹配任何策略
    pub async fn evaluate_token(
        &self,
        token: &TokenInfo,
        risk_score: &RiskScore,
    ) -> Result<Vec<StrategyMatch>> {
        let mut matches = Vec::new();

        for strategy in &self.strategies {
            if strategy.matches(token, risk_score).await {
                let position_size = strategy.calculate_position_size(token, risk_score).await;
                let expected_profit = strategy.estimate_expected_profit(token, risk_score).await;
                let risk_reward = strategy.calculate_risk_reward(token, risk_score).await;

                matches.push(StrategyMatch {
                    strategy_name: strategy.name().to_string(),
                    position_size,
                    expected_profit,
                    risk_reward_ratio: risk_reward,
                    confidence: risk_score.confidence,
                });
            }
        }

        // 按预期收益排序
        matches.sort_by(|a, b| {
            b.expected_profit.partial_cmp(&a.expected_profit).unwrap()
        });

        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use chrono::Utc;

    #[tokio::test]
    async fn test_strategy_engine() {
        let engine = StrategyEngine::new();

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
            age_minutes: 8,
            age_hours: 0.13,
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

        let risk_score = solsniper_core::RiskScore {
            total: 85.0,
            breakdown: solsniper_core::ScoreBreakdown {
                contract: solsniper_core::Score { value: 100.0, issues: vec![] },
                liquidity: solsniper_core::Score { value: 90.0, issues: vec![] },
                holder: solsniper_core::Score { value: 80.0, issues: vec![] },
                sentiment: solsniper_core::Score { value: 75.0, issues: vec![] },
                similarity: solsniper_core::Score { value: 70.0, issues: vec![] },
                behavior: solsniper_core::Score { value: 85.0, issues: vec![] },
            },
            confidence: 0.85,
            recommendation: solsniper_core::Recommendation::StrongBuy,
            timestamp: Utc::now(),
        };

        let matches = engine.evaluate_token(&token, &risk_score).await.unwrap();

        println!("Found {} strategy matches", matches.len());
        for m in &matches {
            println!("- {}: {:.2} SOL (expected profit: {:.1}%)",
                m.strategy_name, m.position_size, m.expected_profit);
        }

        assert!(matches.len() > 0);
    }
}
