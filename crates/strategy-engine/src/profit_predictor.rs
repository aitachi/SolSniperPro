use solsniper_core::{Error, Result, TokenInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 历史交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalTrade {
    /// 代币mint地址
    pub token_mint: String,

    /// 入场时间戳
    pub entry_timestamp: i64,

    /// 入场价格
    pub entry_price: f64,

    /// 退出价格
    pub exit_price: f64,

    /// 持有时长（秒）
    pub holding_duration_secs: u64,

    /// 收益率（百分比）
    pub return_pct: f64,

    /// 策略名称
    pub strategy_name: String,

    /// 代币特征（入场时）
    pub features: TokenFeatures,
}

/// 代币特征
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenFeatures {
    pub liquidity_sol: f64,
    pub holders_count: u32,
    pub age_hours: f64,
    pub volume_1h: f64,
    pub price_change_1h: f64,
    pub top10_ratio: f64,
    pub volatility_1h: f64,
    pub buy_sell_ratio: f64,
}

impl TokenFeatures {
    pub fn from_token_info(token: &TokenInfo) -> Self {
        let buy_sell_ratio = if token.txns_1h_sells > 0 {
            token.txns_1h_buys as f64 / token.txns_1h_sells as f64
        } else {
            f64::MAX
        };

        Self {
            liquidity_sol: token.liquidity_sol,
            holders_count: token.holders_count,
            age_hours: token.age_hours,
            volume_1h: token.volume_1h,
            price_change_1h: token.price_change_1h,
            top10_ratio: token.top10_ratio,
            volatility_1h: token.volatility_1h,
            buy_sell_ratio,
        }
    }

    /// 计算特征相似度（0-1，越高越相似）
    pub fn similarity(&self, other: &TokenFeatures) -> f64 {
        let mut similarity_score = 0.0;
        let mut weight_sum = 0.0;

        // 流动性相似度
        let liq_sim = self.calculate_field_similarity(
            self.liquidity_sol,
            other.liquidity_sol,
            100.0, // tolerance
        );
        similarity_score += liq_sim * 0.2;
        weight_sum += 0.2;

        // 持有者相似度
        let holders_sim = self.calculate_field_similarity(
            self.holders_count as f64,
            other.holders_count as f64,
            200.0,
        );
        similarity_score += holders_sim * 0.15;
        weight_sum += 0.15;

        // 年龄相似度
        let age_sim = self.calculate_field_similarity(self.age_hours, other.age_hours, 12.0);
        similarity_score += age_sim * 0.1;
        weight_sum += 0.1;

        // 交易量相似度
        let vol_sim = self.calculate_field_similarity(self.volume_1h, other.volume_1h, 500.0);
        similarity_score += vol_sim * 0.15;
        weight_sum += 0.15;

        // 价格变化相似度
        let price_change_sim = self.calculate_field_similarity(
            self.price_change_1h,
            other.price_change_1h,
            20.0,
        );
        similarity_score += price_change_sim * 0.1;
        weight_sum += 0.1;

        // 集中度相似度
        let concentration_sim = self.calculate_field_similarity(
            self.top10_ratio,
            other.top10_ratio,
            0.2,
        );
        similarity_score += concentration_sim * 0.15;
        weight_sum += 0.15;

        // 波动性相似度
        let vol_sim = self.calculate_field_similarity(
            self.volatility_1h,
            other.volatility_1h,
            0.1,
        );
        similarity_score += vol_sim * 0.15;
        weight_sum += 0.15;

        similarity_score / weight_sum
    }

    /// 计算单个字段的相似度
    fn calculate_field_similarity(&self, val1: f64, val2: f64, tolerance: f64) -> f64 {
        let diff = (val1 - val2).abs();
        let similarity = 1.0 - (diff / tolerance).min(1.0);
        similarity.max(0.0)
    }
}

/// 预测结果
#[derive(Debug, Clone)]
pub struct ProfitPrediction {
    /// 预期收益率（百分比）
    pub expected_return_pct: f64,

    /// 收益率区间（百分比）
    pub return_range: (f64, f64),

    /// 预期持有时长（秒）
    pub expected_holding_secs: u64,

    /// 置信度（0-1）
    pub confidence: f64,

    /// 相似交易数量
    pub similar_trades_count: usize,

    /// 胜率
    pub win_rate: f64,

    /// 风险调整后收益（夏普比率）
    pub sharpe_ratio: f64,

    /// 样本数据统计
    pub stats: PredictionStats,
}

/// 预测统计
#[derive(Debug, Clone)]
pub struct PredictionStats {
    pub avg_return: f64,
    pub median_return: f64,
    pub std_dev: f64,
    pub best_return: f64,
    pub worst_return: f64,
}

/// 收益预测器
///
/// 基于历史数据预测未来收益
pub struct ProfitPredictor {
    /// 历史交易数据
    historical_trades: Vec<HistoricalTrade>,

    /// 最小相似度阈值
    min_similarity: f64,

    /// 最小样本数量
    min_sample_size: usize,
}

impl ProfitPredictor {
    /// 创建新的预测器
    pub fn new() -> Self {
        Self {
            historical_trades: Vec::new(),
            min_similarity: 0.6,
            min_sample_size: 10,
        }
    }

    /// 设置最小相似度阈值
    pub fn with_min_similarity(mut self, min_similarity: f64) -> Self {
        self.min_similarity = min_similarity;
        self
    }

    /// 设置最小样本数量
    pub fn with_min_sample_size(mut self, min_sample_size: usize) -> Self {
        self.min_sample_size = min_sample_size;
        self
    }

    /// 添加历史交易
    pub fn add_historical_trade(&mut self, trade: HistoricalTrade) {
        self.historical_trades.push(trade);
    }

    /// 批量添加历史交易
    pub fn add_historical_trades(&mut self, trades: Vec<HistoricalTrade>) {
        self.historical_trades.extend(trades);
    }

    /// 预测收益
    ///
    /// 基于相似代币的历史表现预测收益
    pub fn predict(&self, token: &TokenInfo, strategy_name: &str) -> Result<ProfitPrediction> {
        if self.historical_trades.is_empty() {
            return Err(Error::Internal(
                "No historical data available for prediction".to_string(),
            ));
        }

        let current_features = TokenFeatures::from_token_info(token);

        // 找到相似的历史交易
        let mut similar_trades: Vec<(&HistoricalTrade, f64)> = self
            .historical_trades
            .iter()
            .filter(|trade| {
                // 可选：筛选同一策略的交易
                strategy_name.is_empty() || trade.strategy_name == strategy_name
            })
            .map(|trade| {
                let similarity = current_features.similarity(&trade.features);
                (trade, similarity)
            })
            .filter(|(_, similarity)| *similarity >= self.min_similarity)
            .collect();

        if similar_trades.len() < self.min_sample_size {
            return Err(Error::Internal(format!(
                "Insufficient similar trades: {} (required: {})",
                similar_trades.len(),
                self.min_sample_size
            )));
        }

        // 按相似度排序
        similar_trades.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // 计算加权统计
        let prediction = self.calculate_weighted_prediction(&similar_trades)?;

        tracing::info!(
            "📊 Profit prediction: expected={:.2}%, confidence={:.2}, similar_trades={}",
            prediction.expected_return_pct,
            prediction.confidence,
            prediction.similar_trades_count
        );

        Ok(prediction)
    }

    /// 计算加权预测
    fn calculate_weighted_prediction(
        &self,
        similar_trades: &[(&HistoricalTrade, f64)],
    ) -> Result<ProfitPrediction> {
        let mut returns: Vec<f64> = Vec::new();
        let mut holding_times: Vec<u64> = Vec::new();
        let mut total_weight = 0.0;
        let mut weighted_return = 0.0;
        let mut weighted_holding_time = 0.0;

        for (trade, similarity) in similar_trades {
            let weight = *similarity;
            returns.push(trade.return_pct);
            holding_times.push(trade.holding_duration_secs);

            weighted_return += trade.return_pct * weight;
            weighted_holding_time += trade.holding_duration_secs as f64 * weight;
            total_weight += weight;
        }

        let expected_return = weighted_return / total_weight;
        let expected_holding = (weighted_holding_time / total_weight) as u64;

        // 计算统计数据
        let stats = self.calculate_stats(&returns);

        // 计算置信区间（使用标准差）
        let confidence_interval = 1.96 * stats.std_dev; // 95% 置信区间
        let return_range = (
            expected_return - confidence_interval,
            expected_return + confidence_interval,
        );

        // 计算胜率
        let wins = returns.iter().filter(|&&r| r > 0.0).count();
        let win_rate = wins as f64 / returns.len() as f64;

        // 计算夏普比率（简化版：假设无风险利率为0）
        let sharpe_ratio = if stats.std_dev > 0.0 {
            stats.avg_return / stats.std_dev
        } else {
            0.0
        };

        // 计算置信度
        let confidence = self.calculate_confidence(similar_trades.len(), stats.std_dev, win_rate);

        Ok(ProfitPrediction {
            expected_return_pct: expected_return,
            return_range,
            expected_holding_secs: expected_holding,
            confidence,
            similar_trades_count: similar_trades.len(),
            win_rate,
            sharpe_ratio,
            stats,
        })
    }

    /// 计算统计数据
    fn calculate_stats(&self, returns: &[f64]) -> PredictionStats {
        if returns.is_empty() {
            return PredictionStats {
                avg_return: 0.0,
                median_return: 0.0,
                std_dev: 0.0,
                best_return: 0.0,
                worst_return: 0.0,
            };
        }

        let sum: f64 = returns.iter().sum();
        let avg_return = sum / returns.len() as f64;

        let mut sorted = returns.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median_return = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        let variance: f64 = returns
            .iter()
            .map(|&r| (r - avg_return).powi(2))
            .sum::<f64>()
            / returns.len() as f64;

        let std_dev = variance.sqrt();

        PredictionStats {
            avg_return,
            median_return,
            std_dev,
            best_return: *sorted.last().unwrap_or(&0.0),
            worst_return: *sorted.first().unwrap_or(&0.0),
        }
    }

    /// 计算置信度
    fn calculate_confidence(&self, sample_size: usize, std_dev: f64, win_rate: f64) -> f64 {
        // 基于样本量的置信度
        let sample_confidence = (sample_size as f64 / (sample_size as f64 + 50.0)).min(1.0);

        // 基于标准差的置信度（低波动 = 高置信度）
        let volatility_confidence = (1.0 / (1.0 + std_dev / 50.0)).min(1.0);

        // 基于胜率的置信度
        let win_rate_confidence = win_rate;

        // 综合置信度
        (sample_confidence * 0.4 + volatility_confidence * 0.3 + win_rate_confidence * 0.3).min(1.0)
    }

    /// 获取策略表现统计
    pub fn get_strategy_performance(&self, strategy_name: &str) -> HashMap<String, f64> {
        let strategy_trades: Vec<&HistoricalTrade> = self
            .historical_trades
            .iter()
            .filter(|t| t.strategy_name == strategy_name)
            .collect();

        if strategy_trades.is_empty() {
            return HashMap::new();
        }

        let returns: Vec<f64> = strategy_trades.iter().map(|t| t.return_pct).collect();
        let stats = self.calculate_stats(&returns);

        let wins = returns.iter().filter(|&&r| r > 0.0).count();
        let win_rate = wins as f64 / returns.len() as f64;

        let profit_factor = {
            let total_profit: f64 = returns.iter().filter(|&&r| r > 0.0).sum();
            let total_loss: f64 = returns.iter().filter(|&&r| r < 0.0).sum::<f64>().abs();
            if total_loss > 0.0 {
                total_profit / total_loss
            } else {
                0.0
            }
        };

        let mut performance = HashMap::new();
        performance.insert("total_trades".to_string(), strategy_trades.len() as f64);
        performance.insert("win_rate".to_string(), win_rate);
        performance.insert("avg_return".to_string(), stats.avg_return);
        performance.insert("median_return".to_string(), stats.median_return);
        performance.insert("std_dev".to_string(), stats.std_dev);
        performance.insert("best_return".to_string(), stats.best_return);
        performance.insert("worst_return".to_string(), stats.worst_return);
        performance.insert("profit_factor".to_string(), profit_factor);
        performance.insert(
            "sharpe_ratio".to_string(),
            if stats.std_dev > 0.0 {
                stats.avg_return / stats.std_dev
            } else {
                0.0
            },
        );

        performance
    }

    /// 获取历史交易数量
    pub fn get_historical_count(&self) -> usize {
        self.historical_trades.len()
    }

    /// 清空历史数据
    pub fn clear_history(&mut self) {
        self.historical_trades.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_trade(
        return_pct: f64,
        liquidity: f64,
        holders: u32,
        strategy: &str,
    ) -> HistoricalTrade {
        HistoricalTrade {
            token_mint: "test_mint".to_string(),
            entry_timestamp: 1000000,
            entry_price: 1.0,
            exit_price: 1.0 + return_pct / 100.0,
            holding_duration_secs: 3600,
            return_pct,
            strategy_name: strategy.to_string(),
            features: TokenFeatures {
                liquidity_sol: liquidity,
                holders_count: holders,
                age_hours: 1.0,
                volume_1h: 100.0,
                price_change_1h: 5.0,
                top10_ratio: 0.3,
                volatility_1h: 0.05,
                buy_sell_ratio: 2.0,
            },
        }
    }

    fn create_test_token() -> TokenInfo {
        use chrono::Utc;
        TokenInfo {
            mint: solana_sdk::pubkey::Pubkey::new_unique(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            total_supply: 1_000_000,
            circulating_supply: 800_000,
            price_usd: 0.001,
            market_cap_usd: 800.0,
            liquidity_sol: 50.0,
            liquidity_usd: 7500.0,
            volume_24h: 1000.0,
            volume_1h: 100.0,
            volume_6h: 500.0,
            price_change_1h: 5.0,
            price_change_6h: 10.0,
            price_change_24h: 20.0,
            holders_count: 500,
            top10_ratio: 0.3,
            top20_ratio: 0.45,
            top50_ratio: 0.65,
            dex: "Raydium".to_string(),
            pool_address: Some(solana_sdk::pubkey::Pubkey::new_unique()),
            creator: Some(solana_sdk::pubkey::Pubkey::new_unique()),
            age_minutes: 60.0,
            age_hours: 1.0,
            buy_tax: 0.0,
            sell_tax: 0.0,
            is_renounced: true,
            is_frozen: false,
            txns_1h_buys: 50,
            txns_1h_sells: 30,
            txns_1h_total: 80,
            txns_6h_buys: 200,
            txns_6h_sells: 150,
            txns_6h_total: 350,
            volatility_1h: 0.05,
            volatility_6h: 0.08,
            social_twitter: Some("@test".to_string()),
            social_telegram: None,
            social_website: None,
            discovered_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_feature_similarity() {
        let features1 = TokenFeatures {
            liquidity_sol: 50.0,
            holders_count: 500,
            age_hours: 1.0,
            volume_1h: 100.0,
            price_change_1h: 5.0,
            top10_ratio: 0.3,
            volatility_1h: 0.05,
            buy_sell_ratio: 2.0,
        };

        let features2 = TokenFeatures {
            liquidity_sol: 55.0, // Close
            holders_count: 520, // Close
            age_hours: 1.2,     // Close
            volume_1h: 110.0,   // Close
            price_change_1h: 6.0,
            top10_ratio: 0.32,
            volatility_1h: 0.06,
            buy_sell_ratio: 2.1,
        };

        let similarity = features1.similarity(&features2);
        assert!(similarity > 0.8); // Should be very similar
    }

    #[test]
    fn test_profit_prediction() {
        let mut predictor = ProfitPredictor::new()
            .with_min_similarity(0.5)
            .with_min_sample_size(5);

        // Add similar profitable trades
        for i in 0..10 {
            predictor.add_historical_trade(create_test_trade(
                20.0 + i as f64 * 5.0, // 20% to 65% returns
                50.0,
                500,
                "test_strategy",
            ));
        }

        let token = create_test_token();
        let prediction = predictor.predict(&token, "").unwrap();

        assert!(prediction.expected_return_pct > 0.0);
        assert!(prediction.confidence > 0.0);
        assert_eq!(prediction.similar_trades_count, 10);
        assert!(prediction.win_rate > 0.0);
    }

    #[test]
    fn test_insufficient_data() {
        let mut predictor = ProfitPredictor::new()
            .with_min_sample_size(10);

        // Only add 5 trades
        for _ in 0..5 {
            predictor.add_historical_trade(create_test_trade(20.0, 50.0, 500, "test"));
        }

        let token = create_test_token();
        let result = predictor.predict(&token, "");

        assert!(result.is_err());
    }

    #[test]
    fn test_strategy_performance() {
        let mut predictor = ProfitPredictor::new();

        predictor.add_historical_trade(create_test_trade(30.0, 50.0, 500, "early_bird"));
        predictor.add_historical_trade(create_test_trade(-10.0, 50.0, 500, "early_bird"));
        predictor.add_historical_trade(create_test_trade(50.0, 50.0, 500, "early_bird"));

        let perf = predictor.get_strategy_performance("early_bird");

        assert_eq!(perf.get("total_trades").unwrap(), &3.0);
        assert!(perf.get("win_rate").unwrap() > &0.6);
        assert!(perf.get("avg_return").unwrap() > &0.0);
    }

    #[test]
    fn test_stats_calculation() {
        let predictor = ProfitPredictor::new();
        let returns = vec![10.0, 20.0, -5.0, 30.0, 15.0];

        let stats = predictor.calculate_stats(&returns);

        assert_eq!(stats.avg_return, 14.0);
        assert_eq!(stats.median_return, 15.0);
        assert_eq!(stats.best_return, 30.0);
        assert_eq!(stats.worst_return, -5.0);
        assert!(stats.std_dev > 0.0);
    }
}
