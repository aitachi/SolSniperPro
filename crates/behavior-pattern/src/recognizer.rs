use solsniper_core::{TokenInfo, BehaviorPattern, Result};
use crate::{Pattern, PatternMatch, Indicator, PatternLibrary};
use sqlx::PgPool;
use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

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
                self.detect_creator_selloff(token, *threshold_pct).await
            }

            Indicator::LpUnlock { time_after_launch_hours } => {
                Ok(!token.lp_locked && token.age_hours >= *time_after_launch_hours as f64)
            }

            Indicator::CoordinatedBuying { wallet_count, timeframe_seconds } => {
                self.detect_coordinated_buying(token, *wallet_count, *timeframe_seconds).await
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
                self.detect_wash_trading(token, *same_wallet_ratio).await
            }

            Indicator::FakeVolume { suspicious_tx_ratio } => {
                self.detect_fake_volume(token, *suspicious_tx_ratio).await
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

    // ========================================
    // Advanced Detection Methods
    // ========================================

    /// 检测创建者卖出行为
    ///
    /// 通过分析以下指标:
    /// 1. 识别创建者地址（pool创建者或最大初始持有者）
    /// 2. 查询创建者的交易历史
    /// 3. 计算卖出比例
    async fn detect_creator_selloff(&self, token: &TokenInfo, threshold_pct: f64) -> Result<bool> {
        // 如果没有数据库连接，使用启发式方法
        if self.db.is_none() {
            return self.heuristic_creator_selloff(token, threshold_pct);
        }

        // TODO: 实际数据库查询实现
        // 1. 从数据库查询pool创建交易，提取创建者地址
        // 2. 查询创建者的所有卖出交易
        // 3. 计算卖出比例

        // 临时启发式方法
        self.heuristic_creator_selloff(token, threshold_pct)
    }

    /// 启发式创建者卖出检测
    ///
    /// 基于间接指标:
    /// - 流动性突降 + top10持有量下降 = 可能创建者卖出
    /// - 高卖压（sells >> buys）+ 价格暴跌
    fn heuristic_creator_selloff(&self, token: &TokenInfo, threshold_pct: f64) -> Result<bool> {
        // 指标1: Top10持有比例是否异常低（可能已卖出）
        let top10_suspiciously_low = token.top10_ratio < 0.3 && token.age_hours < 24.0;

        // 指标2: 流动性相对于总供应量过低（创建者可能移除了流动性）
        let liquidity_too_low = token.liquidity_sol < 5.0 && token.age_hours > 1.0;

        // 指标3: 交易数据显示大量卖出
        let heavy_sell_pressure = if token.txns_1h_total > 0 {
            let sell_ratio = token.txns_1h_sells as f64 / token.txns_1h_total as f64;
            sell_ratio > 0.7 // 70%以上是卖单
        } else {
            false
        };

        // 指标4: 价格持续暴跌
        let price_crash = token.price_change_1h < -30.0 && token.price_change_6h < -50.0;

        // 综合判断：至少满足3个指标
        let indicator_count = [
            top10_suspiciously_low,
            liquidity_too_low,
            heavy_sell_pressure,
            price_crash,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        Ok(indicator_count >= 3)
    }

    /// 检测协同买入（pump组织）
    ///
    /// 特征:
    /// 1. 短时间内多个新钱包买入
    /// 2. 买入金额相近
    /// 3. 买入时间间隔极短
    async fn detect_coordinated_buying(
        &self,
        token: &TokenInfo,
        min_wallet_count: u32,
        timeframe_seconds: u64,
    ) -> Result<bool> {
        if self.db.is_none() {
            return self.heuristic_coordinated_buying(token, min_wallet_count);
        }

        // TODO: 数据库查询实现
        // 查询最近timeframe_seconds秒内的所有买入交易
        // 分析钱包地址、金额、时间间隔

        self.heuristic_coordinated_buying(token, min_wallet_count)
    }

    /// 启发式协同买入检测
    fn heuristic_coordinated_buying(&self, token: &TokenInfo, min_wallet_count: u32) -> Result<bool> {
        // 指标1: 短时间内持有者数量激增
        let holder_growth_rate = token.holders_count as f64 / token.age_hours.max(0.1);
        let rapid_holder_growth = holder_growth_rate > 50.0; // 每小时新增50+持有者

        // 指标2: 交易量暴增但价格涨幅不大（可能是pump准备阶段）
        let volume_spike_without_price = token.volume_1h > 1000.0 && token.price_change_1h < 20.0;

        // 指标3: 买入交易数远多于卖出（组织在建仓）
        let buy_dominated = token.txns_1h_buys > token.txns_1h_sells * 3;

        // 指标4: 持有分布相对均匀（多个钱包平均持有）
        let distributed_holding = token.top10_ratio < 0.6 && token.holders_count > 30;

        // 综合判断
        let indicator_count = [
            rapid_holder_growth,
            volume_spike_without_price,
            buy_dominated,
            distributed_holding,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        Ok(indicator_count >= 2)
    }

    /// 检测洗盘交易
    ///
    /// 特征:
    /// 1. 同一钱包在短时间内反复买入卖出
    /// 2. 交易金额相近
    /// 3. 交易形成循环（A->B->A）
    async fn detect_wash_trading(&self, token: &TokenInfo, max_same_wallet_ratio: f64) -> Result<bool> {
        if self.db.is_none() {
            return self.heuristic_wash_trading(token, max_same_wallet_ratio);
        }

        // TODO: 数据库查询实现
        // 1. 查询最近1小时的所有交易
        // 2. 构建交易图，检测循环
        // 3. 统计同一钱包交易比例

        self.heuristic_wash_trading(token, max_same_wallet_ratio)
    }

    /// 启发式洗盘检测
    fn heuristic_wash_trading(&self, token: &TokenInfo, max_same_wallet_ratio: f64) -> Result<bool> {
        // 指标1: 高交易量但持有者数量不增长
        let high_volume_low_holder_growth = token.volume_1h > 500.0 && token.holders_count < 50;

        // 指标2: 买入和卖出数量几乎相等（持续对倒）
        let balanced_buy_sell = if token.txns_1h_total > 0 {
            let buy_ratio = token.txns_1h_buys as f64 / token.txns_1h_total as f64;
            (buy_ratio - 0.5).abs() < 0.1 // 买卖比例接近50:50
        } else {
            false
        };

        // 指标3: 价格波动大但总体变化小（反复拉升打压）
        let high_volatility_small_change = token.volatility_1h > 0.5 && token.price_change_1h.abs() < 10.0;

        // 指标4: 持有高度集中（少数钱包控制）
        let highly_concentrated = token.top10_ratio > 0.8;

        // 综合判断
        let indicator_count = [
            high_volume_low_holder_growth,
            balanced_buy_sell,
            high_volatility_small_change,
            highly_concentrated,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        Ok(indicator_count >= 3)
    }

    /// 检测虚假交易量
    ///
    /// 特征:
    /// 1. 机器人账户交易（新账户、无历史）
    /// 2. 交易金额整数化（如正好1 SOL、10 SOL）
    /// 3. 交易时间规律（每N秒一次）
    async fn detect_fake_volume(&self, token: &TokenInfo, max_suspicious_ratio: f64) -> Result<bool> {
        if self.db.is_none() {
            return self.heuristic_fake_volume(token, max_suspicious_ratio);
        }

        // TODO: 数据库查询实现
        // 1. 查询所有交易
        // 2. 检测机器人模式：规律时间、整数金额
        // 3. 统计可疑交易比例

        self.heuristic_fake_volume(token, max_suspicious_ratio)
    }

    /// 启发式虚假交易量检测
    fn heuristic_fake_volume(&self, token: &TokenInfo, max_suspicious_ratio: f64) -> Result<bool> {
        // 指标1: 交易量极高但持有者极少（明显刷量）
        let volume_holder_mismatch = token.volume_24h > 10000.0 && token.holders_count < 100;

        // 指标2: 交易数量多但均价很低（大量小额刷量）
        let avg_tx_value = if token.txns_1h_total > 0 {
            token.volume_1h / token.txns_1h_total as f64
        } else {
            0.0
        };
        let many_small_txs = token.txns_1h_total > 100 && avg_tx_value < 0.5;

        // 指标3: 高交易量但社交媒体关注度极低（自然流量不足）
        let low_organic_interest = token.volume_24h > 5000.0
            && token.twitter_mentions < 10
            && token.telegram_members < 100;

        // 指标4: Top10持有者比例极高 + 高交易量（庄家自己刷）
        let insider_wash = token.top10_ratio > 0.9 && token.volume_1h > 1000.0;

        // 综合判断
        let indicator_count = [
            volume_holder_mismatch,
            many_small_txs,
            low_organic_interest,
            insider_wash,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        Ok(indicator_count >= 2)
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
