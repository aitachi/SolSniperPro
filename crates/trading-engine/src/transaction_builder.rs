use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    transaction::Transaction,
    message::Message,
    signature::{Keypair, Signer},
    system_instruction,
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
};
use solsniper_core::{Result, TokenInfo};
use std::sync::Arc;

/// 交易构建器
///
/// 负责构建各种Solana交易，包括：
/// - Token Swap交易
/// - 计算单元预算设置
/// - 优先费用设置
pub struct TransactionBuilder {
    /// 最大计算单元
    max_compute_units: u32,

    /// 默认优先费用（micro-lamports per compute unit）
    default_priority_fee: u64,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            max_compute_units: 200_000,
            default_priority_fee: 50_000,
        }
    }

    /// 设置最大计算单元
    pub fn with_compute_units(mut self, units: u32) -> Self {
        self.max_compute_units = units;
        self
    }

    /// 设置优先费用
    pub fn with_priority_fee(mut self, fee: u64) -> Self {
        self.default_priority_fee = fee;
        self
    }

    /// 构建Raydium Swap交易
    ///
    /// # 参数
    /// - `wallet`: 交易签名钱包
    /// - `pool_id`: Raydium池子地址
    /// - `token_in_mint`: 输入代币mint
    /// - `token_out_mint`: 输出代币mint
    /// - `amount_in`: 输入金额（lamports）
    /// - `min_amount_out`: 最小输出金额（用于滑点保护）
    /// - `recent_blockhash`: 最新区块哈希
    pub async fn build_raydium_swap(
        &self,
        wallet: Arc<Keypair>,
        pool_id: &Pubkey,
        token_in_mint: &Pubkey,
        token_out_mint: &Pubkey,
        amount_in: u64,
        min_amount_out: u64,
        recent_blockhash: Hash,
    ) -> Result<Transaction> {
        // Raydium AMM V4 Program ID
        let raydium_program_id = Pubkey::new_from_array([
            103, 75, 80, 88, 57, 77, 72, 84, 106, 83, 50, 122, 116, 49, 113,
            102, 114, 49, 78, 89, 72, 117, 122, 101, 76, 88, 102, 81, 77, 57, 72, 50
        ]); // 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8

        // 构建Swap指令数据
        // Raydium Swap指令布局: [9, amount_in(u64), min_amount_out(u64)]
        let mut instruction_data = vec![9u8]; // Swap指令ID
        instruction_data.extend_from_slice(&amount_in.to_le_bytes());
        instruction_data.extend_from_slice(&min_amount_out.to_le_bytes());

        // 这里简化了账户构建，实际应该根据池子状态查询所有必需账户
        let swap_instruction = Instruction {
            program_id: raydium_program_id,
            accounts: vec![
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new(*pool_id, false),
                // ... 实际需要更多账户，包括AMM authority, open orders, target orders等
                // 这里仅作示例
            ],
            data: instruction_data,
        };

        // 构建完整交易
        self.build_transaction_with_instructions(
            wallet,
            vec![swap_instruction],
            recent_blockhash,
        ).await
    }

    /// 构建Orca Whirlpool Swap交易
    pub async fn build_orca_swap(
        &self,
        wallet: Arc<Keypair>,
        whirlpool: &Pubkey,
        amount_in: u64,
        min_amount_out: u64,
        recent_blockhash: Hash,
    ) -> Result<Transaction> {
        // Orca Whirlpool Program ID
        let orca_program_id = Pubkey::new_from_array([
            119, 104, 105, 114, 76, 98, 77, 105, 105, 99, 86, 100, 105, 111,
            52, 113, 118, 85, 102, 77, 53, 75, 65, 103, 54, 67, 116, 56, 86, 119, 112, 89
        ]); // whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc

        // Orca swap指令构建
        let instruction_data = vec![
            // Orca指令格式
        ];

        let swap_instruction = Instruction {
            program_id: orca_program_id,
            accounts: vec![
                // Orca账户列表
            ],
            data: instruction_data,
        };

        self.build_transaction_with_instructions(
            wallet,
            vec![swap_instruction],
            recent_blockhash,
        ).await
    }

    /// 构建Jupiter聚合Swap交易（推荐用于实际生产）
    ///
    /// Jupiter会自动找到最优路由和最佳价格
    pub async fn build_jupiter_swap(
        &self,
        wallet: Arc<Keypair>,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        amount: u64,
        slippage_bps: u16,
    ) -> Result<Transaction> {
        // 实际实现需要调用Jupiter API
        // GET https://quote-api.jup.ag/v6/quote
        // POST https://quote-api.jup.ag/v6/swap

        tracing::info!(
            "Building Jupiter swap: {} -> {}, amount: {}, slippage: {}bps",
            input_mint, output_mint, amount, slippage_bps
        );

        // TODO: 实际Jupiter API集成
        Err(solsniper_core::Error::Internal("Jupiter integration not yet implemented".to_string()))
    }

    /// 构建简单的SOL转账交易
    pub async fn build_sol_transfer(
        &self,
        from: Arc<Keypair>,
        to: &Pubkey,
        lamports: u64,
        recent_blockhash: Hash,
    ) -> Result<Transaction> {
        let transfer_instruction = system_instruction::transfer(
            &from.pubkey(),
            to,
            lamports,
        );

        self.build_transaction_with_instructions(
            from,
            vec![transfer_instruction],
            recent_blockhash,
        ).await
    }

    /// 使用指令列表构建完整交易
    ///
    /// 自动添加计算预算和优先费用指令
    pub async fn build_transaction_with_instructions(
        &self,
        wallet: Arc<Keypair>,
        mut instructions: Vec<Instruction>,
        recent_blockhash: Hash,
    ) -> Result<Transaction> {
        // 1. 添加计算单元限制
        let compute_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(
            self.max_compute_units
        );

        // 2. 添加优先费用
        let compute_price_ix = ComputeBudgetInstruction::set_compute_unit_price(
            self.default_priority_fee
        );

        // 3. 将预算指令插入到最前面
        instructions.insert(0, compute_price_ix);
        instructions.insert(0, compute_limit_ix);

        // 4. 创建消息
        let message = Message::new_with_blockhash(
            &instructions,
            Some(&wallet.pubkey()),
            &recent_blockhash,
        );

        // 5. 创建并签名交易
        let mut transaction = Transaction::new_unsigned(message);
        transaction.sign(&[&*wallet], recent_blockhash);

        tracing::debug!(
            "Built transaction with {} instructions, signature: {}",
            instructions.len(),
            transaction.signatures[0]
        );

        Ok(transaction)
    }

    /// 计算交易大小（字节）
    pub fn estimate_transaction_size(instruction_count: usize) -> usize {
        // 基础大小 + 每个指令的大小估算
        const SIGNATURE_SIZE: usize = 64;
        const PUBKEY_SIZE: usize = 32;
        const HEADER_SIZE: usize = 3;
        const BLOCKHASH_SIZE: usize = 32;

        let base_size = SIGNATURE_SIZE + HEADER_SIZE + BLOCKHASH_SIZE;
        let instruction_size = instruction_count * 100; // 每个指令约100字节

        base_size + instruction_size
    }

    /// 验证交易大小是否在限制内
    pub fn validate_transaction_size(&self, transaction: &Transaction) -> Result<()> {
        const MAX_TRANSACTION_SIZE: usize = 1232; // Solana交易最大限制

        let serialized = bincode::serialize(transaction)
            .map_err(|e| solsniper_core::Error::Internal(format!("Failed to serialize transaction: {}", e)))?;

        if serialized.len() > MAX_TRANSACTION_SIZE {
            return Err(solsniper_core::Error::Internal(
                format!("Transaction too large: {} bytes (max: {})", serialized.len(), MAX_TRANSACTION_SIZE)
            ));
        }

        Ok(())
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_size_estimation() {
        let size_1_ix = TransactionBuilder::estimate_transaction_size(1);
        let size_5_ix = TransactionBuilder::estimate_transaction_size(5);

        assert!(size_1_ix < size_5_ix);
        assert!(size_5_ix < 1232); // 应该在限制内
    }

    #[tokio::test]
    async fn test_sol_transfer_build() {
        let builder = TransactionBuilder::new();
        let wallet = Arc::new(Keypair::new());
        let to = Pubkey::new_unique();
        let blockhash = Hash::new_unique();

        let tx = builder.build_sol_transfer(
            wallet,
            &to,
            1_000_000,
            blockhash,
        ).await.unwrap();

        // 应该有3个指令：compute_limit + compute_price + transfer
        assert!(tx.message.instructions.len() == 3);
    }
}
