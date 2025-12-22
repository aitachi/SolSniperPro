use solsniper_core::{Error, Result, TokenInfo, RiskScore, StrategyMatch};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 策略优先级配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPriority {
    /// 策略名称
    pub name: String,

    /// 优先级（0-100，越高越优先）
    pub priority: u8,

    /// 是否启用
    pub enabled: bool,

    /// 最小置信度要求
    pub min_confidence: f64,

    /// 最小风险评分要求
    pub min_risk_score: f64,

    /// 最大仓位限制（SOL）
    pub max_position_sol: f64,

    /// 策略权重（用于组合策略）
    pub weight: f64,
}

impl StrategyPriority {
    pub fn new(name: String) -> Self {
        Self {
            name,
            priority: 50,
            enabled: true,
            min_confidence: 0.5,
            min_risk_score: 60.0,
            max_position_sol: 10.0,
            weight: 1.0,
        }
    }
}

/// 策略筛选结果
#[derive(Debug, Clone)]
pub struct FilteredStrategy {
    /// 策略匹配
    pub strategy_match: StrategyMatch,

    /// 优先级
    pub priority: u8,

    /// 调整后的仓位大小
    pub adjusted_position_size: f64,

    /// 是否通过筛选
    pub passed_filter: bool,

    /// 筛选原因（如果未通过）
    pub filter_reasons: Vec<String>,
}

/// 策略选择结果
#[derive(Debug, Clone)]
pub struct StrategySelection {
    /// 选中的策略
    pub selected_strategy: Option<FilteredStrategy>,

    /// 所有候选策略
    pub candidates: Vec<FilteredStrategy>,

    /// 选择原因
    pub selection_reason: String,
}

/// 策略优先级管理器
///
/// 管理多个策略的优先级、筛选和选择
pub struct StrategyPriorityManager {
    /// 策略优先级配置
    priorities: HashMap<String, StrategyPriority>,

    /// 全局最大仓位限制
    global_max_position: f64,

    /// 启用策略组合模式
    enable_combination: bool,

    /// 组合模式下的最大策略数
    max_combined_strategies: usize,
}

impl StrategyPriorityManager {
    /// 创建新的管理器
    pub fn new() -> Self {
        Self {
            priorities: HashMap::new(),
            global_max_position: 50.0,
            enable_combination: false,
            max_combined_strategies: 3,
        }
    }

    /// 设置全局最大仓位
    pub fn with_global_max_position(mut self, max_position: f64) -> Self {
        self.global_max_position = max_position;
        self
    }

    /// 启用策略组合模式
    pub fn with_combination_mode(mut self, enabled: bool) -> Self {
        self.enable_combination = enabled;
        self
    }

    /// 添加策略优先级配置
    pub fn add_strategy_priority(&mut self, priority: StrategyPriority) {
        self.priorities.insert(priority.name.clone(), priority);
    }

    /// 批量添加策略优先级
    pub fn add_strategies(&mut self, priorities: Vec<StrategyPriority>) {
        for priority in priorities {
            self.add_strategy_priority(priority);
        }
    }

    /// 设置策略优先级
    pub fn set_priority(&mut self, strategy_name: &str, priority: u8) -> Result<()> {
        let config = self
            .priorities
            .get_mut(strategy_name)
            .ok_or_else(|| Error::Internal(format!("Strategy {} not found", strategy_name)))?;

        config.priority = priority;
        tracing::info!("⚙️ Updated priority for {}: {}", strategy_name, priority);
        Ok(())
    }

    /// 启用/禁用策略
    pub fn set_enabled(&mut self, strategy_name: &str, enabled: bool) -> Result<()> {
        let config = self
            .priorities
            .get_mut(strategy_name)
            .ok_or_else(|| Error::Internal(format!("Strategy {} not found", strategy_name)))?;

        config.enabled = enabled;
        tracing::info!(
            "⚙️ {} strategy: {}",
            if enabled { "Enabled" } else { "Disabled" },
            strategy_name
        );
        Ok(())
    }

    /// 筛选和排序策略
    ///
    /// 根据优先级配置筛选候选策略
    pub fn filter_and_rank(
        &self,
        matches: Vec<StrategyMatch>,
        risk_score: &RiskScore,
    ) -> Vec<FilteredStrategy> {
        let mut filtered: Vec<FilteredStrategy> = matches
            .into_iter()
            .filter_map(|strategy_match| {
                self.filter_strategy(strategy_match, risk_score)
            })
            .collect();

        // 按优先级排序
        filtered.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| {
                    b.strategy_match
                        .expected_profit
                        .partial_cmp(&a.strategy_match.expected_profit)
                        .unwrap()
                })
        });

        filtered
    }

    /// 筛选单个策略
    fn filter_strategy(
        &self,
        strategy_match: StrategyMatch,
        risk_score: &RiskScore,
    ) -> Option<FilteredStrategy> {
        let priority_config = self.priorities.get(&strategy_match.strategy_name)?;

        let mut filter_reasons = Vec::new();
        let mut passed = true;

        // 检查是否启用
        if !priority_config.enabled {
            filter_reasons.push("Strategy is disabled".to_string());
            passed = false;
        }

        // 检查置信度
        if strategy_match.confidence < priority_config.min_confidence {
            filter_reasons.push(format!(
                "Confidence {:.2} < required {:.2}",
                strategy_match.confidence, priority_config.min_confidence
            ));
            passed = false;
        }

        // 检查风险评分
        if risk_score.total < priority_config.min_risk_score {
            filter_reasons.push(format!(
                "Risk score {:.1} < required {:.1}",
                risk_score.total, priority_config.min_risk_score
            ));
            passed = false;
        }

        // 调整仓位大小
        let adjusted_position_size = strategy_match
            .position_size
            .min(priority_config.max_position_sol)
            .min(self.global_max_position);

        Some(FilteredStrategy {
            strategy_match,
            priority: priority_config.priority,
            adjusted_position_size,
            passed_filter: passed,
            filter_reasons,
        })
    }

    /// 选择最佳策略
    ///
    /// 从筛选后的策略中选择最优策略
    pub fn select_best_strategy(
        &self,
        filtered: Vec<FilteredStrategy>,
    ) -> StrategySelection {
        // 只保留通过筛选的策略
        let passed: Vec<FilteredStrategy> = filtered
            .iter()
            .filter(|f| f.passed_filter)
            .cloned()
            .collect();

        if passed.is_empty() {
            return StrategySelection {
                selected_strategy: None,
                candidates: filtered,
                selection_reason: "No strategies passed filtering criteria".to_string(),
            };
        }

        // 如果启用组合模式
        if self.enable_combination {
            return self.select_combined_strategy(passed, filtered);
        }

        // 单一策略模式：选择优先级最高的
        let best = passed.into_iter().next().unwrap();

        StrategySelection {
            selected_strategy: Some(best.clone()),
            candidates: filtered,
            selection_reason: format!(
                "Selected {} with priority {} and expected profit {:.2}%",
                best.strategy_match.strategy_name,
                best.priority,
                best.strategy_match.expected_profit
            ),
        }
    }

    /// 选择组合策略
    fn select_combined_strategy(
        &self,
        passed: Vec<FilteredStrategy>,
        all_candidates: Vec<FilteredStrategy>,
    ) -> StrategySelection {
        // 取前N个最高优先级的策略
        let top_strategies: Vec<FilteredStrategy> = passed
            .into_iter()
            .take(self.max_combined_strategies)
            .collect();

        if top_strategies.is_empty() {
            return StrategySelection {
                selected_strategy: None,
                candidates: all_candidates,
                selection_reason: "No strategies available for combination".to_string(),
            };
        }

        // 计算组合权重
        let total_weight: f64 = top_strategies
            .iter()
            .filter_map(|f| self.priorities.get(&f.strategy_match.strategy_name))
            .map(|p| p.weight)
            .sum();

        // 创建组合策略
        let combined_position_size: f64 = top_strategies
            .iter()
            .filter_map(|f| {
                let weight = self.priorities.get(&f.strategy_match.strategy_name)?.weight;
                Some(f.adjusted_position_size * weight / total_weight)
            })
            .sum();

        let combined_expected_profit: f64 = top_strategies
            .iter()
            .filter_map(|f| {
                let weight = self.priorities.get(&f.strategy_match.strategy_name)?.weight;
                Some(f.strategy_match.expected_profit * weight / total_weight)
            })
            .sum();

        let avg_priority: u8 = (top_strategies.iter().map(|f| f.priority as u32).sum::<u32>()
            / top_strategies.len() as u32) as u8;

        let combined = FilteredStrategy {
            strategy_match: StrategyMatch {
                strategy_name: format!("Combined({})", top_strategies.len()),
                position_size: combined_position_size,
                expected_profit: combined_expected_profit,
                risk_reward_ratio: top_strategies
                    .iter()
                    .map(|f| f.strategy_match.risk_reward_ratio)
                    .sum::<f64>()
                    / top_strategies.len() as f64,
                confidence: top_strategies
                    .iter()
                    .map(|f| f.strategy_match.confidence)
                    .sum::<f64>()
                    / top_strategies.len() as f64,
            },
            priority: avg_priority,
            adjusted_position_size: combined_position_size,
            passed_filter: true,
            filter_reasons: Vec::new(),
        };

        let strategy_names: Vec<String> = top_strategies
            .iter()
            .map(|f| f.strategy_match.strategy_name.clone())
            .collect();

        StrategySelection {
            selected_strategy: Some(combined),
            candidates: all_candidates,
            selection_reason: format!(
                "Combined {} strategies: {:?}",
                top_strategies.len(),
                strategy_names
            ),
        }
    }

    /// 获取策略统计
    pub fn get_strategy_stats(&self) -> HashMap<String, StrategyStats> {
        self.priorities
            .iter()
            .map(|(name, config)| {
                (
                    name.clone(),
                    StrategyStats {
                        name: name.clone(),
                        priority: config.priority,
                        enabled: config.enabled,
                        min_confidence: config.min_confidence,
                        min_risk_score: config.min_risk_score,
                        max_position_sol: config.max_position_sol,
                        weight: config.weight,
                    },
                )
            })
            .collect()
    }

    /// 获取启用的策略列表
    pub fn get_enabled_strategies(&self) -> Vec<String> {
        self.priorities
            .iter()
            .filter(|(_, config)| config.enabled)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// 重置所有优先级
    pub fn reset_priorities(&mut self) {
        for (_, config) in self.priorities.iter_mut() {
            config.priority = 50;
        }
        tracing::info!("🔄 Reset all strategy priorities to 50");
    }
}

/// 策略统计
#[derive(Debug, Clone)]
pub struct StrategyStats {
    pub name: String,
    pub priority: u8,
    pub enabled: bool,
    pub min_confidence: f64,
    pub min_risk_score: f64,
    pub max_position_sol: f64,
    pub weight: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_match(name: &str, profit: f64, confidence: f64) -> StrategyMatch {
        StrategyMatch {
            strategy_name: name.to_string(),
            position_size: 5.0,
            expected_profit: profit,
            risk_reward_ratio: 2.0,
            confidence,
        }
    }

    fn create_test_risk_score(total: f64) -> RiskScore {
        use chrono::Utc;
        use solsniper_core::{Score, ScoreBreakdown, Recommendation};

        RiskScore {
            total,
            breakdown: ScoreBreakdown {
                contract: Score {
                    value: total,
                    issues: vec![],
                },
                liquidity: Score {
                    value: total,
                    issues: vec![],
                },
                holder: Score {
                    value: total,
                    issues: vec![],
                },
                sentiment: Score {
                    value: total,
                    issues: vec![],
                },
                similarity: Score {
                    value: total,
                    issues: vec![],
                },
                behavior: Score {
                    value: total,
                    issues: vec![],
                },
            },
            confidence: 0.85,
            recommendation: Recommendation::StrongBuy,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_priority_manager_creation() {
        let manager = StrategyPriorityManager::new();
        assert_eq!(manager.priorities.len(), 0);
        assert_eq!(manager.global_max_position, 50.0);
    }

    #[test]
    fn test_add_strategy_priority() {
        let mut manager = StrategyPriorityManager::new();

        let priority = StrategyPriority::new("test_strategy".to_string());
        manager.add_strategy_priority(priority);

        assert_eq!(manager.priorities.len(), 1);
    }

    #[test]
    fn test_filter_and_rank() {
        let mut manager = StrategyPriorityManager::new();

        // Add strategies with different priorities
        let mut high_priority = StrategyPriority::new("high".to_string());
        high_priority.priority = 80;
        high_priority.min_confidence = 0.5;

        let mut low_priority = StrategyPriority::new("low".to_string());
        low_priority.priority = 20;
        low_priority.min_confidence = 0.5;

        manager.add_strategy_priority(high_priority);
        manager.add_strategy_priority(low_priority);

        let matches = vec![
            create_test_match("high", 30.0, 0.8),
            create_test_match("low", 50.0, 0.8),
        ];

        let risk_score = create_test_risk_score(70.0);
        let filtered = manager.filter_and_rank(matches, &risk_score);

        // High priority should be first, even with lower profit
        assert_eq!(filtered[0].strategy_match.strategy_name, "high");
        assert_eq!(filtered[0].priority, 80);
    }

    #[test]
    fn test_confidence_filtering() {
        let mut manager = StrategyPriorityManager::new();

        let mut priority = StrategyPriority::new("test".to_string());
        priority.min_confidence = 0.8; // Require high confidence
        manager.add_strategy_priority(priority);

        let matches = vec![
            create_test_match("test", 30.0, 0.7), // Too low confidence
        ];

        let risk_score = create_test_risk_score(70.0);
        let filtered = manager.filter_and_rank(matches, &risk_score);

        assert_eq!(filtered.len(), 1);
        assert!(!filtered[0].passed_filter);
        assert!(filtered[0].filter_reasons[0].contains("Confidence"));
    }

    #[test]
    fn test_select_best_strategy() {
        let mut manager = StrategyPriorityManager::new();

        let mut high = StrategyPriority::new("high".to_string());
        high.priority = 90;

        let mut low = StrategyPriority::new("low".to_string());
        low.priority = 10;

        manager.add_strategy_priority(high);
        manager.add_strategy_priority(low);

        let matches = vec![
            create_test_match("high", 20.0, 0.8),
            create_test_match("low", 50.0, 0.8),
        ];

        let risk_score = create_test_risk_score(70.0);
        let filtered = manager.filter_and_rank(matches, &risk_score);
        let selection = manager.select_best_strategy(filtered);

        assert!(selection.selected_strategy.is_some());
        assert_eq!(
            selection.selected_strategy.unwrap().strategy_match.strategy_name,
            "high"
        );
    }

    #[test]
    fn test_combined_strategy() {
        let mut manager = StrategyPriorityManager::new()
            .with_combination_mode(true);

        let mut s1 = StrategyPriority::new("strategy1".to_string());
        s1.priority = 80;
        s1.weight = 0.5;

        let mut s2 = StrategyPriority::new("strategy2".to_string());
        s2.priority = 70;
        s2.weight = 0.3;

        manager.add_strategy_priority(s1);
        manager.add_strategy_priority(s2);

        let matches = vec![
            create_test_match("strategy1", 30.0, 0.8),
            create_test_match("strategy2", 20.0, 0.8),
        ];

        let risk_score = create_test_risk_score(70.0);
        let filtered = manager.filter_and_rank(matches, &risk_score);
        let selection = manager.select_best_strategy(filtered);

        assert!(selection.selected_strategy.is_some());
        let selected = selection.selected_strategy.unwrap();
        assert!(selected.strategy_match.strategy_name.starts_with("Combined"));
    }

    #[test]
    fn test_disable_strategy() {
        let mut manager = StrategyPriorityManager::new();

        let priority = StrategyPriority::new("test".to_string());
        manager.add_strategy_priority(priority);

        manager.set_enabled("test", false).unwrap();

        let matches = vec![create_test_match("test", 30.0, 0.8)];
        let risk_score = create_test_risk_score(70.0);
        let filtered = manager.filter_and_rank(matches, &risk_score);

        assert!(!filtered[0].passed_filter);
        assert!(filtered[0].filter_reasons[0].contains("disabled"));
    }

    #[test]
    fn test_get_enabled_strategies() {
        let mut manager = StrategyPriorityManager::new();

        let mut s1 = StrategyPriority::new("enabled".to_string());
        s1.enabled = true;

        let mut s2 = StrategyPriority::new("disabled".to_string());
        s2.enabled = false;

        manager.add_strategy_priority(s1);
        manager.add_strategy_priority(s2);

        let enabled = manager.get_enabled_strategies();
        assert_eq!(enabled.len(), 1);
        assert!(enabled.contains(&"enabled".to_string()));
    }
}
