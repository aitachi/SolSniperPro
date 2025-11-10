use solsniper_core::{TokenInfo, Score};

/// 流动性风险分析器
pub struct LiquidityAnalyzer {
    min_liquidity_sol: f64,
    ideal_liquidity_sol: f64,
}

impl LiquidityAnalyzer {
    pub fn new(min_liquidity_sol: f64, ideal_liquidity_sol: f64) -> Self {
        Self {
            min_liquidity_sol,
            ideal_liquidity_sol,
        }
    }

    pub async fn analyze(&self, token: &TokenInfo) -> Score {
        let mut score = 100.0;
        let mut issues = Vec::new();

        // 流动性总量检查
        if token.liquidity_sol < self.min_liquidity_sol {
            score -= 40.0;
            issues.push(format!(
                "🚨 流动性不足: {:.2} SOL (最低 {:.2} SOL)",
                token.liquidity_sol, self.min_liquidity_sol
            ));
        } else if token.liquidity_sol < self.ideal_liquidity_sol {
            score -= 15.0;
            issues.push(format!(
                "⚠️ 流动性偏低: {:.2} SOL",
                token.liquidity_sol
            ));
        } else {
            issues.push(format!("✅ 流动性充足: {:.2} SOL", token.liquidity_sol));
        }

        // LP锁定检查
        if !token.lp_locked && !token.lp_burned {
            score -= 35.0;
            issues.push("🚨 LP未锁定也未燃烧（高Rug风险）".to_string());
        } else if token.lp_burned {
            score += 5.0;
            issues.push("✅ LP已燃烧（永久锁定）".to_string());
        } else if token.lp_locked {
            issues.push("✅ LP已锁定".to_string());
        }

        // 流动性与市值比例
        let liquidity_ratio = token.liquidity_usd / (token.price_usd * token.circulating_supply as f64);
        if liquidity_ratio < 0.05 {
            score -= 20.0;
            issues.push("⚠️ 流动性占比过低（<5%）".to_string());
        }

        Score {
            value: score.max(0.0),
            issues,
        }
    }
}
