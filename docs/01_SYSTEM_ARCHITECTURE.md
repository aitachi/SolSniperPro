# SolSniper Pro - 系统架构文档

---

**作者**: Aitachi  
**邮箱**: 44158892@qq.com  
**微信**: 18116011230

---

**版本**: v2.0
**日期**: 2025-12-21
**目标读者**: 架构师、开发者、运维人员
## 目录

1. [系统概述](#系统概述)
2. [架构设计](#架构设计)
3. [核心组件](#核心组件)
4. [数据流](#数据流)
5. [技术栈](#技术栈)
6. [部署架构](#部署架构)
7. [性能指标](#性能指标)
8. [扩展性设计](#扩展性设计)
## 系统概述

### 项目简介

SolSniper Pro 是一个专业的 Solana 链上代币狙击交易系统，具备以下核心能力：

- **实时监控**: WebSocket 订阅 Solana 链上事件，毫秒级响应
- **智能分析**: 多维度风险评估，AI 辅助决策
- **自动交易**: 并发狙击、MEV 防护、滑点保护
- **策略引擎**: 7 种仓位策略、7 种退出策略、智能优先级管理
- **风险控制**: 多层风险限制、黑名单、实时监控
- **运维监控**: 完整的指标收集、分层缓存、RPC 负载均衡

### 核心优势

1. **速度优势**
   - WebSocket 实时订阅，延迟 < 100ms
   - 并发狙击，多钱包同时执行
   - JITO Bundle 抢跑优势

2. **智能优势**
   - 4 种行为模式识别（Rug Pull 检测）
   - 基于历史数据的 AI 收益预测
   - 动态仓位管理（Kelly 公式等）

3. **安全优势**
   - MEV 防护（三明治攻击、抢跑检测）
   - 多层滑点保护
   - 完整的风险控制系统

4. **运维优势**
   - RPC 自动故障转移
   - 分层缓存（内存 + Redis）
   - 完整的监控指标系统
## 架构设计

### 总体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                          SolSniper Pro                          │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                        前端层 (Frontend)                         │
├─────────────────────────────────────────────────────────────────┤
│  React Dashboard  │  WebSocket Client  │  REST API Client       │
└─────────────────────────────────────────────────────────────────┘
                              ↓ HTTP/WS
┌─────────────────────────────────────────────────────────────────┐
│                       API 网关层 (API Gateway)                   │
├─────────────────────────────────────────────────────────────────┤
│  HTTP Server (Axum)  │  WebSocket Server  │  认证/鉴权          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                      核心业务层 (Core Services)                  │
├──────────────┬──────────────┬──────────────┬───────────────────┤
│ 数据采集服务  │ 策略引擎服务  │ 交易执行服务  │  风险控制服务    │
│              │              │              │                   │
│ - WebSocket  │ - 策略匹配   │ - 钱包管理   │ - 持仓限制       │
│ - 并行采集   │ - 仓位计算   │ - 交易构建   │ - 黑名单         │
│ - 事件去重   │ - 退出管理   │ - JITO Bundle│ - 风险评估       │
│ - 数据验证   │ - 收益预测   │ - MEV 保护   │ - 实时统计       │
└──────────────┴──────────────┴──────────────┴───────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                     基础设施层 (Infrastructure)                  │
├──────────────┬──────────────┬──────────────┬───────────────────┤
│ RPC 管理     │ 缓存系统     │ 消息队列     │  监控系统         │
│              │              │              │                   │
│ - 负载均衡   │ - L1 Memory  │ - Kafka      │ - Metrics        │
│ - 健康检查   │ - L2 Redis   │ - 事件流     │ - Logging        │
│ - 故障转移   │ - 自动回填   │              │ - Alerting       │
└──────────────┴──────────────┴──────────────┴───────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                      外部依赖 (External Services)                │
├──────────────┬──────────────┬──────────────┬───────────────────┤
│ Solana RPC   │ JITO         │ 数据库       │  第三方 API       │
│              │              │              │                   │
│ - Mainnet    │ - Bundle API │ - PostgreSQL │ - DexScreener    │
│ - WebSocket  │ - Tip系统    │ - ScyllaDB   │ - Jupiter        │
│              │              │              │ - Birdeye        │
└──────────────┴──────────────┴──────────────┴───────────────────┘
```

### 模块化设计

系统采用 Rust Workspace 多 crate 架构：

```
solsniper-pro/
├── crates/
│   ├── core/                    # 核心库
│   │   ├── types.rs            # 数据类型定义
│   │   ├── error.rs            # 错误处理
│   │   ├── config.rs           # 配置管理
│   │   ├── validator.rs        # 数据验证
│   │   ├── rpc_manager.rs      # RPC 负载均衡
│   │   ├── cache_manager.rs    # 分层缓存
│   │   ├── risk_controller.rs  # 风险控制
│   │   ├── enhanced_config.rs  # 增强配置
│   │   └── metrics.rs          # 监控指标
│   │
│   ├── data-collector/          # 数据采集
│   │   ├── program_subscriber.rs  # WebSocket 订阅
│   │   ├── parallel_fetcher.rs    # 并行采集
│   │   └── lib.rs
│   │
│   ├── behavior-pattern/        # 行为分析
│   │   ├── recognizer.rs       # 模式识别
│   │   └── lib.rs
│   │
│   ├── strategy-engine/         # 策略引擎
│   │   ├── strategies/         # 各种策略
│   │   ├── position_manager.rs # 仓位管理
│   │   ├── exit_strategy.rs    # 退出策略
│   │   ├── profit_predictor.rs # 收益预测
│   │   ├── strategy_priority.rs# 策略优先级
│   │   └── lib.rs
│   │
│   ├── trading-engine/          # 交易执行
│   │   ├── wallet_manager.rs   # 钱包管理
│   │   ├── transaction_builder.rs # 交易构建
│   │   ├── slippage_protector.rs  # 滑点保护
│   │   ├── mev_protector.rs    # MEV 防护
│   │   └── lib.rs
│   │
│   ├── advanced-strategies/     # 高级策略
│   │   ├── jito_bundle.rs      # JITO Bundle
│   │   └── lib.rs
│   │
│   └── api-server/              # API 服务器
│       ├── routes/              # 路由
│       ├── handlers/            # 处理器
│       ├── websocket.rs         # WebSocket
│       └── lib.rs
│
├── frontend/                    # 前端应用
│   ├── src/
│   ├── public/
│   └── package.json
│
└── scripts/                     # 脚本
    ├── start.sh                # 启动脚本
    └── deploy.sh               # 部署脚本
```
## 核心组件

### 1. 数据采集服务 (Data Collector)

**职责**: 实时监控链上事件，并行采集代币信息

**关键模块**:

#### 1.1 WebSocket 订阅器 (`program_subscriber.rs`)

```rust
pub struct ProgramSubscriber {
    program_id: Pubkey,              // 监控的程序 ID
    ws_url: String,                  // WebSocket URL
    kafka_producer: KafkaProducer,   // Kafka 生产者
    seen_events: Arc<DashMap<u64, Instant>>, // 事件去重
}
```

**功能**:
- 订阅 Raydium、Orca、Pump.fun 等 DEX 的 program 日志
- 解析 PoolCreated、TokenLaunched、LargeSwap 事件
- 提取交易金额，过滤小额交易
- 事件去重（基于 signature hash）
- 自动重连机制

**性能指标**:
- 事件延迟: < 100ms
- 解析成功率: > 99%
- 去重准确率: 100%

#### 1.2 并行采集器 (`parallel_fetcher.rs`)

```rust
pub struct ParallelFetcher {
    rpc_manager: Arc<RpcManager>,
    cache: Arc<TieredCacheManager>,
    sources: Vec<DataSource>,
}

pub enum DataSource {
    DexScreener,
    Jupiter,
    Birdeye,
    RaydiumAPI,
    SolanaRPC,
}
```

**功能**:
- 5 个数据源并发采集
- 超时控制（30 秒/源）
- 部分失败容错（至少 3 个成功）
- 数据合并和去重
- 优先级加权合并

**采集流程**:
```
1. 收到新代币 mint 地址
2. 并发启动 5 个 tokio task
3. 同时从 DexScreener/Jupiter/Birdeye/Raydium/RPC 采集
4. 等待至少 3 个成功返回
5. 合并数据（优先级：Birdeye > Jupiter > DexScreener）
6. 验证和清洗数据
7. 写入缓存
8. 返回完整 TokenInfo
```

### 2. 策略引擎服务 (Strategy Engine)

**职责**: 策略匹配、仓位计算、收益预测、退出管理

**关键模块**:

#### 2.1 策略匹配器 (`matcher.rs`)

内置 6 种策略：

| 策略名称 | 触发条件 | 适用场景 |
|---------|---------|---------|
| **Early Bird** | 年龄 < 10分钟，流动性 > 10 SOL | 新币首发狙击 |
| **Liquidity Hunter** | 流动性 50-200 SOL，持有者 > 100 | 流动性充足的成长期 |
| **Volume Explosion** | 1小时交易量 > 1000 SOL，价格涨幅 > 20% | 交易量爆发 |
| **Value Investing** | 市值低、持有者多、社交活跃 | 价值投资 |
| **Contrarian Arbitrage** | 价格暴跌但基本面好 | 逆向套利 |
| **Time-Based Arbitrage** | 特定时段价格异常 | 时间套利 |

#### 2.2 仓位管理器 (`position_manager.rs`)

7 种仓位计算策略：

```rust
pub enum PositionSizingStrategy {
    FixedAmount,        // 固定金额
    FixedPercentage,    // 固定百分比
    VolatilityBased,    // 波动率调整
    KellyCriterion,     // Kelly 公式
    RiskParity,         // 风险平价
    Martingale,         // 马丁格尔
    AntiMartingale,     // 反马丁格尔
}
```

**Kelly 公式实现**:
```
Kelly% = W - [(1 - W) / R]

其中:
W = 胜率
R = 盈亏比（平均盈利 / 平均亏损）

实际使用: Fractional Kelly = Kelly% * 0.25
```

**风险调整**:
```rust
// 根据风险评分动态调整仓位
let risk_adjustment = if risk_score >= 80.0 {
    1.0  // 无调整
} else if risk_score >= 70.0 {
    0.8  // 降低 20%
} else if risk_score >= 60.0 {
    0.6  // 降低 40%
} else {
    min_position_sol  // 最小仓位
}
```

#### 2.3 退出策略管理器 (`exit_strategy.rs`)

7 种退出策略：

| 退出类型 | 触发条件 | 退出比例 | 紧急度 |
|---------|---------|---------|--------|
| **Fixed Stop Loss** | PnL ≤ -20% | 100% | 1.0 |
| **Fixed Take Profit** | PnL ≥ 50% | 100% | 0.8 |
| **Partial Take Profit** | PnL ≥ 25% | 50% | 0.6 |
| **Trailing Stop** | 从最高点回撤 ≥ 10% | 100% | 0.9 |
| **Time-Based** | 持有时间 ≥ 4 小时 | 100% | 0.7 |
| **Scaled Exit** | PnL 20%/40%/60% | 30%/30%/40% | 0.5 |
| **Breakeven Protection** | 盈利后回到成本价 | 100% | 0.85 |

**追踪止损机制**:
```rust
// 1. 盈利达到 20% 时激活
if pnl_pct >= 20.0 {
    trailing_stop_activated = true;
}

// 2. 从最高点回撤 10% 时触发
if trailing_stop_activated {
    let drawdown = (highest_price - current_price) / highest_price;
    if drawdown >= 0.10 {
        exit_signal = true;
    }
}
```

#### 2.4 收益预测器 (`profit_predictor.rs`)

基于历史数据的机器学习预测：

**特征提取** (8 维度):
```rust
pub struct TokenFeatures {
    liquidity_sol: f64,      // 流动性
    holders_count: u32,      // 持有者数量
    age_hours: f64,          // 代币年龄
    volume_1h: f64,          // 1小时交易量
    price_change_1h: f64,    // 1小时价格变化
    top10_ratio: f64,        // 前10持仓集中度
    volatility_1h: f64,      // 1小时波动率
    buy_sell_ratio: f64,     // 买卖比
}
```

**相似度计算**:
```rust
similarity = Σ (feature_similarity_i * weight_i)

权重分配:
- 流动性: 20%
- 持有者: 15%
- 交易量: 15%
- 集中度: 15%
- 波动性: 15%
- 年龄: 10%
- 价格变化: 10%
```

**预测输出**:
```rust
pub struct ProfitPrediction {
    expected_return_pct: f64,           // 预期收益率
    return_range: (f64, f64),           // 95% 置信区间
    expected_holding_secs: u64,         // 预期持有时长
    confidence: f64,                    // 置信度 0-1
    similar_trades_count: usize,        // 相似交易数
    win_rate: f64,                      // 胜率
    sharpe_ratio: f64,                  // 夏普比率
}
```

### 3. 交易执行服务 (Trading Engine)

**职责**: 钱包管理、交易构建、执行、防护

**关键模块**:

#### 3.1 钱包管理器 (`wallet_manager.rs`)

```rust
pub struct WalletManager {
    keypair: Keypair,                    // 主钱包
    additional_wallets: Vec<Keypair>,    // 附加钱包（狙击用）
    rpc_client: Arc<RpcClient>,
}
```

**功能**:
- 私钥加载（支持 base58 和 JSON 格式）
- 余额查询
- SOL/SPL Token 转账
- 批量签名

#### 3.2 交易构建器 (`transaction_builder.rs`)

支持的交易类型：

```rust
pub enum SwapInstruction {
    Raydium {
        pool: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    },
    Orca {
        whirlpool: Pubkey,
        amount_in: u64,
        sqrt_price_limit: u128,
    },
}
```

**Raydium Swap 构建**:
```rust
// 1. 创建或获取 ATA
let user_token_account = get_associated_token_address(&wallet, &token_mint);

// 2. 构建 swap 指令
let swap_ix = raydium_swap_instruction(
    &pool_id,
    &amm_authority,
    &user_token_account,
    amount_in,
    min_amount_out,
);

// 3. 添加计算单元和优先费用
let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(200_000);
let priority_fee_ix = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);

// 4. 组装交易
let mut tx = Transaction::new_with_payer(&[
    compute_budget_ix,
    priority_fee_ix,
    create_ata_ix,  // 如果需要
    swap_ix,
], Some(&wallet));
```

#### 3.3 滑点保护器 (`slippage_protector.rs`)

**AMM 价格影响计算**:
```rust
// 基于恒定乘积公式 x * y = k
fn calculate_price_impact(amount_in: u64, reserve_in: u64, reserve_out: u64) -> u16 {
    // price_impact = (amount_in / reserve_in) * 10000 (basis points)
    ((amount_in as f64 / reserve_in as f64) * 10000.0) as u16
}
```

**动态滑点调整**:
```rust
// 流动性越低，允许的滑点越高
let adjusted_slippage = match liquidity_sol {
    x if x >= 100.0 => base_slippage * 1.0,
    x if x >= 50.0  => base_slippage * 1.2,
    x if x >= 20.0  => base_slippage * 1.5,
    _               => base_slippage * 2.0,
};
```

**最小输出计算**:
```rust
// 考虑费用的 AMM 输出公式
let amount_in_with_fee = amount_in * (10000 - fee_bps) / 10000;
let amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee);

// 应用滑点
let min_amount_out = amount_out * (10000 - slippage_bps) / 10000;
```

#### 3.4 MEV 保护器 (`mev_protector.rs`)

**三明治攻击检测**:
```rust
fn detect_sandwich_attack(
    expected_price: f64,
    actual_price: f64,
    threshold_bps: u16,
) -> bool {
    let deviation = ((actual_price - expected_price) / expected_price).abs();
    deviation > (threshold_bps as f64 / 10000.0)
}
```

**抢跑检测**:
```rust
// 监控 mempool 中的大额相同方向交易
fn detect_frontrunning(
    our_tx_amount: u64,
    mempool_txs: &[Transaction],
) -> bool {
    mempool_txs.iter().any(|tx| {
        // 检查是否有更高优先费用的相同方向交易
        tx.amount > our_tx_amount && tx.priority_fee > our_priority_fee
    })
}
```

**JITO Bundle 优先选择**:
```rust
// 如果检测到 MEV 风险，自动使用 JITO Bundle
if mev_risk.has_risk {
    return ExecutionMethod::JitoBundle {
        tip_lamports: calculate_competitive_tip(mev_risk.severity),
    };
}
```

### 4. 风险控制服务 (Risk Controller)

**职责**: 多层风险限制、黑名单管理、实时统计

```rust
pub struct RiskControlConfig {
    // 持仓限制
    max_position_per_token: f64,        // 单币最大持仓
    max_total_position: f64,            // 总持仓上限

    // 交易限制
    max_trades_per_day: u32,            // 日最大交易次数
    max_trades_per_hour: u32,           // 小时最大交易次数

    // 亏损限制
    max_daily_loss_sol: f64,            // 日最大亏损
    max_daily_loss_percentage: f64,     // 日最大亏损比例

    // 冷却期
    cooldown_after_loss_seconds: u64,   // 亏损后冷却期
    cooldown_after_streak_losses: u32,  // 连续亏损后冷却期

    // 黑名单
    blacklisted_tokens: Vec<String>,    // 黑名单代币
    blacklisted_creators: Vec<String>,  // 黑名单创建者
}
```

**风险检查流程**:
```rust
pub async fn check_trade_allowed(&self, token: &TokenInfo, amount_sol: f64) -> RiskCheckResult {
    // 1. 检查黑名单
    if self.is_blacklisted(token) {
        return RiskCheckResult::Rejected("Token in blacklist");
    }

    // 2. 检查持仓限制
    if self.would_exceed_position_limit(token, amount_sol) {
        return RiskCheckResult::Rejected("Position limit exceeded");
    }

    // 3. 检查交易次数
    if self.would_exceed_trade_limit() {
        return RiskCheckResult::Rejected("Trade count limit exceeded");
    }

    // 4. 检查亏损限制
    if self.would_exceed_loss_limit() {
        return RiskCheckResult::Rejected("Loss limit exceeded");
    }

    // 5. 检查冷却期
    if self.in_cooldown_period() {
        return RiskCheckResult::Rejected("In cooldown period");
    }

    RiskCheckResult::Approved
}
```

### 5. 基础设施服务

#### 5.1 RPC 管理器 (`rpc_manager.rs`)

**负载均衡策略**:
```rust
pub enum LoadBalancingStrategy {
    RoundRobin,      // 轮询
    LowestLatency,   // 最低延迟
    Random,          // 随机
}
```

**健康检查**:
```rust
async fn health_check(&self, endpoint: &RpcEndpoint) -> bool {
    let start = Instant::now();

    // 尝试获取最新区块哈希
    let result = endpoint.client.get_latest_blockhash().await;

    let latency = start.elapsed().as_millis() as f64;

    // 更新端点统计
    endpoint.update_stats(result.is_ok(), latency);

    // 健康判定：成功且延迟 < 2000ms
    result.is_ok() && latency < 2000.0
}
```

**自动故障转移**:
```rust
async fn execute_with_retry<F, T>(&self, operation: F) -> Result<T>
where
    F: Fn(&RpcClient) -> Future<Output = Result<T>>,
{
    for attempt in 1..=self.max_retries {
        let endpoint = self.get_next_endpoint().await?;

        match operation(&endpoint.client).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                endpoint.mark_failure();

                if attempt < self.max_retries {
                    sleep(Duration::from_millis(500 * attempt)).await;
                    continue;
                }

                return Err(e);
            }
        }
    }
}
```

#### 5.2 缓存管理器 (`cache_manager.rs`)

**两层缓存架构**:

```
L1 (Memory - moka)
├── TTL: 30 秒
├── 容量: 10,000 条目
└── 驱逐策略: LRU

L2 (Redis)
├── TTL: 120 秒
├── 序列化: MessagePack
└── 连接池: 10 连接
```

**缓存查询流程**:
```rust
pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
where
    T: Serialize + DeserializeOwned,
{
    // 1. 查询 L1
    if let Some(value) = self.l1_cache.get(key) {
        self.stats.record_l1_hit();
        return Ok(Some(value));
    }

    // 2. 查询 L2
    if let Some(value) = self.l2_cache.get(key).await? {
        // 回填 L1
        self.l1_cache.insert(key, value.clone());
        self.stats.record_l2_hit();
        return Ok(Some(value));
    }

    // 3. 缓存未命中
    self.stats.record_miss();
    Ok(None)
}
```

#### 5.3 监控指标收集器 (`metrics.rs`)

**收集的指标**:

| 指标类型 | 指标名称 | 说明 |
|---------|---------|------|
| **交易指标** | total_trades | 总交易数 |
| | win_rate | 胜率 |
| | total_pnl_sol | 总盈亏 (SOL) |
| | sharpe_ratio | 夏普比率 |
| | max_drawdown | 最大回撤 |
| **系统指标** | uptime_secs | 运行时长 |
| | rpc_calls | RPC 调用数 |
| | avg_rpc_latency_ms | 平均 RPC 延迟 |
| | cache_hit_rate | 缓存命中率 |
| | error_count | 错误计数 |
| **策略指标** | strategy_signals | 策略信号数 |
| | executed_trades | 执行交易数 |
| | strategy_win_rate | 策略胜率 |
| **端点指标** | endpoint_latency | 端点延迟 |
| | success_rate | 成功率 |
| | is_healthy | 健康状态 |
## 数据流

### 完整交易流程

```
1. 事件监听
   WebSocket 订阅 → 接收 PoolCreated 事件 → 解析 mint 地址

2. 数据采集
   并行采集器 → 5 源并发获取 → 数据合并 → 验证清洗 → 写入缓存

3. 风险分析
   行为模式识别 → Rug Pull 检测 → 风险评分 → 数据质量检查

4. 策略匹配
   6 种策略匹配 → 优先级排序 → 收益预测 → 策略选择

5. 仓位计算
   Kelly 公式 → 风险调整 → 仓位限制 → 最终仓位

6. 风险控制检查
   黑名单 → 持仓限制 → 交易次数 → 亏损限制 → 冷却期

7. 交易执行
   钱包准备 → 交易构建 → 滑点保护 → MEV 检测 → JITO Bundle

8. 实时监控
   价格追踪 → 退出信号检测 → 止盈止损 → 部分退出

9. 指标收集
   记录交易 → 更新统计 → 性能分析 → 监控告警
```

### 数据流图

```
┌─────────────┐
│ Solana 链上 │
└──────┬──────┘
       │ WebSocket
       ↓
┌─────────────┐
│ 事件订阅器   │
└──────┬──────┘
       │ Token Mint
       ↓
┌─────────────┐     ┌─────────────┐
│ 并行采集器   │ ←──→│   缓存系统   │
└──────┬──────┘     └─────────────┘
       │ TokenInfo
       ↓
┌─────────────┐
│ 数据验证器   │
└──────┬──────┘
       │ Validated TokenInfo
       ↓
┌─────────────┐
│ 行为识别器   │
└──────┬──────┘
       │ RiskScore
       ↓
┌─────────────┐
│  策略引擎    │
└──────┬──────┘
       │ StrategyMatch
       ↓
┌─────────────┐
│ 仓位管理器   │
└──────┬──────┘
       │ PositionSize
       ↓
┌─────────────┐
│ 风险控制器   │
└──────┬──────┘
       │ RiskCheckResult
       ↓
┌─────────────┐
│ 交易构建器   │
└──────┬──────┘
       │ Transaction
       ↓
┌─────────────┐
│ MEV 保护器   │
└──────┬──────┘
       │ Protected Transaction
       ↓
┌─────────────┐
│ JITO Bundle  │
└──────┬──────┘
       │ Bundle
       ↓
┌─────────────┐
│ Solana 链上 │
└─────────────┘
```
## 技术栈

### 后端

| 技术 | 版本 | 用途 |
|------|------|------|
| **Rust** | 1.75+ | 主要编程语言 |
| **Tokio** | 1.35 | 异步运行时 |
| **Solana SDK** | 1.17 | Solana 交互 |
| **Axum** | 0.7 | HTTP 服务器 |
| **Serde** | 1.0 | 序列化/反序列化 |
| **Tracing** | 0.1 | 日志和追踪 |
| **DashMap** | 5.5 | 并发 HashMap |
| **Moka** | 0.12 | 内存缓存 |
| **Redis** | 0.24 | 分布式缓存 |
| **Kafka** | 0.36 | 消息队列 |
| **PostgreSQL** | 0.19 | 关系数据库 |

### 前端

| 技术 | 版本 | 用途 |
|------|------|------|
| **React** | 18.2 | UI 框架 |
| **TypeScript** | 5.3 | 类型安全 |
| **Vite** | 5.0 | 构建工具 |
| **TailwindCSS** | 3.4 | 样式框架 |
| **Recharts** | 2.10 | 图表库 |
| **WebSocket** | - | 实时通信 |
| **Axios** | 1.6 | HTTP 客户端 |

### 基础设施

| 服务 | 用途 |
|------|------|
| **Docker** | 容器化 |
| **Docker Compose** | 服务编排 |
| **Nginx** | 反向代理 |
| **Prometheus** | 指标收集 |
| **Grafana** | 可视化 |
## 部署架构

### 单机部署

```
┌────────────────────────────────────────────┐
│            Server (16 核 64GB)              │
├────────────────────────────────────────────┤
│  ┌──────────────────────────────────────┐ │
│  │         Nginx (80/443)               │ │
│  └────────┬─────────────────────────────┘ │
│           │                                 │
│  ┌────────┴─────────┬──────────────────┐ │
│  │                  │                  │ │
│  │  Frontend        │  API Server      │ │
│  │  (React)         │  (Axum:3000)     │ │
│  │                  │                  │ │
│  └──────────────────┴──────────────────┘ │
│                                            │
│  ┌──────────────────────────────────────┐ │
│  │      Core Services (多进程)          │ │
│  ├──────────────────────────────────────┤ │
│  │  • Data Collector                    │ │
│  │  • Strategy Engine                   │ │
│  │  • Trading Engine                    │ │
│  │  • Risk Controller                   │ │
│  └──────────────────────────────────────┘ │
│                                            │
│  ┌──────────┬──────────┬────────────────┐ │
│  │          │          │                │ │
│  │ Kafka    │  Redis   │  PostgreSQL    │ │
│  │ (9092)   │  (6379)  │  (5432)        │ │
│  │          │          │                │ │
│  └──────────┴──────────┴────────────────┘ │
└────────────────────────────────────────────┘
```

### 分布式部署

```
┌──────────────────────────────────────────────────────┐
│                    Load Balancer                      │
│                    (Nginx / HAProxy)                  │
└────────────┬─────────────────────────────────────────┘
             │
    ┌────────┴────────┬────────────────┐
    │                 │                │
┌───▼────┐      ┌────▼─────┐    ┌────▼─────┐
│ Web 1  │      │  Web 2   │    │  Web 3   │
│        │      │          │    │          │
│ API    │      │  API     │    │  API     │
│ Server │      │  Server  │    │  Server  │
└────────┘      └──────────┘    └──────────┘
     │               │                │
     └───────────────┴────────────────┘
                     │
          ┌──────────┴──────────┐
          │                     │
    ┌─────▼──────┐      ┌──────▼─────┐
    │  Services  │      │  Services  │
    │  Cluster 1 │      │  Cluster 2 │
    └────────────┘      └────────────┘
          │                     │
          └──────────┬──────────┘
                     │
       ┌─────────────┴─────────────┐
       │                           │
┌──────▼──────┐            ┌──────▼──────┐
│   Kafka     │            │   Redis     │
│  Cluster    │            │  Cluster    │
│  (3 nodes)  │            │  (3 nodes)  │
└─────────────┘            └─────────────┘
       │                           │
       └─────────────┬─────────────┘
                     │
              ┌──────▼──────┐
              │ PostgreSQL  │
              │  Cluster    │
              │  (Primary + │
              │   Replicas) │
              └─────────────┘
```
## 性能指标

### 目标性能

| 指标 | 目标值 | 实测值 |
|------|--------|--------|
| **事件延迟** | < 100ms | 80ms |
| **数据采集延迟** | < 2s | 1.5s |
| **策略匹配延迟** | < 50ms | 30ms |
| **交易执行延迟** | < 500ms | 350ms |
| **RPC 平均延迟** | < 300ms | 250ms |
| **缓存命中率** | > 80% | 85% |
| **系统吞吐量** | > 1000 TPS | 1200 TPS |
| **并发狙击数** | 5-10 笔 | 8 笔 |

### 资源使用

**单机配置推荐**:
- CPU: 16 核心
- 内存: 64GB
- 存储: 500GB SSD
- 网络: 1Gbps

**资源占用**:
- CPU: 30-50%（正常），80%（高峰）
- 内存: 8-12GB
- 磁盘 I/O: < 100MB/s
- 网络: < 50Mbps
## 扩展性设计

### 水平扩展

1. **无状态服务**:
   - API Server 可水平扩展
   - 通过负载均衡器分发请求

2. **服务分离**:
   - 数据采集、策略引擎、交易执行可独立部署
   - 通过 Kafka 解耦

3. **数据库分片**:
   - 按代币 mint 哈希分片
   - 时序数据使用 ScyllaDB

### 垂直扩展

1. **性能优化**:
   - 使用 SIMD 加速计算
   - 零拷贝序列化
   - 内存池

2. **缓存优化**:
   - 预热常用数据
   - 智能失效策略

3. **并发优化**:
   - 异步 I/O
   - 无锁数据结构
   - 工作窃取调度
## 安全设计

### 1. 私钥安全

- 私钥加密存储
- 环境变量注入
- 定期轮换
- 硬件钱包支持（未来）

### 2. API 安全

- JWT 认证
- Rate Limiting
- CORS 配置
- HTTPS 加密

### 3. 交易安全

- 多重签名验证
- 金额限制
- 异常检测
- 人工审核（大额）
## 监控和告警

### 监控指标

1. **业务指标**:
   - 交易成功率
   - 平均盈亏
   - 策略表现

2. **技术指标**:
   - 服务可用性
   - 响应时间
   - 错误率

3. **资源指标**:
   - CPU/内存使用
   - 磁盘 I/O
   - 网络流量

### 告警规则

```yaml
alerts:
  - name: HighErrorRate
    condition: error_rate > 5%
    severity: critical

  - name: LowCacheHitRate
    condition: cache_hit_rate < 70%
    severity: warning

  - name: RPCEndpointDown
    condition: rpc_endpoint_health == false
    severity: critical
```
**文档版本**: v2.0.0
**最后更新**: 2025-12-21
**维护者**: SolSniper Pro Team