use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;

/// 指标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    /// 计数器（单调递增）
    Counter,
    /// 仪表盘（可增可减）
    Gauge,
    /// 直方图（分布统计）
    Histogram,
    /// 摘要（统计信息）
    Summary,
}

/// 指标标签
pub type MetricLabels = HashMap<String, String>;

/// 指标值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(HistogramData),
    Summary(SummaryData),
}

/// 直方图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramData {
    /// 样本数量
    pub count: u64,
    /// 总和
    pub sum: f64,
    /// 分桶数据 (upper_bound, count)
    pub buckets: Vec<(f64, u64)>,
}

impl HistogramData {
    pub fn new(buckets: Vec<f64>) -> Self {
        Self {
            count: 0,
            sum: 0.0,
            buckets: buckets.into_iter().map(|b| (b, 0)).collect(),
        }
    }

    pub fn observe(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;

        for (upper_bound, count) in &mut self.buckets {
            if value <= *upper_bound {
                *count += 1;
            }
        }
    }

    pub fn mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}

/// 摘要数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryData {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub percentiles: HashMap<String, f64>, // "p50", "p95", "p99"
}

impl Default for SummaryData {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            mean: 0.0,
            std_dev: 0.0,
            percentiles: HashMap::new(),
        }
    }
}

/// 指标定义
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub help: String,
    pub labels: MetricLabels,
    pub value: MetricValue,
    pub timestamp: SystemTime,
}

/// 交易指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingMetrics {
    /// 总交易次数
    pub total_trades: u64,

    /// 成功交易次数
    pub successful_trades: u64,

    /// 失败交易次数
    pub failed_trades: u64,

    /// 总盈亏（SOL）
    pub total_pnl_sol: f64,

    /// 总盈亏（USD）
    pub total_pnl_usd: f64,

    /// 胜率
    pub win_rate: f64,

    /// 平均盈利
    pub avg_win: f64,

    /// 平均亏损
    pub avg_loss: f64,

    /// 最大盈利
    pub max_win: f64,

    /// 最大亏损
    pub max_loss: f64,

    /// 盈亏比
    pub profit_factor: f64,

    /// 夏普比率
    pub sharpe_ratio: f64,

    /// 最大回撤
    pub max_drawdown: f64,

    /// 平均持仓时长（秒）
    pub avg_holding_duration_secs: u64,

    /// 最后更新时间
    pub updated_at: SystemTime,
}

impl Default for TradingMetrics {
    fn default() -> Self {
        Self {
            total_trades: 0,
            successful_trades: 0,
            failed_trades: 0,
            total_pnl_sol: 0.0,
            total_pnl_usd: 0.0,
            win_rate: 0.0,
            avg_win: 0.0,
            avg_loss: 0.0,
            max_win: 0.0,
            max_loss: 0.0,
            profit_factor: 0.0,
            sharpe_ratio: 0.0,
            max_drawdown: 0.0,
            avg_holding_duration_secs: 0,
            updated_at: SystemTime::now(),
        }
    }
}

/// 策略性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyMetrics {
    pub strategy_name: String,
    pub total_signals: u64,
    pub executed_trades: u64,
    pub win_rate: f64,
    pub avg_return_pct: f64,
    pub total_pnl: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub updated_at: SystemTime,
}

impl StrategyMetrics {
    pub fn new(strategy_name: String) -> Self {
        Self {
            strategy_name,
            total_signals: 0,
            executed_trades: 0,
            win_rate: 0.0,
            avg_return_pct: 0.0,
            total_pnl: 0.0,
            sharpe_ratio: 0.0,
            max_drawdown: 0.0,
            updated_at: SystemTime::now(),
        }
    }
}

/// 系统健康指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthMetrics {
    /// 启动时间
    pub start_time: SystemTime,

    /// 运行时长（秒）
    pub uptime_secs: u64,

    /// 错误计数
    pub error_count: u64,

    /// 警告计数
    pub warning_count: u64,

    /// RPC调用次数
    pub rpc_calls: u64,

    /// RPC成功次数
    pub rpc_success: u64,

    /// RPC失败次数
    pub rpc_failures: u64,

    /// 平均RPC延迟（毫秒）
    pub avg_rpc_latency_ms: f64,

    /// 缓存命中次数
    pub cache_hits: u64,

    /// 缓存未命中次数
    pub cache_misses: u64,

    /// 缓存命中率
    pub cache_hit_rate: f64,

    /// 活动连接数
    pub active_connections: u64,

    /// 待处理任务数
    pub pending_tasks: u64,

    /// 最后更新时间
    pub updated_at: SystemTime,
}

impl SystemHealthMetrics {
    pub fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
            uptime_secs: 0,
            error_count: 0,
            warning_count: 0,
            rpc_calls: 0,
            rpc_success: 0,
            rpc_failures: 0,
            avg_rpc_latency_ms: 0.0,
            cache_hits: 0,
            cache_misses: 0,
            cache_hit_rate: 0.0,
            active_connections: 0,
            pending_tasks: 0,
            updated_at: SystemTime::now(),
        }
    }

    pub fn update_uptime(&mut self) {
        self.uptime_secs = self.start_time.elapsed().unwrap_or_default().as_secs();
        self.updated_at = SystemTime::now();
    }

    pub fn update_cache_hit_rate(&mut self) {
        let total = self.cache_hits + self.cache_misses;
        self.cache_hit_rate = if total > 0 {
            self.cache_hits as f64 / total as f64
        } else {
            0.0
        };
    }

    pub fn update_rpc_success_rate(&self) -> f64 {
        if self.rpc_calls > 0 {
            self.rpc_success as f64 / self.rpc_calls as f64
        } else {
            0.0
        }
    }
}

/// RPC端点指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcEndpointMetrics {
    pub endpoint_url: String,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub timeout_requests: u64,
    pub avg_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub last_error: Option<String>,
    pub last_error_time: Option<SystemTime>,
    pub is_healthy: bool,
    pub updated_at: SystemTime,
}

impl RpcEndpointMetrics {
    pub fn new(endpoint_url: String) -> Self {
        Self {
            endpoint_url,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            timeout_requests: 0,
            avg_latency_ms: 0.0,
            min_latency_ms: f64::MAX,
            max_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
            last_error: None,
            last_error_time: None,
            is_healthy: true,
            updated_at: SystemTime::now(),
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_requests > 0 {
            self.successful_requests as f64 / self.total_requests as f64
        } else {
            0.0
        }
    }

    pub fn update_health_status(&mut self) {
        // 判断健康状态：成功率 > 90% 且 平均延迟 < 2000ms
        self.is_healthy = self.success_rate() > 0.9 && self.avg_latency_ms < 2000.0;
    }
}

/// 指标收集器
pub struct MetricsCollector {
    /// 交易指标
    trading_metrics: Arc<RwLock<TradingMetrics>>,

    /// 策略指标（按策略名称）
    strategy_metrics: Arc<RwLock<HashMap<String, StrategyMetrics>>>,

    /// 系统健康指标
    system_health: Arc<RwLock<SystemHealthMetrics>>,

    /// RPC端点指标
    rpc_endpoints: Arc<RwLock<HashMap<String, RpcEndpointMetrics>>>,

    /// 自定义指标
    custom_metrics: Arc<RwLock<HashMap<String, Metric>>>,

    /// 延迟记录（用于计算百分位数）
    latency_samples: Arc<RwLock<Vec<f64>>>,

    /// 启用状态
    enabled: bool,
}

impl MetricsCollector {
    /// 创建新的指标收集器
    pub fn new() -> Self {
        Self {
            trading_metrics: Arc::new(RwLock::new(TradingMetrics::default())),
            strategy_metrics: Arc::new(RwLock::new(HashMap::new())),
            system_health: Arc::new(RwLock::new(SystemHealthMetrics::new())),
            rpc_endpoints: Arc::new(RwLock::new(HashMap::new())),
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
            latency_samples: Arc::new(RwLock::new(Vec::new())),
            enabled: true,
        }
    }

    /// 禁用指标收集
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// 启用指标收集
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// 记录交易
    pub async fn record_trade(&self, pnl_sol: f64, pnl_usd: f64, holding_duration_secs: u64, is_win: bool) {
        if !self.enabled {
            return;
        }

        let mut metrics = self.trading_metrics.write().await;
        metrics.total_trades += 1;

        if is_win {
            metrics.successful_trades += 1;
        } else {
            metrics.failed_trades += 1;
        }

        metrics.total_pnl_sol += pnl_sol;
        metrics.total_pnl_usd += pnl_usd;

        // 更新胜率
        metrics.win_rate = metrics.successful_trades as f64 / metrics.total_trades as f64;

        // 更新平均盈亏
        if is_win {
            let total_wins = metrics.successful_trades as f64;
            metrics.avg_win = ((metrics.avg_win * (total_wins - 1.0)) + pnl_sol) / total_wins;
            metrics.max_win = metrics.max_win.max(pnl_sol);
        } else {
            let total_losses = metrics.failed_trades as f64;
            metrics.avg_loss = ((metrics.avg_loss * (total_losses - 1.0)) + pnl_sol) / total_losses;
            metrics.max_loss = metrics.max_loss.min(pnl_sol);
        }

        // 更新盈亏比
        if metrics.avg_loss < 0.0 {
            metrics.profit_factor = metrics.avg_win / metrics.avg_loss.abs();
        }

        // 更新平均持仓时长
        let total = metrics.total_trades as f64;
        metrics.avg_holding_duration_secs =
            ((metrics.avg_holding_duration_secs as f64 * (total - 1.0)) + holding_duration_secs as f64) as u64 / metrics.total_trades;

        metrics.updated_at = SystemTime::now();

        tracing::debug!(
            "📊 Trade recorded: PnL={:.4} SOL ({:.2} USD), Duration={}s, Win={}",
            pnl_sol, pnl_usd, holding_duration_secs, is_win
        );
    }

    /// 记录策略信号
    pub async fn record_strategy_signal(&self, strategy_name: &str, executed: bool) {
        if !self.enabled {
            return;
        }

        let mut metrics = self.strategy_metrics.write().await;
        let strategy = metrics
            .entry(strategy_name.to_string())
            .or_insert_with(|| StrategyMetrics::new(strategy_name.to_string()));

        strategy.total_signals += 1;
        if executed {
            strategy.executed_trades += 1;
        }
        strategy.updated_at = SystemTime::now();
    }

    /// 更新策略性能
    pub async fn update_strategy_performance(
        &self,
        strategy_name: &str,
        win_rate: f64,
        avg_return_pct: f64,
        total_pnl: f64,
    ) {
        if !self.enabled {
            return;
        }

        let mut metrics = self.strategy_metrics.write().await;
        let strategy = metrics
            .entry(strategy_name.to_string())
            .or_insert_with(|| StrategyMetrics::new(strategy_name.to_string()));

        strategy.win_rate = win_rate;
        strategy.avg_return_pct = avg_return_pct;
        strategy.total_pnl = total_pnl;
        strategy.updated_at = SystemTime::now();
    }

    /// 记录RPC调用
    pub async fn record_rpc_call(&self, endpoint: &str, latency_ms: f64, success: bool, error: Option<String>) {
        if !self.enabled {
            return;
        }

        // 更新系统健康指标
        {
            let mut health = self.system_health.write().await;
            health.rpc_calls += 1;
            if success {
                health.rpc_success += 1;
            } else {
                health.rpc_failures += 1;
            }

            // 更新平均延迟
            let total = health.rpc_calls as f64;
            health.avg_rpc_latency_ms =
                ((health.avg_rpc_latency_ms * (total - 1.0)) + latency_ms) / total;
        }

        // 更新端点指标
        {
            let mut endpoints = self.rpc_endpoints.write().await;
            let endpoint_metrics = endpoints
                .entry(endpoint.to_string())
                .or_insert_with(|| RpcEndpointMetrics::new(endpoint.to_string()));

            endpoint_metrics.total_requests += 1;
            if success {
                endpoint_metrics.successful_requests += 1;
            } else {
                endpoint_metrics.failed_requests += 1;
                endpoint_metrics.last_error = error;
                endpoint_metrics.last_error_time = Some(SystemTime::now());
            }

            // 更新延迟统计
            let total = endpoint_metrics.total_requests as f64;
            endpoint_metrics.avg_latency_ms =
                ((endpoint_metrics.avg_latency_ms * (total - 1.0)) + latency_ms) / total;
            endpoint_metrics.min_latency_ms = endpoint_metrics.min_latency_ms.min(latency_ms);
            endpoint_metrics.max_latency_ms = endpoint_metrics.max_latency_ms.max(latency_ms);

            endpoint_metrics.update_health_status();
            endpoint_metrics.updated_at = SystemTime::now();
        }

        // 保存延迟样本用于百分位数计算
        {
            let mut samples = self.latency_samples.write().await;
            samples.push(latency_ms);

            // 限制样本数量
            if samples.len() > 10000 {
                samples.drain(0..5000);
            }
        }
    }

    /// 记录缓存命中
    pub async fn record_cache_hit(&self) {
        if !self.enabled {
            return;
        }

        let mut health = self.system_health.write().await;
        health.cache_hits += 1;
        health.update_cache_hit_rate();
    }

    /// 记录缓存未命中
    pub async fn record_cache_miss(&self) {
        if !self.enabled {
            return;
        }

        let mut health = self.system_health.write().await;
        health.cache_misses += 1;
        health.update_cache_hit_rate();
    }

    /// 记录错误
    pub async fn record_error(&self) {
        if !self.enabled {
            return;
        }

        let mut health = self.system_health.write().await;
        health.error_count += 1;
        health.updated_at = SystemTime::now();
    }

    /// 记录警告
    pub async fn record_warning(&self) {
        if !self.enabled {
            return;
        }

        let mut health = self.system_health.write().await;
        health.warning_count += 1;
        health.updated_at = SystemTime::now();
    }

    /// 更新活动连接数
    pub async fn set_active_connections(&self, count: u64) {
        if !self.enabled {
            return;
        }

        let mut health = self.system_health.write().await;
        health.active_connections = count;
        health.updated_at = SystemTime::now();
    }

    /// 更新待处理任务数
    pub async fn set_pending_tasks(&self, count: u64) {
        if !self.enabled {
            return;
        }

        let mut health = self.system_health.write().await;
        health.pending_tasks = count;
        health.updated_at = SystemTime::now();
    }

    /// 获取交易指标
    pub async fn get_trading_metrics(&self) -> TradingMetrics {
        self.trading_metrics.read().await.clone()
    }

    /// 获取策略指标
    pub async fn get_strategy_metrics(&self, strategy_name: &str) -> Option<StrategyMetrics> {
        self.strategy_metrics.read().await.get(strategy_name).cloned()
    }

    /// 获取所有策略指标
    pub async fn get_all_strategy_metrics(&self) -> HashMap<String, StrategyMetrics> {
        self.strategy_metrics.read().await.clone()
    }

    /// 获取系统健康指标
    pub async fn get_system_health(&self) -> SystemHealthMetrics {
        let mut health = self.system_health.write().await;
        health.update_uptime();
        health.clone()
    }

    /// 获取RPC端点指标
    pub async fn get_rpc_endpoint_metrics(&self, endpoint: &str) -> Option<RpcEndpointMetrics> {
        self.rpc_endpoints.read().await.get(endpoint).cloned()
    }

    /// 获取所有RPC端点指标
    pub async fn get_all_rpc_metrics(&self) -> HashMap<String, RpcEndpointMetrics> {
        self.rpc_endpoints.read().await.clone()
    }

    /// 计算延迟百分位数
    pub async fn calculate_latency_percentile(&self, percentile: f64) -> f64 {
        let samples = self.latency_samples.read().await;
        if samples.is_empty() {
            return 0.0;
        }

        let mut sorted = samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = ((percentile / 100.0) * sorted.len() as f64) as usize;
        sorted.get(index.min(sorted.len() - 1)).copied().unwrap_or(0.0)
    }

    /// 获取摘要报告
    pub async fn get_summary_report(&self) -> MetricsSummary {
        let trading = self.get_trading_metrics().await;
        let health = self.get_system_health().await;
        let strategies = self.get_all_strategy_metrics().await;
        let rpc = self.get_all_rpc_metrics().await;

        MetricsSummary {
            trading,
            health,
            strategies,
            rpc_endpoints: rpc,
            p95_latency_ms: self.calculate_latency_percentile(95.0).await,
            p99_latency_ms: self.calculate_latency_percentile(99.0).await,
            timestamp: SystemTime::now(),
        }
    }

    /// 重置所有指标
    pub async fn reset_all(&self) {
        *self.trading_metrics.write().await = TradingMetrics::default();
        self.strategy_metrics.write().await.clear();
        *self.system_health.write().await = SystemHealthMetrics::new();
        self.rpc_endpoints.write().await.clear();
        self.custom_metrics.write().await.clear();
        self.latency_samples.write().await.clear();

        tracing::info!("🔄 All metrics have been reset");
    }

    /// 导出为JSON
    pub async fn export_json(&self) -> String {
        let summary = self.get_summary_report().await;
        serde_json::to_string_pretty(&summary).unwrap_or_default()
    }
}

/// 指标摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub trading: TradingMetrics,
    pub health: SystemHealthMetrics,
    pub strategies: HashMap<String, StrategyMetrics>,
    pub rpc_endpoints: HashMap<String, RpcEndpointMetrics>,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub timestamp: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_trade() {
        let collector = MetricsCollector::new();

        // 记录盈利交易
        collector.record_trade(5.0, 750.0, 3600, true).await;

        let metrics = collector.get_trading_metrics().await;
        assert_eq!(metrics.total_trades, 1);
        assert_eq!(metrics.successful_trades, 1);
        assert_eq!(metrics.total_pnl_sol, 5.0);
        assert_eq!(metrics.win_rate, 1.0);

        // 记录亏损交易
        collector.record_trade(-2.0, -300.0, 1800, false).await;

        let metrics = collector.get_trading_metrics().await;
        assert_eq!(metrics.total_trades, 2);
        assert_eq!(metrics.failed_trades, 1);
        assert_eq!(metrics.win_rate, 0.5);
    }

    #[tokio::test]
    async fn test_strategy_metrics() {
        let collector = MetricsCollector::new();

        collector.record_strategy_signal("early_bird", true).await;
        collector.record_strategy_signal("early_bird", false).await;

        let metrics = collector.get_strategy_metrics("early_bird").await.unwrap();
        assert_eq!(metrics.total_signals, 2);
        assert_eq!(metrics.executed_trades, 1);
    }

    #[tokio::test]
    async fn test_rpc_metrics() {
        let collector = MetricsCollector::new();

        collector.record_rpc_call("https://api.mainnet-beta.solana.com", 150.0, true, None).await;
        collector.record_rpc_call("https://api.mainnet-beta.solana.com", 200.0, true, None).await;
        collector.record_rpc_call("https://api.mainnet-beta.solana.com", 500.0, false, Some("timeout".to_string())).await;

        let endpoint_metrics = collector
            .get_rpc_endpoint_metrics("https://api.mainnet-beta.solana.com")
            .await
            .unwrap();

        assert_eq!(endpoint_metrics.total_requests, 3);
        assert_eq!(endpoint_metrics.successful_requests, 2);
        assert_eq!(endpoint_metrics.failed_requests, 1);
        assert!((endpoint_metrics.avg_latency_ms - 283.33).abs() < 1.0);
    }

    #[tokio::test]
    async fn test_cache_metrics() {
        let collector = MetricsCollector::new();

        collector.record_cache_hit().await;
        collector.record_cache_hit().await;
        collector.record_cache_miss().await;

        let health = collector.get_system_health().await;
        assert_eq!(health.cache_hits, 2);
        assert_eq!(health.cache_misses, 1);
        assert!((health.cache_hit_rate - 0.666).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_histogram() {
        let mut histogram = HistogramData::new(vec![10.0, 50.0, 100.0, 500.0]);

        histogram.observe(5.0);
        histogram.observe(25.0);
        histogram.observe(75.0);
        histogram.observe(150.0);

        assert_eq!(histogram.count, 4);
        assert_eq!(histogram.sum, 255.0);
        assert!((histogram.mean() - 63.75).abs() < 0.01);

        // 检查分桶
        assert_eq!(histogram.buckets[0].1, 1); // <= 10.0
        assert_eq!(histogram.buckets[1].1, 2); // <= 50.0
        assert_eq!(histogram.buckets[2].1, 3); // <= 100.0
        assert_eq!(histogram.buckets[3].1, 4); // <= 500.0
    }

    #[tokio::test]
    async fn test_latency_percentile() {
        let collector = MetricsCollector::new();

        // 添加一些延迟样本
        for i in 1..=100 {
            collector.latency_samples.write().await.push(i as f64);
        }

        let p50 = collector.calculate_latency_percentile(50.0).await;
        let p95 = collector.calculate_latency_percentile(95.0).await;
        let p99 = collector.calculate_latency_percentile(99.0).await;

        assert!((p50 - 50.0).abs() < 2.0);
        assert!((p95 - 95.0).abs() < 2.0);
        assert!((p99 - 99.0).abs() < 2.0);
    }

    #[tokio::test]
    async fn test_system_health_uptime() {
        let collector = MetricsCollector::new();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let health = collector.get_system_health().await;
        assert!(health.uptime_secs >= 0);
    }

    #[tokio::test]
    async fn test_reset_metrics() {
        let collector = MetricsCollector::new();

        collector.record_trade(5.0, 750.0, 3600, true).await;
        collector.record_error().await;
        collector.record_cache_hit().await;

        collector.reset_all().await;

        let metrics = collector.get_trading_metrics().await;
        assert_eq!(metrics.total_trades, 0);

        let health = collector.get_system_health().await;
        assert_eq!(health.error_count, 0);
        assert_eq!(health.cache_hits, 0);
    }

    #[tokio::test]
    async fn test_export_json() {
        let collector = MetricsCollector::new();

        collector.record_trade(5.0, 750.0, 3600, true).await;

        let json = collector.export_json().await;
        assert!(json.contains("trading"));
        assert!(json.contains("health"));
        assert!(json.contains("total_trades"));
    }
}
