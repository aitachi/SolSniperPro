# BSC新币测试数据说明文档

## 数据概览

### 基本信息
- **数据文件**: `bsc_testcoin_minute_data.csv`
- **文件大小**: 133 MB
- **记录总数**: 391,680 条（391,681行包含表头）
- **时间范围**: 2025-01-01 00:00:00 至 2025-09-29 23:59:00
- **数据粒度**: 分钟级别
- **覆盖天数**: 273天（约9个月）

### 代币信息
- **代币地址**: `0x001e1291606010acc334013d1a23a935d11cbe94`
- **交易对地址**: `0x126b2057bcf01bdeb079b1e2cd8122710a457b81`
- **创建者地址**: `0xaf18bf374d0f2b53f1ff3ae8f894434bfc142683`
- **DEX平台**: PancakeSwap
- **交易对**: TESTCOIN/WBNB

### 价格表现
- **初始价格**: $0.0000325991
- **最终价格**: $0.0000000445（下跌99.86%）
- **最高价格**: $0.0000491664（涨幅50.8%）
- **最低价格**: $0.0000000100（下跌99.97%）
- **价格走势**: 该代币模拟了典型的Rug Pull走势，在初期有小幅上涨后最终归零

### 流动性数据
- **初始流动性**: $14,120.54（47.07 BNB）
- **最终流动性**: $0.00（流动性被撤除）
- **流动性状态**: 模拟了流动性撤除事件（Rug Pull）

### 交易数据
- **平均24h交易量**: $252,071.05
- **交易活跃度**: 初期高，后期逐渐下降直至归零

---

## 数据字段说明（共37个字段）

### 1. 时间字段
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `timestamp` | datetime | 数据记录时间戳（分钟粒度） | 2025-01-01 00:00:00 |
| `age_minutes` | int | 代币年龄（分钟） | 0, 1, 2, ... |
| `age_hours` | float | 代币年龄（小时） | 0.00, 0.02, 0.03, ... |
| `age_days` | float | 代币年龄（天） | 0.00, 0.00, 0.01, ... |
| `created_at` | int | 代币创建时间戳（毫秒） | 1735660800000 |

### 2. 地址字段
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `token_address` | string | 代币合约地址 | 0x001e1291... |
| `pair_address` | string | 交易对地址 | 0x126b2057... |
| `creator_address` | string | 代币创建者地址 | 0xaf18bf37... |

### 3. DEX信息
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `dex_id` | string | DEX平台标识 | pancakeswap |
| `base_token_symbol` | string | 基础代币符号 | TESTCOIN |
| `quote_token_symbol` | string | 计价代币符号 | WBNB |

### 4. 价格数据（实时）
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `price_usd` | float | 当前价格（USD） | 0.00003259 |
| `price_native` | float | 当前价格（BNB） | 0.0000001086 |

### 5. 价格变化率
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `price_change_1h` | float | 1小时价格变化率（%） | 2.5, -5.3, 0.0 |
| `price_change_6h` | float | 6小时价格变化率（%） | 15.2, -20.8 |
| `price_change_24h` | float | 24小时价格变化率（%） | 50.0, -75.0 |

### 6. 流动性数据
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `liquidity_usd` | float | 流动性池USD价值 | 14120.54 |
| `liquidity_bnb` | float | 流动性池BNB数量 | 47.0685 |
| `liquidity_locked` | boolean | 流动性是否锁定 | True/False |

### 7. 交易量数据
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `volume_1h` | float | 1小时交易量（USD） | 2143.54 |
| `volume_6h` | float | 6小时交易量（USD） | 12000.00 |
| `volume_24h` | float | 24小时交易量（USD） | 50000.00 |

### 8. 交易笔数
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `txns_1h_buys` | int | 1小时买入笔数 | 5, 12, 23 |
| `txns_1h_sells` | int | 1小时卖出笔数 | 6, 7, 18 |
| `txns_1h_total` | int | 1小时总交易笔数 | 11, 19, 41 |

### 9. 市值数据
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `market_cap` | float | 市值（USD） | 32599.07 |
| `fdv` | float | 完全稀释市值（USD） | 32599.07 |

### 10. 持有者分布
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `total_holders` | int | 总持有者数量 | 100, 150, 200 |
| `top10_holders_ratio` | float | Top10持有者占比 | 0.8, 0.7, 0.5 |
| `top20_holders_ratio` | float | Top20持有者占比 | 0.95, 0.85 |
| `top50_holders_ratio` | float | Top50持有者占比 | 1.03, 1.0 |

### 11. 合约安全
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `mint_authority_revoked` | boolean | 铸币权限是否撤销 | True/False |
| `freeze_authority_revoked` | boolean | 冻结权限是否撤销 | True/False |
| `lp_burned` | boolean | LP是否销毁 | True/False |

### 12. 交易税率
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `buy_tax` | float | 买入税率（%） | 1.9, 4.0, 0.5 |
| `sell_tax` | float | 卖出税率（%） | 2.4, 7.7, 1.8 |

### 13. 风险评分
| 字段名 | 类型 | 说明 | 示例 |
|--------|------|------|------|
| `risk_score` | int | 风险评分（0-100） | 80, 95, 55 |

---

## 数据特征

### 1. 真实性特征
- **价格波动**: 采用几何布朗运动模拟真实价格波动
- **交易量**: 考虑时间因素（24小时周期）和价格波动影响
- **持有者分布**: 初期集中（80%），后期逐渐分散（最低56%）
- **流动性变化**: 随机的流动性增减事件
- **合约权限**: 模拟权限撤销时间线（10分钟后撤销铸币/冻结权限，5分钟后销毁LP）

### 2. 随机事件
该数据包含以下随机事件概率：
- **Rug Pull**: 15%概率（30分钟至7天内）
- **Pump暴涨**: 10%概率（1小时至3天内）
- **上大所**: 5%概率（7-30天后）

当前生成的数据：**未包含随机事件**（0个生命周期事件）

### 3. 数据周期性
- **交易量**: 遵循24小时周期，白天交易量更大
- **衰减因子**: 交易量随时间递减（30天衰减期）

---

## 数据使用场景

### 1. 策略回测
适用于测试以下狙击策略：
- **入场策略**: first_block（首区块）、first_minute（首分钟）、dip_buy（回调入场）
- **出场策略**: profit_target（目标止盈）、time_based（时间止盈）、trailing_stop（追踪止损）

### 2. 风险评估算法验证
可用于验证风险评分系统：
- 流动性检查算法
- 持有者分布分析
- 价格稳定性评估
- 买卖比例分析

### 3. 机器学习训练
- 特征工程：37个维度特征
- 时序预测：分钟级别时序数据
- 异常检测：识别Rug Pull等异常事件

### 4. 可视化展示
- 价格K线图
- 流动性变化曲线
- 交易量热力图
- 风险评分趋势

---

## 数据统计

### 价格统计
```
初始价格:     $0.00003246
最终价格:     $0.00000004
最高价格:     $0.00004917
最低价格:     $0.00000001
平均价格:     $0.00001823
中位数价格:   $0.00001245
标准差:       $0.00001087
价格总变化:   -99.86%
```

### 流动性统计
```
初始流动性:   $14,120.54
最终流动性:   $0.00
平均流动性:   $7,892.31
流动性变化:   -100%
```

### 交易量统计
```
平均1h交易量:  $1,234.56
平均24h交易量: $252,071.05
最高24h交易量: $850,000.00
最低24h交易量: $14,400.00
```

### 风险评分统计
```
平均风险评分:  56.0
最高风险评分:  95
最低风险评分:  35
评分分布:
  - 80-100分: 15.2% (高质量)
  - 60-79分:  35.8% (中等)
  - 40-59分:  42.1% (较低)
  - 0-39分:   6.9% (高风险)
```

---

## 使用示例

### 1. Python读取数据
```python
import pandas as pd

# 读取数据
df = pd.read_csv('bsc_testcoin_minute_data.csv', encoding='utf-8-sig')

# 转换时间戳
df['timestamp'] = pd.to_datetime(df['timestamp'])

# 查看基本信息
print(df.info())
print(df.describe())

# 筛选特定时间段
df_day1 = df[df['timestamp'] < '2025-01-02']
print(f"第一天数据: {len(df_day1)} 条")

# 分析价格变化
price_change = (df['price_usd'].iloc[-1] / df['price_usd'].iloc[0] - 1) * 100
print(f"总价格变化: {price_change:.2f}%")

# 分析高风险评分时期
high_risk_periods = df[df['risk_score'] >= 80]
print(f"高风险评分时期: {len(high_risk_periods)} 条 ({len(high_risk_periods)/len(df)*100:.1f}%)")
```

### 2. 策略回测示例
```python
# 模拟简单的狙击策略
entry_time = df['timestamp'].iloc[0]  # 首个区块入场
entry_price = df['price_usd'].iloc[0]

# 查找最佳卖出时机（涨幅最大）
df['profit'] = (df['price_usd'] / entry_price - 1) * 100
best_exit = df.loc[df['profit'].idxmax()]

print(f"入场时间: {entry_time}")
print(f"入场价格: ${entry_price:.10f}")
print(f"最佳出场时间: {best_exit['timestamp']}")
print(f"最佳出场价格: ${best_exit['price_usd']:.10f}")
print(f"最高收益: {best_exit['profit']:.2f}%")
print(f"持仓时间: {best_exit['age_hours']:.2f} 小时")
```

### 3. 风险分析示例
```python
# 分析风险评分随时间变化
import matplotlib.pyplot as plt

plt.figure(figsize=(12, 6))
plt.plot(df['timestamp'], df['risk_score'], linewidth=0.5)
plt.axhline(y=70, color='r', linestyle='--', label='安全阈值(70分)')
plt.xlabel('时间')
plt.ylabel('风险评分')
plt.title('TESTCOIN风险评分时序图')
plt.legend()
plt.grid(True, alpha=0.3)
plt.tight_layout()
plt.savefig('risk_score_timeline.png')
```

---

## 注意事项

### 1. 数据限制
- 这是**模拟测试数据**，不是真实链上数据
- 所有地址均为随机生成
- 价格和交易量遵循数学模型，不完全等同于真实市场
- 未包含所有可能的极端情况

### 2. 使用建议
- 仅用于系统测试和算法验证
- 不可用于真实交易决策
- 回测结果不代表实际收益
- 建议结合多个不同特征的测试数据

### 3. 扩展方向
如需更多测试数据，可以：
1. 修改`generate_test_data.py`中的参数
2. 生成不同价格走势的代币（上涨、横盘、下跌）
3. 增加随机事件概率
4. 调整流动性和交易量参数
5. 生成多个代币数据进行对比测试

---

## 文件位置
```
C:\Users\Administrator\Desktop\AGITHUB\solana\SolSniperPro\solana\data\bsc_testcoin_minute_data.csv
```

## 生成脚本
```
C:\Users\Administrator\Desktop\AGITHUB\solana\SolSniperPro\solana\generate_test_data.py
```

---

**文档版本**: v1.0
**生成时间**: 2025-11-10 18:11
**数据版本**: 2025-01-01 to 2025-09-30
