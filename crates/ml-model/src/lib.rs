pub mod feature_extractor;
pub mod classifier;
pub mod regressor;
pub mod online_learning;

use ndarray::{Array1, Array2};
use solsniper_core::{TokenInfo, MLPrediction, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use feature_extractor::FeatureExtractor;
pub use classifier::RugPullClassifier;
pub use regressor::GainRegressor;

/// ML 增强策略引擎
pub struct MLEnhancedStrategy {
    /// 分类器: 判断是否为 Rug Pull
    classifier: Arc<RwLock<RugPullClassifier>>,

    /// 回归器: 预测预期涨幅
    regressor: Arc<RwLock<GainRegressor>>,

    /// 特征提取器
    feature_extractor: FeatureExtractor,

    /// 模型版本
    model_version: String,
}

impl MLEnhancedStrategy {
    pub fn new(model_path: &str) -> Result<Self> {
        let classifier = RugPullClassifier::load(model_path)?;
        let regressor = GainRegressor::load(model_path)?;

        Ok(Self {
            classifier: Arc::new(RwLock::new(classifier)),
            regressor: Arc::new(RwLock::new(regressor)),
            feature_extractor: FeatureExtractor::new(),
            model_version: "v2.0.0".to_string(),
        })
    }

    /// 预测代币结果
    pub async fn predict_outcome(&self, token: &TokenInfo) -> Result<MLPrediction> {
        // 1. 提取特征向量
        let features = self.feature_extractor.extract(token);

        // 2. Rug Pull 概率预测
        let classifier = self.classifier.read().await;
        let rug_probability = classifier.predict_proba(&features)?;
        drop(classifier);

        // 3. 预期涨幅预测
        let regressor = self.regressor.read().await;
        let expected_gain = regressor.predict(&features)?;
        drop(regressor);

        // 4. 计算置信度
        let confidence = self.calculate_confidence(&features, rug_probability);

        Ok(MLPrediction {
            is_rug: rug_probability > 0.5,
            rug_probability,
            expected_gain_pct: expected_gain,
            confidence,
            model_version: self.model_version.clone(),
        })
    }

    /// 在线学习: 根据实际结果更新模型
    pub async fn update_from_outcome(
        &self,
        token: &TokenInfo,
        actual_gain: f64,
        is_rug: bool,
    ) -> Result<()> {
        let features = self.feature_extractor.extract(token);

        // 更新分类器
        let mut classifier = self.classifier.write().await;
        classifier.partial_fit(&features, if is_rug { 1.0 } else { 0.0 })?;
        drop(classifier);

        // 更新回归器
        if !is_rug {
            let mut regressor = self.regressor.write().await;
            regressor.partial_fit(&features, actual_gain)?;
            drop(regressor);
        }

        tracing::info!(
            "Model updated with outcome: gain={:.2}%, is_rug={}",
            actual_gain,
            is_rug
        );

        Ok(())
    }

    fn calculate_confidence(&self, features: &Array1<f64>, rug_prob: f64) -> f64 {
        // 基于特征完整性和预测概率计算置信度
        let feature_completeness = features.iter()
            .filter(|&&x| x != 0.0)
            .count() as f64 / features.len() as f64;

        // 预测概率越极端(接近0或1),置信度越高
        let prediction_confidence = (rug_prob - 0.5).abs() * 2.0;

        (feature_completeness * 0.6 + prediction_confidence * 0.4).min(1.0)
    }

    /// 保存模型
    pub async fn save_model(&self, path: &str) -> Result<()> {
        let classifier = self.classifier.read().await;
        classifier.save(path)?;
        drop(classifier);

        let regressor = self.regressor.read().await;
        regressor.save(path)?;
        drop(regressor);

        tracing::info!("Models saved to {}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use chrono::Utc;

    #[tokio::test]
    async fn test_ml_prediction() {
        let strategy = MLEnhancedStrategy::new("./models").unwrap();

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

        let prediction = strategy.predict_outcome(&token).await.unwrap();

        assert!(prediction.confidence > 0.0);
        assert!(prediction.rug_probability >= 0.0 && prediction.rug_probability <= 1.0);
    }
}
