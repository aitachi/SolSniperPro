use solsniper_core::{TokenInfo, Score};

/// 合约安全分析器
pub struct ContractAnalyzer;

impl ContractAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, token: &TokenInfo) -> Score {
        let mut score = 100.0;
        let mut issues = Vec::new();

        // 铸币权限检查
        if !token.mint_authority_revoked {
            score -= 30.0;
            issues.push("⚠️ 铸币权限未撤销（可增发）".to_string());
        }

        // 冻结权限检查
        if !token.freeze_authority_revoked {
            score -= 25.0;
            issues.push("⚠️ 冻结权限未撤销（可冻结账户）".to_string());
        }

        // 验证检查
        if !token.is_verified {
            score -= 10.0;
            issues.push("⚠️ 合约代码未验证".to_string());
        }

        // 税费检查
        let total_tax = token.buy_tax + token.sell_tax;
        if total_tax > 10.0 {
            score -= 20.0;
            issues.push(format!("⚠️ 高交易税费: {:.1}%", total_tax));
        } else if total_tax > 5.0 {
            score -= 10.0;
            issues.push(format!("⚠️ 中等交易税费: {:.1}%", total_tax));
        }

        // 卖出税高于买入税（可疑）
        if token.sell_tax > token.buy_tax * 1.5 {
            score -= 15.0;
            issues.push("⚠️ 卖出税明显高于买入税".to_string());
        }

        Score {
            value: score.max(0.0),
            issues,
        }
    }
}
