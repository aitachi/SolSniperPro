# SolSniper Pro - 架构设计文档

## 1. 系统架构总览

### 1.1 分层架构

```
┌─────────────────────────────────────────────────────────────────┐
│                       应用层 (Application Layer)                  │
│                      Web UI / CLI / API                          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                       业务逻辑层 (Business Layer)                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ ML风险模型   │  │ 聪明钱跟单   │  │ 行为模式识别 │          │
│  │ (P1)         │  │ (P1)         │  │ (P2)         │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ 策略引擎     │  │ 交易引擎     │  │ 风险分析器   │          │
│  │ (待实现)     │  │ (待实现)     │  │ (待实现)     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                       数据层 (Data Layer)                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ PostgreSQL   │  │ ScyllaDB     │  │ Redis        │          │
│  │ (关系数据)   │  │ (时序数据)   │  │ (缓存)       │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                       基础设施层 (Infrastructure)                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ Solana RPC   │  │ Kafka        │  │ Prometheus   │          │
│  │ (区块链)     │  │ (消息队列)   │  │ (监控)       │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

## 2. 核心模块详解

### 2.1 ML 风险评估模型 (crates/ml-model)

**职责**: 使用机器学习预测代币风险

**核心组件**:
- `FeatureExtractor`: 提取50维特征向量
- `RugPullClassifier`: Rug Pull 分类器
- `GainRegressor`: 涨幅回归器
- `OnlineLearningBuffer`: 在线学习缓冲区

**数据流**:
```
TokenInfo → FeatureExtractor → [50维向量]
                ↓
    ┌───────────┴───────────┐
    ↓                       ↓
Classifier              Regressor
    ↓                       ↓
Rug概率(0-1)          预期涨幅(%)
    └───────────┬───────────┘
                ↓
          MLPrediction
```

**特征工程**:
1. 基础特征 (4): 流动性、供应量、持有者数、年龄
2. 持有者分布 (3): top10/20/50 比例
3. 流动性特征 (3): LP锁定/燃烧、SOL价格
4. 合约安全 (4): 权限撤销、税费
5. 交易活动 (7): 交易笔数、买卖比例、交易量
6. 价格特征 (5): 价格、涨跌幅、波动率
7. 社交特征 (4): Twitter、Telegram、Discord、情绪分数
8. 衍生特征 (20): 动量、趋势、活跃度等

### 2.2 聪明钱跟单系统 (crates/smart-money-tracker)

**职责**: 识别并跟踪高胜率交易者

**核心组件**:
- `SmartWalletIdentifier`: 识别聪明钱钱包
- `SmartMoneyFollower`: 实时跟单执行
- `TradeAnalyzer`: 交易模式分析

**识别标准**:
```yaml
条件:
  总交易次数: >= 50
  胜率: >= 60%
  总收益: >= 100 SOL
  查询周期: 最近30天
```

**跟单流程**:
```
监控聪明钱地址
    ↓
检测买入交易
    ↓
快速风险评估 (risk_score >= 70)
    ↓
计算跟单金额 (10% * 胜率系数, max 2 SOL)
    ↓
发送跟单信号
    ↓
交易引擎执行
```

### 2.3 行为模式识别 (crates/behavior-pattern)

**职责**: 检测链上异常行为模式

**预定义模式**:
1. **快速撤池 Rug** (Critical)
   - 流动性下降 >80%
   - 创建者卖出 >50%

2. **慢速撤池 Rug** (High)
   - 流动性下降 >30%
   - LP解锁后撤池
   - 创建者逐步卖出 >30%

3. **协同拉盘** (Medium)
   - 20+钱包5分钟内协同买入
   - 交易量激增 >5x
   - 价格抛物线上涨

4. **有机增长** (Low)
   - 持有者稳步增长 (>10人/小时)
   - 交易量稳定 (波动率 <0.3)
   - 持有者分布良好 (top10 <40%)

5. **洗售交易** (High)
   - 同一钱包反复交易 >60%
   - 可疑交易比例 >70%

**匹配算法**:
```rust
confidence = Σ(matched_indicators * weight) / Σ(all_indicators * weight)

if confidence > threshold:
    发出模式匹配告警
```

## 3. 高并发设计

### 3.1 异步架构

使用 Tokio 异步运行时:
- **轻量级协程**: 单机支持 10万+ 并发任务
- **无锁并发**: 使用 DashMap 等无锁数据结构
- **流式处理**: 使用 Futures Stream 处理事件流

### 3.2 数据流管道

```
WebSocket订阅 (10个连接)
    ↓
事件解析 (并行)
    ↓
Kafka生产者 (批量提交)
    ↓
Kafka Topic (20个分区)
    ↓
Kafka消费者组 (50个消费者)
    ↓
并行处理 (1000个协程)
    ↓
写入数据库 (连接池)
```

### 3.3 性能优化

- **零拷贝**: 使用 Arc 共享数据,避免大对象拷贝
- **缓存**: Moka 异步缓存,减少数据库查询
- **连接池**: 复用 HTTP/数据库连接
- **批量操作**: 批量写入数据库,批量发送 Kafka 消息

## 4. 数据模型

### 4.1 PostgreSQL 表结构

```sql
-- 代币信息表
CREATE TABLE tokens (
    mint VARCHAR(44) PRIMARY KEY,
    symbol VARCHAR(20),
    name VARCHAR(100),
    created_at TIMESTAMP,
    -- ... 其他字段
);

-- 交易记录表
CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(44),
    token_address VARCHAR(44),
    side VARCHAR(4), -- 'buy' / 'sell'
    amount DECIMAL(20, 9),
    price DECIMAL(20, 12),
    profit_sol DECIMAL(20, 9),
    entry_time TIMESTAMP,
    exit_time TIMESTAMP,
    FOREIGN KEY (token_address) REFERENCES tokens(mint)
);

-- 聪明钱钱包表
CREATE TABLE smart_wallets (
    address VARCHAR(44) PRIMARY KEY,
    total_trades INT,
    profitable_trades INT,
    total_profit_sol DECIMAL(20, 9),
    win_rate DECIMAL(5, 4),
    last_active TIMESTAMP
);
```

### 4.2 ScyllaDB 表结构

```cql
-- 时序价格数据
CREATE TABLE price_history (
    token_address text,
    timestamp timestamp,
    price double,
    volume double,
    PRIMARY KEY (token_address, timestamp)
) WITH CLUSTERING ORDER BY (timestamp DESC);

-- 交易量时序数据
CREATE TABLE volume_metrics (
    token_address text,
    hour timestamp,
    total_volume double,
    buy_volume double,
    sell_volume double,
    tx_count int,
    PRIMARY KEY (token_address, hour)
) WITH CLUSTERING ORDER BY (hour DESC);
```

### 4.3 Redis 缓存策略

```yaml
缓存键:
  token:info:{mint}        # TokenInfo, TTL: 5分钟
  token:holders:{mint}     # 持有者列表, TTL: 10分钟
  token:risk_score:{mint}  # 风险评分, TTL: 1分钟
  smart_wallets:list       # 聪明钱列表, TTL: 1小时
```

## 5. 消息队列设计

### 5.1 Kafka Topics

```yaml
raw-events:
  partitions: 20
  replication: 3
  retention: 7天
  用途: 原始WebSocket事件流

pool-created:
  partitions: 10
  replication: 3
  retention: 7天
  用途: 新池子创建事件

token-analyzed:
  partitions: 10
  replication: 3
  retention: 3天
  用途: 代币分析完成事件

snipe-signals:
  partitions: 5
  replication: 3
  retention: 1天
  用途: 狙击信号

trade-executions:
  partitions: 5
  replication: 3
  retention: 30天
  用途: 交易执行结果

risk-alerts:
  partitions: 3
  replication: 3
  retention: 7天
  用途: 风险告警
```

### 5.2 消息格式

```json
{
  "type": "PoolCreated",
  "data": {
    "pool": "PoolAddress...",
    "token": "TokenAddress...",
    "timestamp": "2025-11-10T12:00:00Z"
  }
}
```

## 6. 部署架构

### 6.1 推荐配置

```yaml
服务器:
  类型: AWS c6i.2xlarge (8核32GB)
  操作系统: Ubuntu 22.04 LTS

服务部署:
  - Rust 服务 (Docker容器)
  - PostgreSQL (主从复制)
  - ScyllaDB (3节点集群)
  - Redis (哨兵模式)
  - Kafka (3节点集群)
  - Prometheus + Grafana (监控)
```

### 6.2 Docker Compose 示例

```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: solsniper
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: secret

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    environment:
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:9092

  # ... 其他服务
```

## 7. 监控指标

### 7.1 关键指标

```yaml
业务指标:
  - 新池子检测数/分钟
  - 风险评分完成数/秒
  - 交易执行成功率
  - 聪明钱识别数

性能指标:
  - 端到端延迟 (P50, P99)
  - 吞吐量 (TPS)
  - 内存使用率
  - CPU 使用率

可靠性指标:
  - 服务可用性
  - RPC 故障切换次数
  - 数据库慢查询数
  - Kafka 消费延迟
```

## 8. 安全措施

```yaml
私钥管理:
  - 使用 AWS KMS 加密存储
  - 不在代码中硬编码
  - 使用环境变量注入

API 安全:
  - JWT 认证
  - 限流 (100 req/min)
  - IP 白名单

数据安全:
  - TLS 1.3 加密通信
  - 数据库连接加密
  - 日志脱敏
```

---

## 附录: 完整文件清单

```
SolSniperPro/
├── Cargo.toml               # Workspace 配置
├── config.toml              # 系统配置
├── .env.example             # 环境变量示例
├── README.md                # 项目说明
├── ARCHITECTURE.md          # 本文档
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs
│   │       ├── error.rs
│   │       └── config.rs
│   ├── ml-model/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── feature_extractor.rs
│   │       ├── classifier.rs
│   │       ├── regressor.rs
│   │       └── online_learning.rs
│   ├── smart-money-tracker/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── identifier.rs
│   │       ├── follower.rs
│   │       └── analyzer.rs
│   └── behavior-pattern/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── patterns.rs
│           ├── indicators.rs
│           └── recognizer.rs
└── product.md               # 产品需求文档
```
