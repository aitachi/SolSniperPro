use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 配置环境
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Environment::Development,
            "staging" | "stage" => Environment::Staging,
            "production" | "prod" => Environment::Production,
            _ => Environment::Development,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Environment::Development => "development",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

/// 增强配置管理器
///
/// 支持以下功能:
/// - 多环境配置
/// - 环境变量覆盖
/// - 配置热重载
/// - 配置验证
/// - 配置版本控制
pub struct EnhancedConfigManager<T> {
    /// 当前配置
    config: Arc<RwLock<T>>,

    /// 配置文件路径
    config_path: String,

    /// 当前环境
    environment: Environment,

    /// 配置版本
    version: Arc<RwLock<String>>,

    /// 是否启用热重载
    hot_reload_enabled: bool,
}

impl<T> EnhancedConfigManager<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + ConfigValidator,
{
    /// 创建新的配置管理器
    pub async fn new(config_path: &str) -> Result<Self> {
        // 从环境变量获取环境
        let env_str = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let environment = Environment::from_str(&env_str);

        // 构建环境特定的配置文件路径
        let env_config_path = Self::get_env_config_path(config_path, environment);

        // 加载配置
        let config = Self::load_config_file(&env_config_path).await?;

        // 应用环境变量覆盖
        let config = Self::apply_env_overrides(config);

        // 验证配置
        config.validate()?;

        // 获取配置版本
        let version = Self::calculate_config_version(&config);

        tracing::info!(
            "📝 Config loaded: env={}, version={}, path={}",
            environment.as_str(),
            version,
            env_config_path
        );

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path: env_config_path,
            environment,
            version: Arc::new(RwLock::new(version)),
            hot_reload_enabled: false,
        })
    }

    /// 启用热重载
    pub fn enable_hot_reload(mut self) -> Self {
        self.hot_reload_enabled = true;
        self
    }

    /// 获取当前配置
    pub async fn get(&self) -> T {
        self.config.read().await.clone()
    }

    /// 更新配置
    pub async fn update(&self, new_config: T) -> Result<()> {
        // 验证新配置
        new_config.validate()?;

        // 更新配置
        *self.config.write().await = new_config.clone();

        // 更新版本
        let new_version = Self::calculate_config_version(&new_config);
        *self.version.write().await = new_version.clone();

        tracing::info!("⚙️ Config updated: version={}", new_version);

        Ok(())
    }

    /// 重新加载配置
    pub async fn reload(&self) -> Result<()> {
        tracing::info!("🔄 Reloading config from: {}", self.config_path);

        // 加载配置文件
        let config = Self::load_config_file(&self.config_path).await?;

        // 应用环境变量覆盖
        let config = Self::apply_env_overrides(config);

        // 验证配置
        config.validate()?;

        // 更新配置
        self.update(config).await?;

        Ok(())
    }

    /// 保存配置到文件
    pub async fn save(&self) -> Result<()> {
        let config = self.config.read().await;
        Self::save_config_file(&self.config_path, &*config).await?;

        tracing::info!("💾 Config saved to: {}", self.config_path);

        Ok(())
    }

    /// 获取配置版本
    pub async fn get_version(&self) -> String {
        self.version.read().await.clone()
    }

    /// 获取当前环境
    pub fn get_environment(&self) -> Environment {
        self.environment
    }

    /// 启动热重载监听器
    pub fn spawn_hot_reload_watcher(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                if self.hot_reload_enabled {
                    if let Err(e) = self.reload().await {
                        tracing::error!("Failed to reload config: {}", e);
                    }
                }
            }
        })
    }

    /// 加载配置文件
    async fn load_config_file(path: &str) -> Result<T> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| Error::Internal(format!("Failed to read config file: {}", e)))?;

        let config: T = toml::from_str(&content)
            .map_err(|e| Error::Internal(format!("Failed to parse config: {}", e)))?;

        Ok(config)
    }

    /// 保存配置文件
    async fn save_config_file(path: &str, config: &T) -> Result<()> {
        let content = toml::to_string_pretty(config)
            .map_err(|e| Error::Internal(format!("Failed to serialize config: {}", e)))?;

        tokio::fs::write(path, content)
            .await
            .map_err(|e| Error::Internal(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    /// 获取环境特定的配置文件路径
    fn get_env_config_path(base_path: &str, env: Environment) -> String {
        let path = Path::new(base_path);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let ext = path.extension().unwrap().to_str().unwrap();
        let dir = path.parent().unwrap_or_else(|| Path::new(""));

        // 例如: config.toml -> config.development.toml
        let env_filename = format!("{}.{}.{}", stem, env.as_str(), ext);

        dir.join(env_filename).to_str().unwrap().to_string()
    }

    /// 应用环境变量覆盖
    ///
    /// 通过环境变量覆盖配置值
    /// 例如: APP_RPC_ENDPOINTS="https://rpc1.com,https://rpc2.com"
    fn apply_env_overrides(config: T) -> T {
        // 注意: 这里简化实现，实际需要根据具体配置结构实现
        // 可以使用 serde 的 deserialize_with 或自定义 trait

        // 示例: 读取环境变量并覆盖
        // if let Ok(endpoints) = std::env::var("APP_RPC_ENDPOINTS") {
        //     // Override RPC endpoints
        // }

        config
    }

    /// 计算配置版本（基于内容哈希）
    fn calculate_config_version(config: &T) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let serialized = toml::to_string(config).unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        serialized.hash(&mut hasher);
        let hash = hasher.finish();

        format!("v{:x}", hash)
    }
}

/// 配置验证 trait
///
/// 所有配置都应实现此 trait 以提供验证逻辑
pub trait ConfigValidator {
    /// 验证配置
    fn validate(&self) -> Result<()>;
}

/// 配置变更通知
#[derive(Debug, Clone)]
pub struct ConfigChange<T> {
    pub old_version: String,
    pub new_version: String,
    pub old_config: T,
    pub new_config: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        value: String,
        number: i32,
    }

    impl ConfigValidator for TestConfig {
        fn validate(&self) -> Result<()> {
            if self.number < 0 {
                return Err(Error::Internal("Number must be non-negative".to_string()));
            }
            Ok(())
        }
    }

    #[test]
    fn test_environment_from_str() {
        assert_eq!(Environment::from_str("development"), Environment::Development);
        assert_eq!(Environment::from_str("dev"), Environment::Development);
        assert_eq!(Environment::from_str("production"), Environment::Production);
        assert_eq!(Environment::from_str("prod"), Environment::Production);
        assert_eq!(Environment::from_str("staging"), Environment::Staging);
        assert_eq!(Environment::from_str("unknown"), Environment::Development);
    }

    #[test]
    fn test_environment_as_str() {
        assert_eq!(Environment::Development.as_str(), "development");
        assert_eq!(Environment::Staging.as_str(), "staging");
        assert_eq!(Environment::Production.as_str(), "production");
    }

    #[test]
    fn test_config_validation() {
        let valid_config = TestConfig {
            value: "test".to_string(),
            number: 5,
        };
        assert!(valid_config.validate().is_ok());

        let invalid_config = TestConfig {
            value: "test".to_string(),
            number: -1,
        };
        assert!(invalid_config.validate().is_err());
    }

    #[tokio::test]
    async fn test_config_version_calculation() {
        let config1 = TestConfig {
            value: "test".to_string(),
            number: 5,
        };

        let config2 = TestConfig {
            value: "test".to_string(),
            number: 10,
        };

        let version1 = <EnhancedConfigManager<TestConfig>>::calculate_config_version(&config1);
        let version2 = <EnhancedConfigManager<TestConfig>>::calculate_config_version(&config2);

        // Different configs should have different versions
        assert_ne!(version1, version2);

        // Same config should have same version
        let version1_again =
            <EnhancedConfigManager<TestConfig>>::calculate_config_version(&config1);
        assert_eq!(version1, version1_again);
    }

    #[test]
    fn test_env_config_path() {
        let base_path = "config.toml";

        let dev_path = <EnhancedConfigManager<TestConfig>>::get_env_config_path(
            base_path,
            Environment::Development,
        );
        assert!(dev_path.contains("development"));

        let prod_path = <EnhancedConfigManager<TestConfig>>::get_env_config_path(
            base_path,
            Environment::Production,
        );
        assert!(prod_path.contains("production"));
    }
}
