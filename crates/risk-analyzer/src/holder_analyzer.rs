use solsniper_core::{TokenInfo, Score};

/// 持有者分布分析器
pub struct HolderAnalyzer {
    max_top10_ratio: f64,
    max_top20_ratio: f64,
}

impl HolderAnalyzer {
    pub fn new(max_top10_ratio: f64, max_top20_ratio: f64) -> Self {
        Self {
            max_top10_ratio,
            max_top20_ratio,
        }
    }

    pub async fn analyze(&self, token: &TokenInfo) -> Score {
        let mut score = 100.0;
        let mut issues = Vec::new();

        // Top10持有比例
        if token.top10_ratio > 0.8 {
            score -= 40.0;
            issues.push(format!(
                "🚨 Top10 持有 {:.1}%（高度集中）",
                token.top10_ratio * 100.0
            ));
        } else if token.top10_ratio > self.max_top10_ratio {
            score -= 25.0;
            issues.push(format!(
                "⚠️ Top10 持有 {:.1}%（集中度偏高）",
                token.top10_ratio * 100.0
            ));
        } else if token.top10_ratio < 0.3 {
            score += 10.0;
            issues.push(format!(
                "✅ Top10 持有 {:.1}%（分布良好）",
                token.top10_ratio * 100.0
            ));
        }

        // Top20持有比例
        if token.top20_ratio > 0.9 {
            score -= 20.0;
            issues.push("⚠️ Top20持有超过90%".to_string());
        }

        // 持有者总数
        if token.holders_count < 50 {
            score -= 25.0;
            issues.push(format!(
                "⚠️ 持有者较少: {} 个",
                token.holders_count
            ));
        } else if token.holders_count > 500 {
            score += 10.0;
            issues.push(format!(
                "✅ 持有者众多: {} 个",
                token.holders_count
            ));
        }

        // 持有者增长速度
        if token.age_hours > 1.0 {
            let growth_rate = (token.holders_count as f64 - 100.0) / token.age_hours;
            if growth_rate > 50.0 {
                issues.push(format!(
                    "✅ 持有者快速增长: {:.1} 人/小时",
                    growth_rate
                ));
            } else if growth_rate < 5.0 {
                score -= 15.0;
                issues.push("⚠️ 持有者增长缓慢".to_string());
            }
        }

        Score {
            value: score.max(0.0),
            issues,
        }
    }
}
