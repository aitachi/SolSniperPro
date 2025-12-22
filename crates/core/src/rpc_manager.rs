use crate::{Error, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// RPC端点健康状态
#[derive(Debug, Clone)]
pub struct EndpointHealth {
    /// 端点URL
    pub url: String,
    /// 是否健康
    pub is_healthy: bool,
    /// 平均延迟（毫秒）
    pub avg_latency_ms: u64,
    /// 最后检查时间
    pub last_check: Instant,
    /// 连续失败次数
    pub consecutive_failures: u32,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
}

impl EndpointHealth {
    fn new(url: String) -> Self {
        Self {
            url,
            is_healthy: true,
            avg_latency_ms: 0,
            last_check: Instant::now(),
            consecutive_failures: 0,
            total_requests: 0,
            successful_requests: 0,
        }
    }

    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.successful_requests as f64 / self.total_requests as f64
    }

    /// 更新健康状态（成功）
    fn mark_success(&mut self, latency_ms: u64) {
        self.is_healthy = true;
        self.consecutive_failures = 0;
        self.total_requests += 1;
        self.successful_requests += 1;
        self.last_check = Instant::now();

        // 指数移动平均
        if self.avg_latency_ms == 0 {
            self.avg_latency_ms = latency_ms;
        } else {
            self.avg_latency_ms = (self.avg_latency_ms * 7 + latency_ms) / 8;
        }
    }

    /// 更新健康状态（失败）
    fn mark_failure(&mut self) {
        self.consecutive_failures += 1;
        self.total_requests += 1;
        self.last_check = Instant::now();

        // 连续失败3次后标记为不健康
        if self.consecutive_failures >= 3 {
            self.is_healthy = false;
        }
    }
}

/// 负载均衡策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    /// 轮询
    RoundRobin,
    /// 最低延迟
    LowestLatency,
    /// 随机选择
    Random,
}

/// RPC管理器
///
/// 提供多端点负载均衡、健康检查和自动故障转移
///
/// # 功能
/// - 多RPC端点管理
/// - 自动健康检查
/// - 负载均衡（轮询/最低延迟/随机）
/// - 自动故障转移
/// - 连接池管理
/// - 性能统计
pub struct RpcManager {
    /// 端点健康状态
    endpoints: Arc<RwLock<Vec<EndpointHealth>>>,

    /// RPC客户端池
    clients: Arc<RwLock<Vec<Arc<RpcClient>>>>,

    /// 负载均衡策略
    strategy: LoadBalancingStrategy,

    /// 当前轮询索引（用于RoundRobin）
    round_robin_index: Arc<RwLock<usize>>,

    /// 超时时间
    timeout: Duration,

    /// 最大重试次数
    max_retries: u32,

    /// 健康检查间隔
    health_check_interval: Duration,
}

impl RpcManager {
    /// 创建新的RPC管理器
    ///
    /// # 参数
    /// - `endpoints`: RPC端点URL列表
    /// - `timeout_seconds`: 超时时间（秒）
    /// - `strategy`: 负载均衡策略
    pub fn new(
        endpoints: Vec<String>,
        timeout_seconds: u64,
        strategy: LoadBalancingStrategy,
    ) -> Self {
        let timeout = Duration::from_secs(timeout_seconds);

        // 创建健康状态
        let endpoint_health: Vec<EndpointHealth> = endpoints
            .iter()
            .map(|url| EndpointHealth::new(url.clone()))
            .collect();

        // 创建RPC客户端
        let clients: Vec<Arc<RpcClient>> = endpoints
            .iter()
            .map(|url| Arc::new(RpcClient::new_with_timeout(url.clone(), timeout)))
            .collect();

        Self {
            endpoints: Arc::new(RwLock::new(endpoint_health)),
            clients: Arc::new(RwLock::new(clients)),
            strategy,
            round_robin_index: Arc::new(RwLock::new(0)),
            timeout,
            max_retries: 3,
            health_check_interval: Duration::from_secs(30),
        }
    }

    /// 从配置创建
    pub fn from_config(config: &crate::config::RpcConfig, strategy: LoadBalancingStrategy) -> Self {
        Self::new(
            config.endpoints.clone(),
            config.timeout_seconds,
            strategy,
        )
        .with_max_retries(config.max_retries)
    }

    /// 设置最大重试次数
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// 设置健康检查间隔
    pub fn with_health_check_interval(mut self, interval_seconds: u64) -> Self {
        self.health_check_interval = Duration::from_secs(interval_seconds);
        self
    }

    /// 获取可用的RPC客户端
    ///
    /// 根据负载均衡策略选择最佳端点
    pub async fn get_client(&self) -> Result<Arc<RpcClient>> {
        let endpoints = self.endpoints.read().await;
        let clients = self.clients.read().await;

        // 过滤出健康的端点
        let healthy_indices: Vec<usize> = endpoints
            .iter()
            .enumerate()
            .filter(|(_, health)| health.is_healthy)
            .map(|(i, _)| i)
            .collect();

        if healthy_indices.is_empty() {
            // 所有端点都不健康，尝试重新激活
            drop(endpoints);
            drop(clients);
            self.reset_all_endpoints().await;

            return Err(Error::Internal(
                "All RPC endpoints are unhealthy".to_string(),
            ));
        }

        // 根据策略选择端点
        let selected_index = match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                self.select_round_robin(&healthy_indices).await
            }
            LoadBalancingStrategy::LowestLatency => {
                self.select_lowest_latency(&endpoints, &healthy_indices)
            }
            LoadBalancingStrategy::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                healthy_indices[rng.gen_range(0..healthy_indices.len())]
            }
        };

        tracing::debug!(
            "Selected RPC endpoint: {} (strategy={:?})",
            endpoints[selected_index].url,
            self.strategy
        );

        Ok(Arc::clone(&clients[selected_index]))
    }

    /// 轮询选择
    async fn select_round_robin(&self, healthy_indices: &[usize]) -> usize {
        let mut index = self.round_robin_index.write().await;
        let selected = healthy_indices[*index % healthy_indices.len()];
        *index = (*index + 1) % healthy_indices.len();
        selected
    }

    /// 选择最低延迟端点
    fn select_lowest_latency(
        &self,
        endpoints: &[EndpointHealth],
        healthy_indices: &[usize],
    ) -> usize {
        let mut best_index = healthy_indices[0];
        let mut best_latency = endpoints[best_index].avg_latency_ms;

        for &idx in healthy_indices.iter().skip(1) {
            let latency = endpoints[idx].avg_latency_ms;
            if latency < best_latency {
                best_latency = latency;
                best_index = idx;
            }
        }

        best_index
    }

    /// 执行带重试的RPC调用
    ///
    /// 自动处理故障转移和重试
    ///
    /// # 参数
    /// - `operation`: 异步操作函数，接收RpcClient引用
    pub async fn execute_with_retry<F, T, Fut>(&self, operation: F) -> Result<T>
    where
        F: Fn(Arc<RpcClient>) -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;

        for attempt in 1..=self.max_retries {
            // 获取客户端
            let client = match self.get_client().await {
                Ok(c) => c,
                Err(e) => {
                    last_error = Some(e);
                    tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                    continue;
                }
            };

            // 记录开始时间
            let start = Instant::now();

            // 执行操作
            match operation(Arc::clone(&client)).await {
                Ok(result) => {
                    // 成功，更新统计
                    let latency = start.elapsed().as_millis() as u64;
                    self.mark_endpoint_success(&client, latency).await;
                    return Ok(result);
                }
                Err(e) => {
                    // 失败，标记端点
                    self.mark_endpoint_failure(&client).await;
                    last_error = Some(e);

                    tracing::warn!(
                        "RPC operation failed (attempt {}/{}): {}",
                        attempt,
                        self.max_retries,
                        last_error.as_ref().unwrap()
                    );

                    if attempt < self.max_retries {
                        tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            Error::Internal("All retry attempts failed".to_string())
        }))
    }

    /// 标记端点成功
    async fn mark_endpoint_success(&self, client: &Arc<RpcClient>, latency_ms: u64) {
        let url = client.url();
        let mut endpoints = self.endpoints.write().await;

        if let Some(endpoint) = endpoints.iter_mut().find(|e| e.url == url) {
            endpoint.mark_success(latency_ms);
        }
    }

    /// 标记端点失败
    async fn mark_endpoint_failure(&self, client: &Arc<RpcClient>) {
        let url = client.url();
        let mut endpoints = self.endpoints.write().await;

        if let Some(endpoint) = endpoints.iter_mut().find(|e| e.url == url) {
            endpoint.mark_failure();

            if !endpoint.is_healthy {
                tracing::error!(
                    "RPC endpoint marked as unhealthy: {} (consecutive failures: {})",
                    endpoint.url,
                    endpoint.consecutive_failures
                );
            }
        }
    }

    /// 执行健康检查
    ///
    /// 检查所有端点的健康状态
    pub async fn perform_health_check(&self) -> Result<()> {
        let clients = self.clients.read().await;

        for (i, client) in clients.iter().enumerate() {
            let start = Instant::now();

            // 尝试获取最新区块高度作为健康检查
            match tokio::time::timeout(
                self.timeout,
                tokio::task::spawn_blocking({
                    let client = Arc::clone(client);
                    move || client.get_slot()
                }),
            )
            .await
            {
                Ok(Ok(Ok(_slot))) => {
                    // 成功
                    let latency = start.elapsed().as_millis() as u64;
                    let mut endpoints = self.endpoints.write().await;
                    endpoints[i].mark_success(latency);

                    tracing::debug!(
                        "Health check passed: {} (latency: {}ms)",
                        endpoints[i].url,
                        latency
                    );
                }
                _ => {
                    // 失败（超时或错误）
                    let mut endpoints = self.endpoints.write().await;
                    endpoints[i].mark_failure();

                    tracing::warn!(
                        "Health check failed: {} (failures: {})",
                        endpoints[i].url,
                        endpoints[i].consecutive_failures
                    );
                }
            }
        }

        Ok(())
    }

    /// 重置所有端点状态
    ///
    /// 将所有端点标记为健康，用于紧急恢复
    async fn reset_all_endpoints(&self) {
        let mut endpoints = self.endpoints.write().await;
        for endpoint in endpoints.iter_mut() {
            endpoint.is_healthy = true;
            endpoint.consecutive_failures = 0;
        }
        tracing::warn!("All endpoints reset to healthy state (emergency recovery)");
    }

    /// 启动后台健康检查任务
    ///
    /// 定期检查所有端点的健康状态
    pub fn spawn_health_checker(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        let interval = self.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = self.perform_health_check().await {
                    tracing::error!("Health check error: {}", e);
                }

                // 打印统计信息
                self.log_statistics().await;
            }
        })
    }

    /// 获取端点统计信息
    pub async fn get_statistics(&self) -> Vec<EndpointHealth> {
        self.endpoints.read().await.clone()
    }

    /// 打印统计日志
    async fn log_statistics(&self) {
        let endpoints = self.endpoints.read().await;

        let healthy_count = endpoints.iter().filter(|e| e.is_healthy).count();
        let total_count = endpoints.len();

        tracing::info!(
            "📊 RPC Health: {}/{} healthy endpoints",
            healthy_count,
            total_count
        );

        for endpoint in endpoints.iter() {
            if endpoint.total_requests > 0 {
                tracing::debug!(
                    "  - {}: {} (latency: {}ms, success: {:.1}%, requests: {})",
                    endpoint.url,
                    if endpoint.is_healthy {
                        "✅ HEALTHY"
                    } else {
                        "❌ UNHEALTHY"
                    },
                    endpoint.avg_latency_ms,
                    endpoint.success_rate() * 100.0,
                    endpoint.total_requests
                );
            }
        }
    }

    /// 获取所有健康端点的URL
    pub async fn get_healthy_endpoints(&self) -> Vec<String> {
        self.endpoints
            .read()
            .await
            .iter()
            .filter(|e| e.is_healthy)
            .map(|e| e.url.clone())
            .collect()
    }

    /// 获取最佳端点（最低延迟）
    pub async fn get_best_endpoint(&self) -> Option<String> {
        let endpoints = self.endpoints.read().await;

        endpoints
            .iter()
            .filter(|e| e.is_healthy && e.total_requests > 0)
            .min_by_key(|e| e.avg_latency_ms)
            .map(|e| e.url.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_health_creation() {
        let health = EndpointHealth::new("https://api.mainnet-beta.solana.com".to_string());
        assert!(health.is_healthy);
        assert_eq!(health.consecutive_failures, 0);
        assert_eq!(health.total_requests, 0);
        assert_eq!(health.success_rate(), 0.0);
    }

    #[test]
    fn test_endpoint_health_success() {
        let mut health = EndpointHealth::new("https://test.com".to_string());

        health.mark_success(100);
        assert!(health.is_healthy);
        assert_eq!(health.consecutive_failures, 0);
        assert_eq!(health.total_requests, 1);
        assert_eq!(health.successful_requests, 1);
        assert_eq!(health.avg_latency_ms, 100);
        assert_eq!(health.success_rate(), 1.0);
    }

    #[test]
    fn test_endpoint_health_failure() {
        let mut health = EndpointHealth::new("https://test.com".to_string());

        health.mark_failure();
        assert!(health.is_healthy); // Still healthy after 1 failure
        assert_eq!(health.consecutive_failures, 1);

        health.mark_failure();
        assert!(health.is_healthy); // Still healthy after 2 failures

        health.mark_failure();
        assert!(!health.is_healthy); // Unhealthy after 3 failures
        assert_eq!(health.consecutive_failures, 3);
        assert_eq!(health.success_rate(), 0.0);
    }

    #[test]
    fn test_endpoint_health_recovery() {
        let mut health = EndpointHealth::new("https://test.com".to_string());

        // Mark as unhealthy
        health.mark_failure();
        health.mark_failure();
        health.mark_failure();
        assert!(!health.is_healthy);

        // Recovery
        health.mark_success(50);
        assert!(health.is_healthy);
        assert_eq!(health.consecutive_failures, 0);
    }

    #[test]
    fn test_latency_moving_average() {
        let mut health = EndpointHealth::new("https://test.com".to_string());

        health.mark_success(100);
        assert_eq!(health.avg_latency_ms, 100);

        health.mark_success(200);
        // (100 * 7 + 200) / 8 = 112
        assert_eq!(health.avg_latency_ms, 112);

        health.mark_success(160);
        // (112 * 7 + 160) / 8 = 118
        assert_eq!(health.avg_latency_ms, 118);
    }

    #[tokio::test]
    async fn test_rpc_manager_creation() {
        let endpoints = vec![
            "https://api.mainnet-beta.solana.com".to_string(),
            "https://solana-api.projectserum.com".to_string(),
        ];

        let manager = RpcManager::new(endpoints, 30, LoadBalancingStrategy::RoundRobin);
        let stats = manager.get_statistics().await;

        assert_eq!(stats.len(), 2);
        assert!(stats.iter().all(|s| s.is_healthy));
    }

    #[tokio::test]
    async fn test_round_robin_selection() {
        let endpoints = vec![
            "https://endpoint1.com".to_string(),
            "https://endpoint2.com".to_string(),
            "https://endpoint3.com".to_string(),
        ];

        let manager = RpcManager::new(endpoints, 30, LoadBalancingStrategy::RoundRobin);

        // All endpoints are healthy, should cycle through them
        let indices = vec![0, 1, 2];
        for _ in 0..3 {
            let selected = manager.select_round_robin(&indices).await;
            assert!(indices.contains(&selected));
        }
    }

    #[tokio::test]
    async fn test_lowest_latency_selection() {
        let mut endpoints = vec![
            EndpointHealth::new("https://slow.com".to_string()),
            EndpointHealth::new("https://fast.com".to_string()),
            EndpointHealth::new("https://medium.com".to_string()),
        ];

        endpoints[0].avg_latency_ms = 300;
        endpoints[1].avg_latency_ms = 50;
        endpoints[2].avg_latency_ms = 150;

        let manager = RpcManager::new(vec![], 30, LoadBalancingStrategy::LowestLatency);
        let indices = vec![0, 1, 2];

        let selected = manager.select_lowest_latency(&endpoints, &indices);
        assert_eq!(selected, 1); // Should select "fast.com"
    }

    #[tokio::test]
    async fn test_get_healthy_endpoints() {
        let endpoints = vec![
            "https://healthy1.com".to_string(),
            "https://unhealthy.com".to_string(),
            "https://healthy2.com".to_string(),
        ];

        let manager = RpcManager::new(endpoints, 30, LoadBalancingStrategy::RoundRobin);

        // Mark second endpoint as unhealthy
        {
            let mut endpoint_health = manager.endpoints.write().await;
            endpoint_health[1].mark_failure();
            endpoint_health[1].mark_failure();
            endpoint_health[1].mark_failure();
        }

        let healthy = manager.get_healthy_endpoints().await;
        assert_eq!(healthy.len(), 2);
        assert!(healthy.contains(&"https://healthy1.com".to_string()));
        assert!(healthy.contains(&"https://healthy2.com".to_string()));
    }

    #[tokio::test]
    async fn test_get_best_endpoint() {
        let endpoints = vec![
            "https://slow.com".to_string(),
            "https://fast.com".to_string(),
        ];

        let manager = RpcManager::new(endpoints, 30, LoadBalancingStrategy::RoundRobin);

        // Add some statistics
        {
            let mut endpoint_health = manager.endpoints.write().await;
            endpoint_health[0].mark_success(300);
            endpoint_health[1].mark_success(50);
        }

        let best = manager.get_best_endpoint().await;
        assert_eq!(best, Some("https://fast.com".to_string()));
    }
}
