# SolSniper Pro

**专业的 Solana 代币狙击交易平台**

---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

## 📋 项目简介

SolSniper Pro 是一个功能完备的 Solana 代币自动狙击交易系统，提供智能策略引擎、实时代币监控、风险管理和高性能交易执行能力。

### ✨ 核心特性

- 🪙 **实时代币监控** - 自动发现和分析新创建的 Solana 代币
- 🎯 **智能策略引擎** - 6种预设交易策略，支持自定义配置
- 💹 **自动化交易** - 基于策略自动执行买卖操作
- 🛡️ **多层风险控制** - 仓位限制、止损止盈、黑名单管理
- 📊 **完整数据分析** - 性能指标、策略对比、交易分析
- 🔄 **实时更新** - WebSocket 实时推送交易和持仓变化
- 🚀 **高性能架构** - Rust 后端 + React 前端，毫秒级响应

---

## 🏗️ 技术架构

### 后端技术栈

- **Rust 1.75+** - 高性能系统语言
- **Tokio** - 异步运行时
- **Solana SDK 1.17** - 链上交互
- **PostgreSQL 16** - 数据存储
- **Redis 7** - 缓存层
- **Kafka** - 消息队列

### 前端技术栈

- **React 18** - UI 框架
- **TypeScript 5** - 类型安全
- **Vite 5** - 构建工具
- **TailwindCSS 3** - 样式框架
- **Zustand** - 状态管理
- **React Query** - 数据获取

---

## 🚀 快速开始

### 系统要求

- Rust 1.75+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 16
- Redis 7
- Kafka

### 一键启动

```bash
# 克隆项目
git clone https://github.com/your-repo/solsniper-pro.git
cd solsniper-pro

# 赋予脚本执行权限
chmod +x scripts/start.sh

# 启动所有服务
./scripts/start.sh
```

### 访问系统

- **前端界面**: http://localhost:5173
- **API 接口**: http://localhost:3000
- **API 文档**: http://localhost:3000/docs

**默认登录信息**:
- 用户名: `admin`
- 密码: `admin123`

---

## 📚 文档导航

### 中文文档

- [快速启动指南](./快速启动指南.md) - 系统安装和启动
- [使用手册](./使用手册.md) - 功能详细说明
- [系统架构](./docs/系统架构.md) - 架构设计文档
- [策略指南](./docs/策略指南.md) - 交易策略说明
- [API参考](./docs/API参考.md) - API接口文档
- [部署指南](./docs/部署指南.md) - 生产环境部署
- [前端架构](./docs/前端架构.md) - 前端技术文档

### English Documentation

- [Quick Start Guide](./QUICK_START.md) - Installation and startup
- [System Architecture](./docs/01_SYSTEM_ARCHITECTURE.md) - Architecture design
- [Strategy Guide](./docs/02_STRATEGY_GUIDE.md) - Trading strategies
- [API Reference](./docs/03_API_REFERENCE.md) - API documentation
- [Deployment Guide](./docs/04_DEPLOYMENT_GUIDE.md) - Production deployment
- [Frontend Architecture](./docs/05_FRONTEND_ARCHITECTURE.md) - Frontend tech docs

---

## 🎯 功能模块

### 1. 代币监控

- 实时发现新创建的 Solana 代币
- 自动分析代币流动性、持有者分布、合约安全性
- 基于风险评分的智能过滤
- 支持自定义筛选条件

### 2. 策略引擎

内置6种交易策略：

- **早鸟策略** (Early Bird) - 捕捉超早期代币
- **流动性猎手** (Liquidity Hunter) - 高流动性代币
- **成交量爆发** (Volume Explosion) - 成交量突增
- **价值投资** (Value Investing) - 被低估代币
- **逆向套利** (Contrarian Arbitrage) - 市场恐慌时机
- **时间基础** (Time-Based) - 特定时间窗口

### 3. 仓位管理

7种仓位管理策略：

- 固定金额 (Fixed Amount)
- 固定百分比 (Fixed Percentage)
- 波动率调整 (Volatility-Based)
- 凯利公式 (Kelly Criterion)
- 风险平价 (Risk Parity)
- 马丁格尔 (Martingale)
- 反马丁格尔 (Anti-Martingale)

### 4. 退出策略

7种智能退出机制：

- 止损 (Stop Loss)
- 止盈 (Take Profit)
- 移动止损 (Trailing Stop)
- 时间退出 (Time-Based)
- 流动性退出 (Liquidity-Based)
- 保本退出 (Break Even)
- 分批退出 (Scale Out)

### 5. 风险控制

- 仓位大小限制
- 每日亏损限制
- 最大回撤控制
- 代币/创建者黑名单
- 冷却期机制
- MEV 保护

### 6. 数据分析

- 实时交易指标
- 策略性能对比
- 盈亏曲线图
- 交易时段热力图
- 夏普比率、最大回撤等关键指标

---

## 📁 项目结构

```
SolSniperPro/
├── crates/                  # Rust 后端代码
│   ├── core/                 # 核心模块
│   ├── strategy-engine/      # 策略引擎
│   ├── risk-manager/         # 风险管理
│   └── api-server/           # API 服务器
│
├── frontend/                # React 前端
│   ├── src/
│   │   ├── components/       # UI 组件
│   │   ├── pages/            # 页面
│   │   ├── hooks/            # 自定义 Hooks
│   │   ├── stores/           # 状态管理
│   │   ├── api/              # API 客户端
│   │   └── types/            # TypeScript 类型
│   └── public/               # 静态资源
│
├── scripts/                 # 工具脚本
│   ├── start.sh              # 启动脚本
│   ├── stop.sh               # 停止脚本
│   └── init_db.sql           # 数据库初始化
│
├── docs/                    # 文档
│   ├── 01_SYSTEM_ARCHITECTURE.md
│   ├── 02_STRATEGY_GUIDE.md
│   ├── 03_API_REFERENCE.md
│   └── ...
│
├── docker-compose.yml       # Docker 编排
├── config.production.toml   # 生产配置
└── README.md                # 本文件
```

---

## ⚙️ 配置说明

### 环境变量 (.env)

```env
# 数据库
DATABASE_URL=postgresql://solsniper:password@localhost:5432/solsniper_db

# Redis
REDIS_URL=redis://localhost:6379

# Kafka
KAFKA_BROKERS=localhost:9092

# Solana RPC
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_WS_URL=wss://api.mainnet-beta.solana.com

# 钱包
WALLET_KEYPAIR_PATH=./wallet.json

# JWT
JWT_SECRET=your_secret_key_here
```

### 策略配置 (config.production.toml)

```toml
[strategies.early_bird]
enabled = true
priority = 90
min_liquidity_sol = 50
max_age_hours = 1
min_risk_score = 80

[position_sizing]
strategy = "FixedPercentage"
percentage = 10
max_position_sol = 20
```

---

## 🔧 开发指南

### 后端开发

```bash
# 编译
cargo build --release

# 运行测试
cargo test

# 启动 API 服务器
cargo run --bin api-server
```

### 前端开发

```bash
cd frontend

# 安装依赖
npm install

# 开发模式
npm run dev

# 生产构建
npm run build
```

---

## 📊 性能指标

- **事件延迟**: < 100ms
- **数据采集**: < 2s
- **策略匹配**: < 50ms
- **交易执行**: < 500ms
- **缓存命中率**: > 80%
- **系统吞吐量**: > 1000 TPS

---

## 🔒 安全特性

- JWT 认证授权
- 速率限制
- SQL 注入防护
- XSS 防护
- CORS 配置
- 敏感信息加密
- 审计日志

---

## 🚀 生产部署

### Docker 部署（推荐）

```bash
# 使用 Docker Compose
docker-compose up -d

# 查看日志
docker-compose logs -f
```

### 传统部署

参见 [部署指南](./docs/04_DEPLOYMENT_GUIDE.md)

---

## 📝 更新日志

### v2.0.0 (2025-12-21)

- ✅ 完整的前后端实现
- ✅ 6种交易策略
- ✅ 7种仓位管理策略
- ✅ 7种退出策略
- ✅ 完整的风险管理系统
- ✅ 实时 WebSocket 更新
- ✅ 专业的数据分析功能
- ✅ 生产级部署支持

---

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 📞 联系方式

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

## ⚠️ 免责声明

**风险提示**: 加密货币交易存在高风险，可能导致部分或全部资金损失。本软件仅供学习和研究使用，不构成任何投资建议。使用本软件进行实际交易的一切后果由使用者自行承担。

**使用须知**:
- 请在测试网充分测试后再使用主网
- 建议从小资金开始
- 设置严格的风险限制
- 定期备份数据和配置
- 及时更新系统

---

**⭐ 如果这个项目对您有帮助，请给个 Star！**

---

<p align="center">
  Made with ❤️ by Aitachi
</p>
