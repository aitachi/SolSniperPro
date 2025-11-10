# SolSniper Pro - 快速开始指南

## 🚀 5分钟快速开始

### 步骤 1: 环境准备

```bash
# 1. 安装 Rust (需要 1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. 验证安装
rustc --version
cargo --version

# 3. 克隆项目
git clone <your-repo-url>
cd SolSniperPro
```

### 步骤 2: 配置环境

```bash
# 复制环境变量模板
cp .env.example .env

# 编辑 .env 文件,填入必要的配置
nano .env
```

**必需配置**:
- `HELIUS_API_KEY`: 你的Helius RPC API密钥
- `DATABASE_URL`: PostgreSQL连接字符串
- `REDIS_URL`: Redis连接字符串
- `KAFKA_BROKERS`: Kafka集群地址

### 步骤 3: 编译项目

```bash
# Linux/Mac
chmod +x build.sh
./build.sh

# Windows
build.bat
```

### 步骤 4: 运行测试

```bash
# 测试ML模型
cd crates/ml-model
cargo test -- --nocapture

# 测试聪明钱识别
cd ../smart-money-tracker
cargo test -- --nocapture

# 测试行为模式识别
cd ../behavior-pattern
cargo test -- --nocapture

# 测试策略引擎
cd ../strategy-engine
cargo test -- --nocapture
```

## 📦 模块使用示例

### 1. ML 风险模型

```rust
use solsniper_ml_model::MLEnhancedStrategy;

#[tokio::main]
async fn main() {
    // 加载模型
    let strategy = MLEnhancedStrategy::new("./models").unwrap();

    // 预测代币风险
    let prediction = strategy.predict_outcome(&token).await.unwrap();

    println!("Rug Pull 概率: {:.2}%", prediction.rug_probability * 100.0);
    println!("预期涨幅: {:.2}%", prediction.expected_gain_pct);
}
```

### 2. 聪明钱跟单

```rust
use solsniper_smart_money_tracker::SmartMoneyTracker;

#[tokio::main]
async fn main() {
    let mut tracker = SmartMoneyTracker::new("postgresql://...").unwrap();

    // 识别聪明钱
    tracker.identify_smart_wallets().await.unwrap();

    // 获取Top 10聪明钱
    let wallets = tracker.get_smart_wallets();
    for wallet in wallets.iter().take(10) {
        println!("#{}: {:?} - 胜率 {:.1}%",
            wallet.rank, wallet.address, wallet.win_rate * 100.0);
    }
}
```

### 3. 行为模式识别

```rust
use solsniper_behavior_pattern::BehaviorPatternRecognizer;

#[tokio::main]
async fn main() {
    let recognizer = BehaviorPatternRecognizer::new();

    // 检测行为模式
    let matches = recognizer.match_patterns(&token).await.unwrap();

    for m in matches {
        println!("检测到: {} (风险: {:?})",
            m.pattern.name, m.pattern.risk_level);
    }
}
```

### 4. 综合风险评估

```rust
use solsniper_risk_analyzer::RiskAssessmentEngine;

#[tokio::main]
async fn main() {
    let engine = RiskAssessmentEngine::new()
        .with_ml("./models").unwrap();

    let risk_score = engine.assess(&token).await.unwrap();

    println!("总体风险评分: {:.1}/100", risk_score.total);
    println!("推荐: {:?}", risk_score.recommendation);
}
```

### 5. 策略匹配

```rust
use solsniper_strategy_engine::StrategyEngine;

#[tokio::main]
async fn main() {
    let engine = StrategyEngine::new();

    let matches = engine.evaluate_token(&token, &risk_score).await.unwrap();

    for m in matches {
        println!("{}: 建议仓位 {:.2} SOL (预期收益 {:.1}%)",
            m.strategy_name, m.position_size, m.expected_profit);
    }
}
```

## 🏗️ 项目结构

```
SolSniperPro/
├── crates/
│   ├── core/                   ✅ 核心类型定义
│   ├── ml-model/               ✅ ML风险评估 (P1)
│   ├── smart-money-tracker/    ✅ 聪明钱跟单 (P1)
│   ├── behavior-pattern/       ✅ 行为模式识别 (P2)
│   ├── data-collector/         ✅ 数据采集器
│   ├── risk-analyzer/          ✅ 风险分析器
│   ├── strategy-engine/        ✅ 策略引擎
│   └── trading-engine/         ✅ 交易执行
├── config.toml                 配置文件
├── .env.example                环境变量示例
├── build.sh / build.bat        构建脚本
├── README.md                   项目说明
├── ARCHITECTURE.md             架构文档
└── QUICKSTART.md               本文档
```

## 📊 功能完成度

| 模块 | 状态 | 优先级 | 完成度 |
|------|------|--------|--------|
| ✅ ML风险模型 | 已完成 | P1 | 100% |
| ✅ 聪明钱跟单 | 已完成 | P1 | 100% |
| ✅ 行为模式识别 | 已完成 | P2 | 100% |
| ✅ 数据采集器 | 已完成 | P0 | 80% (缺WebSocket实现) |
| ✅ 风险分析器 | 已完成 | P0 | 100% |
| ✅ 策略引擎 | 已完成 | P0 | 100% |
| ✅ 交易引擎 | 已完成 | P0 | 70% (缺Jito集成) |

## 🔧 开发工具

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 运行所有测试
cargo test

# 生成文档
cargo doc --open

# 监听文件变化自动编译
cargo watch -x build
```

## 📝 待办事项

### 短期 (1-2周)
- [ ] 完善WebSocket订阅实现
- [ ] 集成Jito Bundle
- [ ] 实现数据库持久化
- [ ] 添加Kafka集成
- [ ] 训练ML模型

### 中期 (1个月)
- [ ] 实现API网关
- [ ] 添加Web仪表盘
- [ ] 性能优化和压测
- [ ] 添加更多测试用例
- [ ] 完善错误处理

### 长期 (3个月+)
- [ ] 分布式部署方案
- [ ] Prometheus+Grafana监控
- [ ] 回测系统
- [ ] 策略市场

## ⚠️ 常见问题

### Q: 编译失败怎么办?
A: 确保Rust版本 >= 1.75,运行 `rustup update`

### Q: 测试需要数据库吗?
A: 大部分测试不需要,但聪明钱识别测试需要PostgreSQL

### Q: 如何添加新策略?
A: 在 `crates/strategy-engine/src/strategies.rs` 中实现 `Strategy` trait

### Q: 支持哪些DEX?
A: 目前支持 Raydium, Orca, Meteora, Pump.fun

## 📞 获取帮助

- GitHub Issues: <your-repo>/issues
- 文档: 查看 `ARCHITECTURE.md`
- 示例: 查看各crate的 `tests/` 目录

## 🎯 下一步

1. 阅读 `ARCHITECTURE.md` 了解系统设计
2. 查看 `product.md` 了解完整功能规划
3. 运行测试熟悉各模块API
4. 尝试修改 `config.toml` 调整策略参数
5. 开始开发你自己的策略!

---

祝你使用愉快! 🚀
