pub mod patterns;
pub mod recognizer;
pub mod indicators;

use solsniper_core::{TokenInfo, BehaviorPattern, RiskLevel, Result};

pub use patterns::*;
pub use recognizer::BehaviorPatternRecognizer;
pub use indicators::*;

/// 模式匹配结果
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern: BehaviorPattern,
    pub confidence: f64,
    pub matched_indicators: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_recognition() {
        // 基础测试
    }
}
