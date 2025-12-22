use crate::{Error, Result};
use async_trait::async_trait;
use moka::future::Cache as MokaCache;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// 缓存键
pub type CacheKey = String;

/// 缓存层trait
#[async_trait]
pub trait CacheLayer: Send + Sync {
    /// 获取缓存值
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>>;

    /// 设置缓存值
    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<()>;

    /// 删除缓存
    async fn delete(&self, key: &str) -> Result<()>;

    /// 批量获取
    async fn get_many<T: DeserializeOwned + Send>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>>;

    /// 批量设置
    async fn set_many<T: Serialize + Send + Sync>(
        &self,
        items: &[(&str, &T, Duration)],
    ) -> Result<()>;

    /// 缓存层名称
    fn name(&self) -> &str;
}

/// L1: 内存缓存（使用moka）
pub struct L1MemoryCache {
    cache: MokaCache<String, Vec<u8>>,
    name: String,
}

impl L1MemoryCache {
    /// 创建新的内存缓存
    ///
    /// # 参数
    /// - `max_capacity`: 最大容量（条目数）
    /// - `default_ttl`: 默认TTL
    pub fn new(max_capacity: u64, default_ttl: Duration) -> Self {
        let cache = MokaCache::builder()
            .max_capacity(max_capacity)
            .time_to_live(default_ttl)
            .build();

        Self {
            cache,
            name: "L1-Memory".to_string(),
        }
    }

    /// 创建默认配置（10000条目，30秒TTL）
    pub fn default() -> Self {
        Self::new(10_000, Duration::from_secs(30))
    }
}

#[async_trait]
impl CacheLayer for L1MemoryCache {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>> {
        match self.cache.get(&key.to_string()).await {
            Some(bytes) => {
                let value: T = bincode::deserialize(&bytes)
                    .map_err(|e| Error::Internal(format!("L1 deserialize error: {}", e)))?;
                tracing::trace!("L1 HIT: {}", key);
                Ok(Some(value))
            }
            None => {
                tracing::trace!("L1 MISS: {}", key);
                Ok(None)
            }
        }
    }

    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        _ttl: Duration,
    ) -> Result<()> {
        let bytes = bincode::serialize(value)
            .map_err(|e| Error::Internal(format!("L1 serialize error: {}", e)))?;

        self.cache.insert(key.to_string(), bytes).await;
        tracing::trace!("L1 SET: {}", key);
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        self.cache.invalidate(&key.to_string()).await;
        tracing::trace!("L1 DELETE: {}", key);
        Ok(())
    }

    async fn get_many<T: DeserializeOwned + Send>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>> {
        let mut results = Vec::with_capacity(keys.len());
        for key in keys {
            results.push(self.get(key).await?);
        }
        Ok(results)
    }

    async fn set_many<T: Serialize + Send + Sync>(
        &self,
        items: &[(&str, &T, Duration)],
    ) -> Result<()> {
        for (key, value, ttl) in items {
            self.set(key, value, *ttl).await?;
        }
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// L2: Redis缓存
pub struct L2RedisCache {
    client: Arc<redis::Client>,
    name: String,
}

impl L2RedisCache {
    /// 创建新的Redis缓存
    ///
    /// # 参数
    /// - `redis_url`: Redis连接URL
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| Error::Internal(format!("Redis connection error: {}", e)))?;

        // 测试连接
        let mut conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Error::Internal(format!("Redis connection test failed: {}", e)))?;

        // Ping测试
        redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| Error::Internal(format!("Redis ping failed: {}", e)))?;

        Ok(Self {
            client: Arc::new(client),
            name: "L2-Redis".to_string(),
        })
    }
}

#[async_trait]
impl CacheLayer for L2RedisCache {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Error::Internal(format!("Redis connection error: {}", e)))?;

        let bytes: Option<Vec<u8>> = conn
            .get(key)
            .await
            .map_err(|e| Error::Internal(format!("Redis get error: {}", e)))?;

        match bytes {
            Some(bytes) => {
                let value: T = bincode::deserialize(&bytes)
                    .map_err(|e| Error::Internal(format!("L2 deserialize error: {}", e)))?;
                tracing::trace!("L2 HIT: {}", key);
                Ok(Some(value))
            }
            None => {
                tracing::trace!("L2 MISS: {}", key);
                Ok(None)
            }
        }
    }

    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Error::Internal(format!("Redis connection error: {}", e)))?;

        let bytes = bincode::serialize(value)
            .map_err(|e| Error::Internal(format!("L2 serialize error: {}", e)))?;

        conn.set_ex(key, bytes, ttl.as_secs() as u64)
            .await
            .map_err(|e| Error::Internal(format!("Redis set error: {}", e)))?;

        tracing::trace!("L2 SET: {} (TTL: {}s)", key, ttl.as_secs());
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Error::Internal(format!("Redis connection error: {}", e)))?;

        conn.del(key)
            .await
            .map_err(|e| Error::Internal(format!("Redis delete error: {}", e)))?;

        tracing::trace!("L2 DELETE: {}", key);
        Ok(())
    }

    async fn get_many<T: DeserializeOwned + Send>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Error::Internal(format!("Redis connection error: {}", e)))?;

        let bytes_vec: Vec<Option<Vec<u8>>> = redis::cmd("MGET")
            .arg(keys)
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Internal(format!("Redis mget error: {}", e)))?;

        let mut results = Vec::with_capacity(bytes_vec.len());
        for bytes_opt in bytes_vec {
            match bytes_opt {
                Some(bytes) => {
                    let value: T = bincode::deserialize(&bytes).map_err(|e| {
                        Error::Internal(format!("L2 batch deserialize error: {}", e))
                    })?;
                    results.push(Some(value));
                }
                None => results.push(None),
            }
        }

        Ok(results)
    }

    async fn set_many<T: Serialize + Send + Sync>(
        &self,
        items: &[(&str, &T, Duration)],
    ) -> Result<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| Error::Internal(format!("Redis connection error: {}", e)))?;

        // 使用pipeline批量设置
        let mut pipe = redis::pipe();
        for (key, value, ttl) in items {
            let bytes = bincode::serialize(value)
                .map_err(|e| Error::Internal(format!("L2 batch serialize error: {}", e)))?;
            pipe.set_ex(*key, bytes, ttl.as_secs() as u64);
        }

        pipe.query_async(&mut conn)
            .await
            .map_err(|e| Error::Internal(format!("Redis pipeline error: {}", e)))?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 分层缓存管理器
///
/// 按照L1(内存) -> L2(Redis) -> L3(数据源)的顺序查找
///
/// # 特性
/// - 自动回填：L2命中时自动回填L1
/// - 批量操作支持
/// - 缓存穿透保护
/// - 统计信息收集
pub struct TieredCacheManager {
    l1: Arc<dyn CacheLayer>,
    l2: Arc<dyn CacheLayer>,
    l1_ttl: Duration,
    l2_ttl: Duration,
    stats: Arc<tokio::sync::RwLock<CacheStats>>,
}

/// 缓存统计
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub l1_hits: u64,
    pub l1_misses: u64,
    pub l2_hits: u64,
    pub l2_misses: u64,
    pub total_requests: u64,
}

impl CacheStats {
    pub fn l1_hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.l1_hits as f64 / self.total_requests as f64
    }

    pub fn l2_hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.l2_hits as f64 / self.total_requests as f64
    }

    pub fn overall_hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.l1_hits + self.l2_hits) as f64 / self.total_requests as f64
    }
}

impl TieredCacheManager {
    /// 创建新的分层缓存管理器
    ///
    /// # 参数
    /// - `l1`: L1缓存层（内存）
    /// - `l2`: L2缓存层（Redis）
    /// - `l1_ttl`: L1缓存TTL
    /// - `l2_ttl`: L2缓存TTL
    pub fn new(
        l1: Arc<dyn CacheLayer>,
        l2: Arc<dyn CacheLayer>,
        l1_ttl: Duration,
        l2_ttl: Duration,
    ) -> Self {
        Self {
            l1,
            l2,
            l1_ttl,
            l2_ttl,
            stats: Arc::new(tokio::sync::RwLock::new(CacheStats::default())),
        }
    }

    /// 创建默认配置
    ///
    /// L1: 30秒，L2: 120秒
    pub async fn with_default_config(redis_url: &str) -> Result<Self> {
        let l1 = Arc::new(L1MemoryCache::default());
        let l2 = Arc::new(L2RedisCache::new(redis_url).await?);

        Ok(Self::new(
            l1,
            l2,
            Duration::from_secs(30),
            Duration::from_secs(120),
        ))
    }

    /// 获取缓存值
    ///
    /// 查找顺序：L1 -> L2 -> 返回None
    /// 如果L2命中，自动回填L1
    pub async fn get<T: DeserializeOwned + Serialize + Send + Sync + Clone>(
        &self,
        key: &str,
    ) -> Result<Option<T>> {
        // 更新统计
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }

        // 1. 尝试L1
        if let Some(value) = self.l1.get::<T>(key).await? {
            let mut stats = self.stats.write().await;
            stats.l1_hits += 1;
            return Ok(Some(value));
        }

        // L1 miss
        {
            let mut stats = self.stats.write().await;
            stats.l1_misses += 1;
        }

        // 2. 尝试L2
        if let Some(value) = self.l2.get::<T>(key).await? {
            // L2命中，回填L1
            if let Err(e) = self.l1.set(key, &value, self.l1_ttl).await {
                tracing::warn!("Failed to backfill L1 cache: {}", e);
            }

            let mut stats = self.stats.write().await;
            stats.l2_hits += 1;
            return Ok(Some(value));
        }

        // L2 miss
        {
            let mut stats = self.stats.write().await;
            stats.l2_misses += 1;
        }

        Ok(None)
    }

    /// 设置缓存值
    ///
    /// 同时写入L1和L2
    pub async fn set<T: Serialize + Send + Sync>(&self, key: &str, value: &T) -> Result<()> {
        // 写入L1（忽略错误）
        if let Err(e) = self.l1.set(key, value, self.l1_ttl).await {
            tracing::warn!("Failed to set L1 cache: {}", e);
        }

        // 写入L2
        self.l2.set(key, value, self.l2_ttl).await?;

        Ok(())
    }

    /// 删除缓存
    ///
    /// 同时删除L1和L2
    pub async fn delete(&self, key: &str) -> Result<()> {
        // 删除L1（忽略错误）
        if let Err(e) = self.l1.delete(key).await {
            tracing::warn!("Failed to delete from L1: {}", e);
        }

        // 删除L2
        self.l2.delete(key).await?;

        Ok(())
    }

    /// 批量获取
    pub async fn get_many<T: DeserializeOwned + Serialize + Send + Sync + Clone>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>> {
        let mut results = Vec::with_capacity(keys.len());
        let mut l2_needed_indices = Vec::new();
        let mut l2_needed_keys = Vec::new();

        // 1. 批量查询L1
        let l1_results = self.l1.get_many::<T>(keys).await?;

        for (i, result) in l1_results.into_iter().enumerate() {
            match result {
                Some(value) => {
                    results.push(Some(value));
                }
                None => {
                    results.push(None);
                    l2_needed_indices.push(i);
                    l2_needed_keys.push(keys[i]);
                }
            }
        }

        // 2. L1未命中的，查询L2
        if !l2_needed_keys.is_empty() {
            let l2_results = self.l2.get_many::<T>(&l2_needed_keys).await?;

            for (idx_in_l2, &idx_in_results) in l2_needed_indices.iter().enumerate() {
                if let Some(value) = &l2_results[idx_in_l2] {
                    // L2命中，回填L1
                    if let Err(e) = self.l1.set(keys[idx_in_results], value, self.l1_ttl).await {
                        tracing::warn!("Failed to backfill L1 in batch: {}", e);
                    }
                    results[idx_in_results] = Some(value.clone());
                }
            }
        }

        Ok(results)
    }

    /// 批量设置
    pub async fn set_many<T: Serialize + Send + Sync>(
        &self,
        items: &[(&str, &T)],
    ) -> Result<()> {
        let l1_items: Vec<(&str, &T, Duration)> = items
            .iter()
            .map(|(k, v)| (*k, *v, self.l1_ttl))
            .collect();

        let l2_items: Vec<(&str, &T, Duration)> = items
            .iter()
            .map(|(k, v)| (*k, *v, self.l2_ttl))
            .collect();

        // 写入L1（忽略错误）
        if let Err(e) = self.l1.set_many(&l1_items).await {
            tracing::warn!("Failed to batch set L1: {}", e);
        }

        // 写入L2
        self.l2.set_many(&l2_items).await?;

        Ok(())
    }

    /// 获取或设置（使用提供的函数获取值）
    ///
    /// 如果缓存未命中，调用fetcher获取值并缓存
    pub async fn get_or_fetch<T, F, Fut>(
        &self,
        key: &str,
        fetcher: F,
    ) -> Result<T>
    where
        T: DeserializeOwned + Serialize + Send + Sync + Clone,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // 尝试从缓存获取
        if let Some(value) = self.get(key).await? {
            return Ok(value);
        }

        // 缓存未命中，调用fetcher
        let value = fetcher().await?;

        // 写入缓存（异步，不等待）
        let cache = self.clone_for_async();
        let key = key.to_string();
        let value_clone = value.clone();
        tokio::spawn(async move {
            if let Err(e) = cache.set(&key, &value_clone).await {
                tracing::warn!("Failed to cache fetched value: {}", e);
            }
        });

        Ok(value)
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// 打印统计信息
    pub async fn log_stats(&self) {
        let stats = self.stats.read().await;

        tracing::info!(
            "📊 Cache Stats: L1 hit rate: {:.1}%, L2 hit rate: {:.1}%, Overall: {:.1}% (total: {} requests)",
            stats.l1_hit_rate() * 100.0,
            stats.l2_hit_rate() * 100.0,
            stats.overall_hit_rate() * 100.0,
            stats.total_requests
        );
    }

    /// 重置统计
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = CacheStats::default();
    }

    /// 克隆用于异步操作
    fn clone_for_async(&self) -> Self {
        Self {
            l1: Arc::clone(&self.l1),
            l2: Arc::clone(&self.l2),
            l1_ttl: self.l1_ttl,
            l2_ttl: self.l2_ttl,
            stats: Arc::clone(&self.stats),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_stats() {
        let mut stats = CacheStats::default();
        stats.total_requests = 100;
        stats.l1_hits = 60;
        stats.l1_misses = 40;
        stats.l2_hits = 25;
        stats.l2_misses = 15;

        assert_eq!(stats.l1_hit_rate(), 0.6);
        assert_eq!(stats.l2_hit_rate(), 0.25);
        assert_eq!(stats.overall_hit_rate(), 0.85);
    }

    #[tokio::test]
    async fn test_l1_memory_cache() {
        let cache = L1MemoryCache::new(100, Duration::from_secs(60));

        // Test set and get
        cache
            .set("test_key", &"test_value".to_string(), Duration::from_secs(60))
            .await
            .unwrap();

        let value: Option<String> = cache.get("test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        // Test delete
        cache.delete("test_key").await.unwrap();
        let value: Option<String> = cache.get("test_key").await.unwrap();
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_l1_batch_operations() {
        let cache = L1MemoryCache::new(100, Duration::from_secs(60));

        // Batch set
        let items = vec![
            ("key1", &1u64, Duration::from_secs(60)),
            ("key2", &2u64, Duration::from_secs(60)),
            ("key3", &3u64, Duration::from_secs(60)),
        ];
        cache.set_many(&items).await.unwrap();

        // Batch get
        let results: Vec<Option<u64>> = cache.get_many(&["key1", "key2", "key3"]).await.unwrap();
        assert_eq!(results, vec![Some(1), Some(2), Some(3)]);
    }

    #[test]
    fn test_l1_cache_name() {
        let cache = L1MemoryCache::default();
        assert_eq!(cache.name(), "L1-Memory");
    }
}
