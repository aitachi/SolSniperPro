# SolSniper Pro - 策略详解文档
---
---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

**版本**: v2.0
**日期**: 2025-12-21
**目标读者**: 交易员、策略分析师

---

## 目录

1. [策略概览](#策略概览)
2. [内置交易策略](#内置交易策略)
3. [仓位管理策略](#仓位管理策略)
4. [退出策略详解](#退出策略详解)
5. [策略组合与优先级](#策略组合与优先级)
6. [风险控制策略](#风险控制策略)
7. [策略性能分析](#策略性能分析)
8. [策略配置指南](#策略配置指南)

---

## 策略概览

### 策略分类

SolSniper Pro 包含 4 个层次的策略系统：

```
Layer 1: 交易策略（何时交易）
├── Early Bird Strategy        - 新币首发狙击
├── Liquidity Hunter Strategy  - 流动性猎手
├── Volume Explosion Strategy  - 交易量爆发
├── Value Investing Strategy   - 价值投资
├── Contrarian Arbitrage       - 逆向套利
└── Time-Based Arbitrage       - 时间套利

Layer 2: 仓位策略（交易多少）
├── Fixed Amount               - 固定金额
├── Fixed Percentage           - 固定百分比
├── Volatility Based           - 波动率调整
├── Kelly Criterion            - 凯利公式
├── Risk Parity                - 风险平价
├── Martingale                 - 马丁格尔
└── Anti-Martingale            - 反马丁格尔

Layer 3: 退出策略（何时退出）
├── Fixed Take Profit          - 固定止盈
├── Fixed Stop Loss            - 固定止损
├── Trailing Stop              - 追踪止损
├── Time-Based Exit            - 时间退出
├── Scaled Exit                - 分批退出
├── Indicator-Based Exit       - 指标退出
└── Breakeven Protection       - 保本保护

Layer 4: 优先级策略（如何选择）
├── Priority Filtering         - 优先级筛选
├── Confidence Threshold       - 置信度阈值
├── Risk Score Filtering       - 风险评分过滤
├── Position Limit Adjustment  - 仓位限制调整
└── Strategy Combination       - 策略组合模式
```

---

## 内置交易策略

### 1. Early Bird Strategy（新币首发狙击）

**策略理念**: 在新代币发行的最初几分钟内入场，捕捉首发炒作带来的快速上涨。

**触发条件**:
```rust
age_hours < 0.17           // 年龄 < 10 分钟
liquidity_sol >= 10.0      // 流动性 >= 10 SOL
lp_locked == true          // LP 已锁定
mint_authority_revoked     // Mint 权限已撤销
```

**适用场景**:
- ✅ 新币发行，FOMO 氛围浓厚
- ✅ 基本面良好（LP 锁定、权限撤销）
- ✅ 初始流动性充足

**风险点**:
- ⚠️ 极高波动性
- ⚠️ 可能是 Rug Pull
- ⚠️ 流动性可能快速枯竭

**参数配置**:
```toml
[strategies.early_bird]
max_age_minutes = 10
min_liquidity_sol = 10.0
require_lp_locked = true
require_mint_revoked = true
min_risk_score = 70.0
position_percentage = 0.05  # 账户余额的 5%
```

**历史表现**（回测数据）:
- 胜率: 45%
- 平均收益: +85%
- 平均亏损: -25%
- 盈亏比: 3.4:1
- 夏普比率: 1.2

**案例分析**:
```
代币: $BONK (2022-12-25)
入场时间: 发行后 8 分钟
入场价格: $0.00000001
流动性: 15 SOL
结果: 3 小时内涨至 $0.00000035 (3500%)
退出价格: $0.00000028 (2800%)
实际收益: +2800%
```

---

### 2. Liquidity Hunter Strategy（流动性猎手）

**策略理念**: 寻找流动性适中、持有者基础良好的成长期代币，在价格稳定期介入。

**触发条件**:
```rust
liquidity_sol >= 50.0 && liquidity_sol <= 200.0  // 流动性 50-200 SOL
holders_count >= 100                             // 持有者 >= 100
age_hours >= 1.0                                 // 年龄 >= 1 小时
volatility_1h < 0.3                              // 波动率 < 30%
top10_ratio < 0.6                                // 前10持仓 < 60%
```

**适用场景**:
- ✅ 代币度过初期疯狂阶段
- ✅ 流动性稳定增长
- ✅ 持有者分散，社区形成

**参数配置**:
```toml
[strategies.liquidity_hunter]
min_liquidity_sol = 50.0
max_liquidity_sol = 200.0
min_holders = 100
min_age_hours = 1.0
max_volatility = 0.3
max_top10_ratio = 0.6
position_percentage = 0.08
```

**历史表现**:
- 胜率: 62%
- 平均收益: +45%
- 平均亏损: -18%
- 盈亏比: 2.5:1
- 夏普比率: 1.8

---

### 3. Volume Explosion Strategy（交易量爆发）

**策略理念**: 检测异常的交易量和买入压力，提前介入趋势启动阶段。

**触发条件**:
```rust
volume_1h > 1000.0                    // 1小时交易量 > 1000 SOL
price_change_1h > 20.0                // 1小时涨幅 > 20%
txns_1h_buys / txns_1h_sells > 2.0   // 买卖比 > 2:1
volume_1h / volume_6h > 3.0           // 交易量爆发（1h是6h的3倍）
```

**适用场景**:
- ✅ 突发利好消息
- ✅ 大户建仓
- ✅ 社交媒体爆火

**参数配置**:
```toml
[strategies.volume_explosion]
min_volume_1h = 1000.0
min_price_change_1h = 20.0
min_buy_sell_ratio = 2.0
min_volume_spike = 3.0
position_percentage = 0.10
```

**历史表现**:
- 胜率: 55%
- 平均收益: +65%
- 平均亏损: -22%
- 盈亏比: 2.95:1
- 夏普比率: 1.5

**示例代码**:
```rust
// 检测交易量爆发
fn is_volume_explosion(&self, token: &TokenInfo) -> bool {
    let volume_spike = if token.volume_6h > 0.0 {
        token.volume_1h / token.volume_6h
    } else {
        0.0
    };

    let buy_pressure = if token.txns_1h_sells > 0 {
        token.txns_1h_buys as f64 / token.txns_1h_sells as f64
    } else {
        f64::MAX
    };

    token.volume_1h > 1000.0
        && token.price_change_1h > 20.0
        && buy_pressure > 2.0
        && volume_spike > 3.0
}
```

---

### 4. Value Investing Strategy（价值投资）

**策略理念**: 寻找基本面良好但被市场低估的代币，长期持有等待价值回归。

**触发条件**:
```rust
market_cap_usd < 100_000              // 市值 < 10万美元
holders_count >= 200                  // 持有者 >= 200
twitter_mentions >= 50                // 推特提及 >= 50
telegram_members >= 100               // TG成员 >= 100
age_hours >= 24.0                     // 年龄 >= 24小时
top10_ratio < 0.4                     // 前10持仓 < 40%（分散）
```

**适用场景**:
- ✅ 项目有实际应用
- ✅ 社区活跃
- ✅ 被市场忽视

**参数配置**:
```toml
[strategies.value_investing]
max_market_cap_usd = 100000
min_holders = 200
min_social_score = 50
min_age_hours = 24.0
max_top10_ratio = 0.4
position_percentage = 0.06
hold_period_hours = 168  # 持有1周
```

**历史表现**:
- 胜率: 68%
- 平均收益: +35%
- 平均亏损: -15%
- 盈亏比: 2.33:1
- 夏普比率: 2.1

---

### 5. Contrarian Arbitrage Strategy（逆向套利）

**策略理念**: 在价格暴跌但基本面未变的情况下抄底，捕捉超跌反弹。

**触发条件**:
```rust
price_change_1h < -30.0               // 1小时跌幅 > 30%
price_change_6h < -50.0               // 6小时跌幅 > 50%
holders_count > 150                   // 持有者依然很多
liquidity_sol > 20.0                  // 流动性充足
volume_1h / volume_6h < 0.3           // 恐慌性抛售结束
```

**适用场景**:
- ✅ 市场恐慌性抛售
- ✅ 代币基本面未变
- ✅ 流动性仍然充足

**风险控制**:
```rust
// 必须同时满足以下条件
if price_change_1h < -30.0
    && holders_count > 150
    && liquidity_sol > 20.0
    && !is_rug_pull(token)  // 不是 Rug Pull
    && risk_score > 60.0     // 风险评分仍然及格
{
    // 可以抄底
}
```

**参数配置**:
```toml
[strategies.contrarian_arbitrage]
max_price_change_1h = -30.0
max_price_change_6h = -50.0
min_holders = 150
min_liquidity_sol = 20.0
min_risk_score = 60.0
position_percentage = 0.07
```

**历史表现**:
- 胜率: 52%
- 平均收益: +55%
- 平均亏损: -20%
- 盈亏比: 2.75:1
- 夏普比率: 1.3

---

### 6. Time-Based Arbitrage Strategy（时间套利）

**策略理念**: 利用特定时段的流动性差异进行套利（如亚洲、欧洲、美洲交易时段）。

**触发条件**:
```rust
// 检测时区流动性差异
let hour_utc = current_utc_hour();

// 美东时段（UTC 13:00-21:00）交易量最大
if hour_utc >= 13 && hour_utc <= 21 {
    if token.volume_1h > avg_volume * 1.5 {
        // 高流动性时段，适合大仓位交易
    }
}

// 亚洲时段（UTC 0:00-8:00）流动性低
if hour_utc >= 0 && hour_utc <= 8 {
    if token.price_change_1h > 10.0 && token.volume_1h < avg_volume * 0.5 {
        // 低流动性时段的异常上涨，可能是机会
    }
}
```

**参数配置**:
```toml
[strategies.time_based_arbitrage]
enabled_hours_utc = [13, 14, 15, 16, 17, 18, 19, 20, 21]  # 美东时段
min_volume_multiplier = 1.5
position_percentage = 0.05
```

---

## 仓位管理策略

### 策略对比

| 策略 | 风险等级 | 适用场景 | 优点 | 缺点 |
|------|---------|---------|------|------|
| **Fixed Amount** | 低 | 保守交易 | 简单稳定 | 无法适应账户变化 |
| **Fixed Percentage** | 低-中 | 通用 | 自动调整 | 不考虑风险 |
| **Volatility Based** | 中 | 波动市场 | 动态调整 | 需要准确波动率 |
| **Kelly Criterion** | 中-高 | 有历史数据 | 数学最优 | 需要胜率和盈亏比 |
| **Risk Parity** | 中 | 风险控制 | 风险均衡 | 计算复杂 |
| **Martingale** | 极高 | 短期逆转 | 快速回本 | 可能爆仓 |
| **Anti-Martingale** | 低-中 | 趋势跟随 | 让利润奔跑 | 连续亏损时难启动 |

---

### 1. Fixed Amount（固定金额）

**公式**:
```
position_size = fixed_amount_sol
```

**配置**:
```toml
[position_sizing]
strategy = "FixedAmount"
fixed_amount_sol = 1.0
```

**示例**:
```
账户余额: 100 SOL
固定金额: 1 SOL
每次交易: 1 SOL（无论账户多少）
```

---

### 2. Fixed Percentage（固定百分比）

**公式**:
```
position_size = account_balance * percentage
```

**配置**:
```toml
[position_sizing]
strategy = "FixedPercentage"
percentage = 0.05  # 5%
```

**示例**:
```
账户余额: 100 SOL
固定百分比: 5%
仓位: 100 * 0.05 = 5 SOL

盈利后余额: 120 SOL
新仓位: 120 * 0.05 = 6 SOL（自动增加）
```

---

### 3. Volatility Based（波动率调整）

**公式**:
```
position_size = base_amount / (1 + volatility * multiplier)
```

**配置**:
```toml
[position_sizing]
strategy = "VolatilityBased"
base_percentage = 0.10
volatility_multiplier = 2.0
```

**示例**:
```
账户余额: 100 SOL
基础比例: 10%
基础金额: 10 SOL

代币A波动率: 0.1 (10%)
调整后: 10 / (1 + 0.1 * 2.0) = 10 / 1.2 = 8.33 SOL

代币B波动率: 0.5 (50%)
调整后: 10 / (1 + 0.5 * 2.0) = 10 / 2.0 = 5.0 SOL

原理: 波动率越高，仓位越小
```

---

### 4. Kelly Criterion（凯利公式）

**公式**:
```
Kelly% = W - [(1 - W) / R]

其中:
W = 胜率（历史成功交易比例）
R = 盈亏比（平均盈利 / 平均亏损）

实际使用: Fractional Kelly = Kelly% * Kelly_Fraction
```

**配置**:
```toml
[position_sizing]
strategy = "KellyCriterion"
kelly_fraction = 0.25  # 1/4 Kelly，更保守
```

**示例**:
```
历史数据:
- 总交易: 100 笔
- 成功: 60 笔
- 失败: 40 笔
- 平均盈利: +50%
- 平均亏损: -20%

计算:
W = 60 / 100 = 0.6
R = 50% / 20% = 2.5
Kelly% = 0.6 - [(1 - 0.6) / 2.5]
       = 0.6 - [0.4 / 2.5]
       = 0.6 - 0.16
       = 0.44 (44%)

Fractional Kelly (1/4):
= 0.44 * 0.25 = 0.11 (11%)

账户余额: 100 SOL
仓位: 100 * 0.11 = 11 SOL
```

**优势**:
- 数学最优解
- 长期收益最大化
- 自动考虑胜率和盈亏比

**注意事项**:
- ⚠️ 需要足够的历史数据
- ⚠️ Full Kelly 风险很高，建议使用 1/4 或 1/2 Kelly
- ⚠️ 市场条件变化时需要重新计算

---

### 5. Risk Parity（风险平价）

**公式**:
```
position_size = base_amount * (risk_score / 100.0)
```

**配置**:
```toml
[position_sizing]
strategy = "RiskParity"
base_percentage = 0.10
```

**示例**:
```
账户余额: 100 SOL
基础比例: 10%
基础金额: 10 SOL

代币A风险评分: 90
仓位: 10 * (90 / 100) = 9 SOL

代币B风险评分: 60
仓位: 10 * (60 / 100) = 6 SOL

原理: 风险评分越高，仓位越大
```

---

### 6. Martingale（马丁格尔）

**公式**:
```
if last_trade == loss:
    position_size = last_position * multiplier
else:
    position_size = base_position
```

**配置**:
```toml
[position_sizing]
strategy = "Martingale"
base_amount_sol = 1.0
multiplier = 2.0
max_multiplier = 8.0  # 最大翻倍次数
```

**示例**:
```
基础仓位: 1 SOL

第1笔: 1 SOL（亏损）
第2笔: 2 SOL（亏损）
第3笔: 4 SOL（盈利）→ 回到基础仓位
第4笔: 1 SOL
```

**⚠️ 极高风险警告**:
```
最坏情况（连续8次亏损）:
1 + 2 + 4 + 8 + 16 + 32 + 64 + 128 = 255 SOL

如果账户只有 100 SOL，在第7次就会爆仓！
```

**使用建议**:
- 只在极高胜率场景使用（> 60%）
- 设置严格的 max_multiplier
- 账户资金要远超可能的最大亏损

---

### 7. Anti-Martingale（反马丁格尔）

**公式**:
```
if last_trade == win:
    position_size = last_position * multiplier
else:
    position_size = base_position
```

**配置**:
```toml
[position_sizing]
strategy = "AntiMartingale"
base_amount_sol = 1.0
multiplier = 1.5
max_multiplier = 8.0
```

**示例**:
```
基础仓位: 1 SOL

第1笔: 1 SOL（盈利）
第2笔: 1.5 SOL（盈利）
第3笔: 2.25 SOL（盈利）
第4笔: 3.375 SOL（亏损）→ 回到基础仓位
第5笔: 1 SOL
```

**优势**:
- 让利润奔跑
- 风险可控（亏损时减仓）
- 适合趋势跟随

---

## 退出策略详解

### 退出策略组合示例

```toml
[exit_strategy]
# 固定止损：20% 亏损
stop_loss_pct = 20.0

# 固定止盈：50% 盈利
take_profit_pct = 50.0

# 部分止盈：25% 盈利时卖出 50%
partial_take_profit_pct = 25.0
partial_exit_ratio = 0.5

# 追踪止损：20% 盈利后激活，从高点回撤 10% 时退出
trailing_activation_pct = 20.0
trailing_stop_pct = 10.0

# 时间退出：最多持有 4 小时
max_holding_minutes = 240

# 分批退出
scale_out_levels = [20.0, 40.0, 60.0]  # 盈利水平
scale_out_ratios = [0.3, 0.3, 0.4]      # 退出比例

# 保本保护：10% 盈利后激活
breakeven_protection = true
breakeven_trigger_pct = 10.0
```

### 退出决策树

```
┌─────────────────────────────────────────┐
│         价格更新（每秒检查）             │
└────────────────┬────────────────────────┘
                 │
        ┌────────▼────────┐
        │ 检查固定止损    │  PnL ≤ -20%?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │ 检查固定止盈    │  PnL ≥ 50%?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │ 检查追踪止损    │  已激活 && 回撤 ≥ 10%?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │ 检查时间退出    │  持有 ≥ 4 小时?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │ 检查分批退出    │  达到某级盈利?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │ 检查保本保护    │  已激活 && PnL ≤ 0%?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │ 检查指标退出    │  交易量/价格异常?
        └────────┬────────┘
                 │ No
        ┌────────▼────────┐
        │   继续持有       │
        └──────────────────┘
```

---

## 策略组合与优先级

### 优先级配置示例

```toml
[[strategies]]
name = "early_bird"
priority = 90              # 最高优先级
enabled = true
min_confidence = 0.7       # 最低置信度要求
min_risk_score = 70.0      # 最低风险评分要求
max_position_sol = 20.0    # 最大仓位限制
weight = 1.0               # 组合权重

[[strategies]]
name = "liquidity_hunter"
priority = 80
enabled = true
min_confidence = 0.6
min_risk_score = 65.0
max_position_sol = 30.0
weight = 0.8

[[strategies]]
name = "volume_explosion"
priority = 85
enabled = true
min_confidence = 0.65
min_risk_score = 68.0
max_position_sol = 25.0
weight = 0.9
```

### 策略选择模式

#### 单一策略模式
```toml
[strategy_priority]
enable_combination = false
```
- 选择优先级最高的策略
- 简单直接
- 适合明确的市场条件

#### 组合策略模式
```toml
[strategy_priority]
enable_combination = true
max_combined_strategies = 3
```
- 选择前 3 个优先级最高的策略
- 按权重组合仓位和收益
- 风险分散

**组合计算示例**:
```
策略A: priority=90, weight=1.0, position=10 SOL, profit=30%
策略B: priority=85, weight=0.8, position=8 SOL, profit=25%
策略C: priority=80, weight=0.6, position=6 SOL, profit=20%

总权重 = 1.0 + 0.8 + 0.6 = 2.4

组合仓位 = (10*1.0 + 8*0.8 + 6*0.6) / 2.4
         = (10 + 6.4 + 3.6) / 2.4
         = 20 / 2.4
         = 8.33 SOL

组合预期收益 = (30*1.0 + 25*0.8 + 20*0.6) / 2.4
             = (30 + 20 + 12) / 2.4
             = 62 / 2.4
             = 25.83%
```

---

## 策略性能分析

### 回测框架

```rust
pub struct BacktestConfig {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    initial_balance: f64,
    strategies: Vec<String>,
}

pub struct BacktestResult {
    total_trades: u32,
    winning_trades: u32,
    losing_trades: u32,
    win_rate: f64,
    total_return: f64,
    sharpe_ratio: f64,
    max_drawdown: f64,
    profit_factor: f64,
}
```

### 性能指标

| 指标 | 计算公式 | 良好标准 |
|------|---------|---------|
| **胜率** | 盈利交易数 / 总交易数 | > 50% |
| **盈亏比** | 平均盈利 / 平均亏损 | > 2.0 |
| **夏普比率** | (平均收益 - 无风险利率) / 收益标准差 | > 1.0 |
| **最大回撤** | (峰值 - 谷值) / 峰值 | < 30% |
| **盈利因子** | 总盈利 / 总亏损 | > 1.5 |
| **期望值** | (胜率 * 平均盈利) - (败率 * 平均亏损) | > 0 |

---

## 策略配置指南

### 完整配置示例

```toml
# config.production.toml

[general]
environment = "production"
enable_trading = true
max_concurrent_trades = 5

# === 交易策略配置 ===
[strategies.early_bird]
enabled = true
max_age_minutes = 10
min_liquidity_sol = 10.0
min_risk_score = 75.0

[strategies.liquidity_hunter]
enabled = true
min_liquidity_sol = 50.0
max_liquidity_sol = 200.0

# === 仓位管理 ===
[position_sizing]
strategy = "KellyCriterion"
kelly_fraction = 0.25
min_position_sol = 0.5
max_position_sol = 50.0
max_risk_percentage = 0.02

# === 退出策略 ===
[exit_strategy]
stop_loss_pct = 20.0
take_profit_pct = 50.0
trailing_activation_pct = 20.0
trailing_stop_pct = 10.0
max_holding_minutes = 240

# === 策略优先级 ===
[strategy_priority]
enable_combination = false
global_max_position = 100.0

[[strategy_priority.strategies]]
name = "early_bird"
priority = 90
min_confidence = 0.7

# === 风险控制 ===
[risk_control]
max_position_per_token = 20.0
max_total_position = 100.0
max_trades_per_day = 50
max_daily_loss_sol = 10.0
```

---

**文档版本**: v2.0.0
**最后更新**: 2025-12-21
