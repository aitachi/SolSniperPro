# SolSniper Pro - 部署运维文档

---

**Author**: Aitachi  
**Email**: 44158892@qq.com  
**Wechat**: 18116011230

---

**版本**: v2.0
**日期**: 2025-12-21
## 目录

1. [系统要求](#系统要求)
2. [依赖服务安装](#依赖服务安装)
3. [项目部署](#项目部署)
4. [配置文件](#配置文件)
5. [启动和停止](#启动和停止)
6. [监控和日志](#监控和日志)
7. [故障排查](#故障排查)
8. [备份和恢复](#备份和恢复)
9. [性能优化](#性能优化)
## 系统要求

### 硬件要求

#### 最小配置（开发/测试）
- CPU: 4 核心
- 内存: 16GB
- 存储: 100GB SSD
- 网络: 100Mbps

#### 推荐配置（生产环境）
- CPU: 16 核心 (或更多)
- 内存: 64GB
- 存储: 500GB NVMe SSD
- 网络: 1Gbps
- 备用电源: UPS

### 软件要求

| 软件 | 版本 | 说明 |
|------|------|------|
| **操作系统** | Ubuntu 22.04 LTS | 推荐 |
| **Rust** | 1.75+ | 编译环境 |
| **Node.js** | 18.x+ | 前端构建 |
| **Docker** | 24.0+ | 容器化（可选） |
| **Docker Compose** | 2.20+ | 服务编排（可选） |
## 依赖服务安装

### 1. Kafka 安装

**使用 Docker**:
```bash
# 创建 docker-compose.yml
cat > kafka-docker-compose.yml <<EOF
version: '3.8'
services:
  zookeeper:
    image: confluentinc/cp-zookeeper:7.5.0
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    ports:
      - "2181:2181"

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      KAFKA_AUTO_CREATE_TOPICS_ENABLE: "true"
EOF

# 启动
docker-compose -f kafka-docker-compose.yml up -d
```

**验证**:
```bash
# 查看 topics
docker exec -it kafka kafka-topics --list --bootstrap-server localhost:9092
```

### 2. PostgreSQL 安装

**使用 Docker**:
```bash
docker run -d \
  --name solsniper-postgres \
  -e POSTGRES_USER=solsniper \
  -e POSTGRES_PASSWORD=your_password \
  -e POSTGRES_DB=solsniper_db \
  -p 5432:5432 \
  -v postgres_data:/var/lib/postgresql/data \
  postgres:16-alpine
```

**初始化数据库**:
```bash
# 连接数据库
psql -h localhost -U solsniper -d solsniper_db

# 执行初始化脚本
\i scripts/init_db.sql
```

**init_db.sql**:
```sql
CREATE TABLE IF NOT EXISTS tokens (
    mint VARCHAR(44) PRIMARY KEY,
    symbol VARCHAR(10),
    name VARCHAR(100),
    data JSONB NOT NULL,
    risk_score FLOAT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_tokens_created_at ON tokens(created_at DESC);
CREATE INDEX idx_tokens_risk_score ON tokens(risk_score DESC);

CREATE TABLE IF NOT EXISTS trades (
    id VARCHAR(36) PRIMARY KEY,
    mint VARCHAR(44) REFERENCES tokens(mint),
    side VARCHAR(4) NOT NULL,
    strategy VARCHAR(50),
    amount_sol NUMERIC(20, 9),
    price_usd NUMERIC(20, 9),
    status VARCHAR(20),
    tx_signature VARCHAR(88),
    created_at TIMESTAMP DEFAULT NOW(),
    executed_at TIMESTAMP
);

CREATE INDEX idx_trades_created_at ON trades(created_at DESC);
CREATE INDEX idx_trades_status ON trades(status);
```

### 3. Redis 安装

**使用 Docker**:
```bash
docker run -d \
  --name solsniper-redis \
  -p 6379:6379 \
  -v redis_data:/data \
  redis:7-alpine \
  redis-server --appendonly yes
```

**验证**:
```bash
redis-cli ping
# 应返回: PONG
```

### 4. 完整 Docker Compose

**docker-compose.yml**:
```yaml
version: '3.8'

services:
  zookeeper:
    image: confluentinc/cp-zookeeper:7.5.0
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    volumes:
      - zookeeper_data:/var/lib/zookeeper/data
      - zookeeper_log:/var/lib/zookeeper/log

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      KAFKA_AUTO_CREATE_TOPICS_ENABLE: "true"
    volumes:
      - kafka_data:/var/lib/kafka/data

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: solsniper
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: solsniper_db
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init_db.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U solsniper"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 3

volumes:
  zookeeper_data:
  zookeeper_log:
  kafka_data:
  postgres_data:
  redis_data:
```

**启动所有服务**:
```bash
# 设置环境变量
export POSTGRES_PASSWORD=your_secure_password

# 启动
docker-compose up -d

# 查看状态
docker-compose ps

# 查看日志
docker-compose logs -f
```
## 项目部署

### 1. 获取代码

```bash
# 克隆仓库
git clone https://github.com/your-org/solsniper-pro.git
cd solsniper-pro

# 检出特定版本
git checkout v2.0.0
```

### 2. 编译项目

```bash
# 编译 Release 版本
cargo build --release

# 编译时间约 5-10 分钟
# 生成的二进制文件在: target/release/
```

### 3. 前端构建

```bash
cd frontend

# 安装依赖
npm install

# 构建生产版本
npm run build

# 生成的文件在: dist/
```

### 4. 配置文件准备

```bash
# 复制配置模板
cp config.example.toml config.production.toml

# 编辑配置
vi config.production.toml
```
## 配置文件

### config.production.toml

```toml
[general]
environment = "production"
log_level = "info"
enable_trading = true
max_concurrent_trades = 5

[server]
host = "0.0.0.0"
port = 3000
cors_origins = ["https://your-frontend-domain.com"]

[rpc]
endpoints = [
    "https://your-premium-rpc-1.com",
    "https://your-premium-rpc-2.com",
    "https://api.mainnet-beta.solana.com"
]
timeout_seconds = 30
max_retries = 5
health_check_interval_secs = 30

[websocket]
url = "wss://your-premium-ws-endpoint.com"
reconnect_interval_secs = 5
max_reconnect_attempts = 10

[kafka]
brokers = ["localhost:9092"]
topic_prefix = "solsniper"
consumer_group = "solsniper-group-prod"

[database]
url = "postgresql://solsniper:password@localhost:5432/solsniper_db"
max_connections = 10
min_connections = 2

[redis]
url = "redis://localhost:6379"
connection_pool_size = 10
default_ttl_seconds = 120

[cache]
l1_max_capacity = 10000
l1_ttl_seconds = 30
l2_ttl_seconds = 120

[wallet]
keypair_path = "/secure/path/to/wallet.json"
additional_wallets_path = "/secure/path/to/additional_wallets/"

[jito]
block_engine_url = "https://mainnet.block-engine.jito.wtf"
default_tip_lamports = 10000

[strategies.early_bird]
enabled = true
max_age_minutes = 10
min_liquidity_sol = 10.0
min_risk_score = 75.0

[strategies.liquidity_hunter]
enabled = true
min_liquidity_sol = 50.0
max_liquidity_sol = 200.0

[position_sizing]
strategy = "KellyCriterion"
kelly_fraction = 0.25
min_position_sol = 0.5
max_position_sol = 50.0

[exit_strategy]
stop_loss_pct = 20.0
take_profit_pct = 50.0
trailing_activation_pct = 20.0
trailing_stop_pct = 10.0
max_holding_minutes = 240

[risk_control]
max_position_per_token = 20.0
max_total_position = 100.0
max_trades_per_day = 50
max_daily_loss_sol = 10.0
```

### 环境变量文件 (.env)

```bash
# 环境
APP_ENV=production

# 数据库密码
POSTGRES_PASSWORD=your_secure_password

# Redis密码（如果有）
REDIS_PASSWORD=

# 钱包密钥路径
WALLET_KEYPAIR_PATH=/secure/path/to/wallet.json

# RPC 端点（可选，覆盖配置文件）
# SOLANA_RPC_URL=https://your-rpc.com
# SOLANA_WS_URL=wss://your-ws.com

# JWT密钥
JWT_SECRET=your_jwt_secret_key_change_this

# 日志级别
RUST_LOG=info
```
## 启动和停止

### 使用启动脚本（推荐）

**scripts/start.sh**:
```bash
#!/bin/bash

# 见下一节完整启动脚本
```

```bash
# 启动所有服务
./scripts/start.sh

# 停止所有服务
./scripts/stop.sh

# 重启服务
./scripts/restart.sh
```

### 手动启动

```bash
# 1. 启动依赖服务
docker-compose up -d

# 2. 启动 Solsniper Pro
cd /path/to/solsniper-pro

# 设置环境变量
export APP_ENV=production
source .env

# 启动后端服务
nohup ./target/release/solsniper-api-server > logs/api.log 2>&1 &
nohup ./target/release/solsniper-data-collector > logs/collector.log 2>&1 &
nohup ./target/release/solsniper-trading-engine > logs/trading.log 2>&1 &

# 3. 启动前端（使用 Nginx）
# 已通过 Nginx 配置静态文件服务
```

### Systemd 服务配置

**/etc/systemd/system/solsniper-api.service**:
```ini
[Unit]
Description=SolSniper Pro API Server
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=solsniper
Group=solsniper
WorkingDirectory=/opt/solsniper-pro
Environment="APP_ENV=production"
Environment="RUST_LOG=info"
EnvironmentFile=/opt/solsniper-pro/.env
ExecStart=/opt/solsniper-pro/target/release/solsniper-api-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# 启用服务
sudo systemctl enable solsniper-api
sudo systemctl start solsniper-api

# 查看状态
sudo systemctl status solsniper-api

# 查看日志
sudo journalctl -u solsniper-api -f
```
## 监控和日志

### 日志配置

**使用 tracing-subscriber**:
```rust
// 在 main.rs 中
tracing_subscriber::fmt()
    .with_env_filter("info")
    .with_target(false)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .json()  // JSON 格式，便于解析
    .init();
```

### 日志查看

```bash
# 查看实时日志
tail -f logs/api.log

# 搜索错误
grep ERROR logs/*.log

# 按时间范围查看
sed -n '/2025-12-21T10:00/,/2025-12-21T11:00/p' logs/api.log
```

### Prometheus 监控

**prometheus.yml**:
```yaml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'solsniper'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: '/api/v1/metrics/export'
```

### Grafana 仪表板

导入预配置的仪表板：
```bash
# 仪表板 JSON 在: scripts/grafana-dashboard.json
```

**关键指标**:
- 交易成功率
- 平均 PnL
- RPC 延迟
- 缓存命中率
- 活跃持仓数量
## 故障排查

### 常见问题

#### 1. RPC 连接失败

**症状**: 日志显示 "Failed to connect to RPC"

**排查**:
```bash
# 测试 RPC 连接
curl -X POST -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' \
  https://api.mainnet-beta.solana.com

# 检查配置文件 RPC 端点
grep endpoints config.production.toml
```

**解决**:
- 使用付费 RPC（Helius, QuickNode）
- 增加超时时间
- 添加更多备用端点

#### 2. Kafka 连接失败

**症状**: "Failed to produce to Kafka"

**排查**:
```bash
# 检查 Kafka 是否运行
docker ps | grep kafka

# 测试连接
docker exec -it kafka kafka-topics --list --bootstrap-server localhost:9092
```

**解决**:
```bash
# 重启 Kafka
docker-compose restart kafka

# 手动创建 topics
docker exec -it kafka kafka-topics --create \
  --bootstrap-server localhost:9092 \
  --topic solsniper-events \
  --partitions 3 \
  --replication-factor 1
```

#### 3. 内存占用过高

**症状**: 系统内存使用 > 90%

**排查**:
```bash
# 查看进程内存
ps aux --sort=-%mem | head -10

# 查看缓存统计
redis-cli INFO memory
```

**解决**:
- 降低 L1 缓存容量
- 清理旧的历史数据
- 增加物理内存

#### 4. 交易失败率高

**症状**: 大量交易状态为 "failed"

**排查**:
```bash
# 查看失败交易日志
grep "Trade failed" logs/trading.log | tail -20

# 查看钱包余额
solana balance /path/to/wallet.json
```

**解决**:
- 检查钱包 SOL 余额
- 增加滑点容忍度
- 提高优先费用
## 备份和恢复

### 数据库备份

**自动备份脚本**:
```bash
#!/bin/bash
# scripts/backup_db.sh

BACKUP_DIR="/backup/solsniper"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/solsniper_db_$DATE.sql.gz"

# 创建备份
pg_dump -h localhost -U solsniper -d solsniper_db | gzip > "$BACKUP_FILE"

# 保留最近 7 天的备份
find "$BACKUP_DIR" -name "solsniper_db_*.sql.gz" -mtime +7 -delete

echo "Backup completed: $BACKUP_FILE"
```

**Crontab 配置**:
```bash
# 每天凌晨 2 点备份
0 2 * * * /opt/solsniper-pro/scripts/backup_db.sh
```

### 数据恢复

```bash
# 停止服务
systemctl stop solsniper-api

# 恢复数据库
gunzip < /backup/solsniper/solsniper_db_20251221_020000.sql.gz | \
  psql -h localhost -U solsniper -d solsniper_db

# 重启服务
systemctl start solsniper-api
```

### 配置文件备份

```bash
# 备份配置和钱包
tar -czf solsniper-config-$(date +%Y%m%d).tar.gz \
  config.production.toml \
  .env \
  /secure/path/to/wallet.json
```
## 性能优化

### 数据库优化

```sql
-- 创建索引
CREATE INDEX CONCURRENTLY idx_tokens_liquidity ON tokens(((data->>'liquidity_sol')::float) DESC);
CREATE INDEX CONCURRENTLY idx_trades_mint_created ON trades(mint, created_at DESC);

-- 分析表
ANALYZE tokens;
ANALYZE trades;

-- 清理旧数据
DELETE FROM tokens WHERE created_at < NOW() - INTERVAL '30 days';
VACUUM ANALYZE tokens;
```

### Redis 优化

```bash
# redis.conf 配置
maxmemory 8gb
maxmemory-policy allkeys-lru
save ""  # 禁用 RDB，只使用 AOF
appendonly yes
appendfsync everysec
```

### Rust 性能优化

**Cargo.toml**:
```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
```

### 系统调优

```bash
# /etc/sysctl.conf

# 网络优化
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_tw_reuse = 1

# 文件描述符
fs.file-max = 2097152

# 应用配置
sudo sysctl -p
```
**文档版本**: v2.0.0
**最后更新**: 2025-12-21