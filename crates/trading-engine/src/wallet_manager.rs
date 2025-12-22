use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};
use solana_client::rpc_client::RpcClient;
use solsniper_core::Result;
use std::sync::Arc;
use parking_lot::RwLock;

/// 钱包管理器
pub struct WalletManager {
    /// 主钱包（用于主要交易）
    primary_wallet: Arc<Keypair>,

    /// 子钱包列表（用于并发狙击）
    sub_wallets: Vec<Arc<Keypair>>,

    /// 钱包余额缓存 (Pubkey -> SOL balance)
    balance_cache: Arc<RwLock<std::collections::HashMap<Pubkey, f64>>>,

    /// RPC客户端
    rpc_client: Arc<RpcClient>,
}

impl WalletManager {
    /// 创建新的钱包管理器
    pub fn new(rpc_client: Arc<RpcClient>) -> Self {
        // 生成新的主钱包
        let primary_wallet = Arc::new(Keypair::new());

        Self {
            primary_wallet,
            sub_wallets: Vec::new(),
            balance_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            rpc_client,
        }
    }

    /// 从私钥文件加载主钱包
    pub fn from_keypair_file(path: &str, rpc_client: Arc<RpcClient>) -> Result<Self> {
        let keypair_bytes = std::fs::read(path)
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to read keypair file: {}", e)))?;

        let keypair = if keypair_bytes.len() == 64 {
            // Raw bytes format
            Keypair::from_bytes(&keypair_bytes)
                .map_err(|e| solsniper_core::Error::Internal(format!("Invalid keypair bytes: {}", e)))?
        } else {
            // JSON format
            let keypair_json: Vec<u8> = serde_json::from_slice(&keypair_bytes)
                .map_err(|e| solsniper_core::Error::Internal(format!("Invalid keypair JSON: {}", e)))?;
            Keypair::from_bytes(&keypair_json)
                .map_err(|e| solsniper_core::Error::Internal(format!("Invalid keypair bytes: {}", e)))?
        };

        Ok(Self {
            primary_wallet: Arc::new(keypair),
            sub_wallets: Vec::new(),
            balance_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            rpc_client,
        })
    }

    /// 添加子钱包
    pub fn add_sub_wallet(&mut self, keypair: Keypair) {
        self.sub_wallets.push(Arc::new(keypair));
    }

    /// 生成并添加新的子钱包
    pub fn generate_sub_wallet(&mut self) -> Pubkey {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        self.sub_wallets.push(Arc::new(keypair));
        pubkey
    }

    /// 获取主钱包
    pub fn get_primary_wallet(&self) -> Arc<Keypair> {
        Arc::clone(&self.primary_wallet)
    }

    /// 获取主钱包公钥
    pub fn get_primary_pubkey(&self) -> Pubkey {
        self.primary_wallet.pubkey()
    }

    /// 获取子钱包
    pub fn get_sub_wallet(&self, index: usize) -> Option<Arc<Keypair>> {
        self.sub_wallets.get(index).map(Arc::clone)
    }

    /// 获取子钱包数量
    pub fn sub_wallet_count(&self) -> usize {
        self.sub_wallets.len()
    }

    /// 获取所有钱包公钥
    pub fn get_all_pubkeys(&self) -> Vec<Pubkey> {
        let mut pubkeys = vec![self.primary_wallet.pubkey()];
        pubkeys.extend(self.sub_wallets.iter().map(|w| w.pubkey()));
        pubkeys
    }

    /// 获取钱包SOL余额（带缓存）
    pub async fn get_sol_balance(&self, pubkey: &Pubkey) -> Result<f64> {
        // 检查缓存
        {
            let cache = self.balance_cache.read();
            if let Some(&balance) = cache.get(pubkey) {
                return Ok(balance);
            }
        }

        // 从RPC获取
        let lamports = self.rpc_client
            .get_balance(pubkey)
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to get balance: {}", e)))?;

        let balance = lamports as f64 / 1e9;

        // 更新缓存
        {
            let mut cache = self.balance_cache.write();
            cache.insert(*pubkey, balance);
        }

        Ok(balance)
    }

    /// 获取主钱包余额
    pub async fn get_primary_balance(&self) -> Result<f64> {
        self.get_sol_balance(&self.primary_wallet.pubkey()).await
    }

    /// 刷新余额缓存
    pub fn clear_balance_cache(&self) {
        let mut cache = self.balance_cache.write();
        cache.clear();
    }

    /// 检查钱包是否有足够余额
    pub async fn has_sufficient_balance(&self, pubkey: &Pubkey, required_sol: f64) -> Result<bool> {
        let balance = self.get_sol_balance(pubkey).await?;
        Ok(balance >= required_sol)
    }

    /// 估算交易所需的总金额（包括手续费）
    pub fn estimate_total_cost(amount_sol: f64, priority_fee_lamports: u64) -> f64 {
        const BASE_FEE_LAMPORTS: u64 = 5_000; // 基础交易费用
        let total_fee_lamports = BASE_FEE_LAMPORTS + priority_fee_lamports;
        amount_sol + (total_fee_lamports as f64 / 1e9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_manager_creation() {
        let rpc_client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));
        let manager = WalletManager::new(rpc_client);

        assert_eq!(manager.sub_wallet_count(), 0);
        assert!(manager.get_primary_pubkey().to_bytes().len() == 32);
    }

    #[test]
    fn test_add_sub_wallets() {
        let rpc_client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));
        let mut manager = WalletManager::new(rpc_client);

        manager.generate_sub_wallet();
        manager.generate_sub_wallet();

        assert_eq!(manager.sub_wallet_count(), 2);
    }

    #[test]
    fn test_estimate_total_cost() {
        let amount = 1.0;
        let priority_fee = 50_000;
        let total = WalletManager::estimate_total_cost(amount, priority_fee);

        // 应该是 1.0 + (5000 + 50000) / 1e9 = 1.000055
        assert!((total - 1.000055).abs() < 1e-6);
    }
}
