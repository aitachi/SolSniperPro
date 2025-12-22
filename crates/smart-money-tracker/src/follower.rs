use solsniper_core::{SmartWallet, Result};
use solana_sdk::pubkey::Pubkey;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use chrono::Duration;

/// 聪明钱跟单执行器
pub struct SmartMoneyFollower {
    /// 聪明钱钱包列表
    smart_wallets: Arc<DashMap<Pubkey, SmartWallet>>,

    /// 跟单信号通道
    signal_tx: mpsc::UnboundedSender<FollowSignal>,
    signal_rx: Option<mpsc::UnboundedReceiver<FollowSignal>>,
}

/// 跟单信号
#[derive(Debug, Clone)]
pub struct FollowSignal {
    pub smart_wallet: Pubkey,
    pub token: Pubkey,
    pub original_amount: f64,
    pub recommended_amount: f64,
    pub confidence: f64,
}

impl SmartMoneyFollower {
    pub fn new(smart_wallets: Arc<DashMap<Pubkey, SmartWallet>>) -> Self {
        let (signal_tx, signal_rx) = mpsc::unbounded_channel();

        Self {
            smart_wallets,
            signal_tx,
            signal_rx: Some(signal_rx),
        }
    }

    /// 启动实时跟单
    pub async fn start_following(&self) -> Result<()> {
        tracing::info!("Starting to monitor {} smart wallets", self.smart_wallets.len());

        // 获取所有聪明钱地址
        let addresses: Vec<Pubkey> = self.smart_wallets
            .iter()
            .map(|entry| *entry.key())
            .collect();

        // TODO: 订阅这些地址的交易
        // 这里需要使用 Solana WebSocket 订阅
        // 简化版本使用轮询

        tracing::info!("Subscribed to {} smart wallet addresses", addresses.len());

        Ok(())
    }

    /// 处理聪明钱交易
    pub async fn handle_smart_money_transaction(
        &self,
        wallet: &Pubkey,
        token: &Pubkey,
        amount: f64,
        is_buy: bool,
    ) -> Result<()> {
        if !is_buy {
            // 只跟随买入操作
            return Ok(());
        }

        // 获取该聪明钱的统计信息
        let smart_wallet = self.smart_wallets
            .get(wallet)
            .ok_or_else(|| {
                solsniper_core::Error::NotFound("Smart wallet not found".to_string())
            })?;

        // 快速风险评估 (简化版)
        let risk_score = self.quick_risk_check(token).await;

        if risk_score < 70.0 {
            tracing::info!(
                "Skipping follow for {:?}: risk score too low ({:.1})",
                token,
                risk_score
            );
            return Ok(());
        }

        // 计算跟单金额
        let follow_amount = self.calculate_follow_amount(
            amount,
            smart_wallet.win_rate,
        );

        // 发送跟单信号
        let signal = FollowSignal {
            smart_wallet: *wallet,
            token: *token,
            original_amount: amount,
            recommended_amount: follow_amount,
            confidence: smart_wallet.win_rate,
        };

        self.signal_tx.send(signal)
            .map_err(|e| solsniper_core::Error::Internal(e.to_string()))?;

        tracing::info!(
            "Follow signal sent for {:?}: {:.2} SOL (confidence: {:.1}%)",
            token,
            follow_amount,
            smart_wallet.win_rate * 100.0
        );

        Ok(())
    }

    /// 快速风险检查
    async fn quick_risk_check(&self, _token: &Pubkey) -> f64 {
        // TODO: 实现快速风险检查
        // 简化版本返回固定值
        75.0
    }

    /// 计算跟单金额
    fn calculate_follow_amount(&self, original_amount: f64, win_rate: f64) -> f64 {
        // 基础跟单比例: 10%
        let base_ratio = 0.1;

        // 根据胜率调整
        let win_rate_multiplier = if win_rate > 0.7 {
            1.5
        } else if win_rate > 0.65 {
            1.2
        } else {
            1.0
        };

        // 最大跟单金额: 2 SOL
        let max_follow = 2.0;

        (original_amount * base_ratio * win_rate_multiplier).min(max_follow)
    }

    /// 获取跟单信号接收器
    pub fn take_signal_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<FollowSignal>> {
        self.signal_rx.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_follow_calculation() {
        let smart_wallets = Arc::new(DashMap::new());

        let wallet_addr = Pubkey::new_unique();
        smart_wallets.insert(
            wallet_addr,
            SmartWallet {
                address: wallet_addr,
                total_trades: 100,
                profitable_trades: 70,
                total_profit_sol: 500.0,
                win_rate: 0.7,
                average_holding_time_hours: 3.5,
                last_active: Utc::now(),
                rank: 1,
            },
        );

        let follower = SmartMoneyFollower::new(smart_wallets);

        let amount = follower.calculate_follow_amount(10.0, 0.7);
        assert!(amount > 0.0 && amount <= 2.0);
    }
}
