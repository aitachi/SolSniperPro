use solana_sdk::signature::Keypair;

/// 钱包管理器
pub struct WalletManager {
    wallets: Vec<Keypair>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            wallets: Vec::new(),
        }
    }

    pub fn add_wallet(&mut self, keypair: Keypair) {
        self.wallets.push(keypair);
    }

    pub fn get_wallet(&self, index: usize) -> Option<&Keypair> {
        self.wallets.get(index)
    }

    pub fn wallet_count(&self) -> usize {
        self.wallets.len()
    }
}
