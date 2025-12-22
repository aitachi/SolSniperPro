#!/bin/bash

###############################################################################
# SolSniper Pro - 全流程启动脚本
# 版本: v2.0.0
# 日期: 2025-12-21
#
# Author: Aitachi
# Email: 44158892@qq.com
# Wechat: 18116011230
#
# 功能:
# 1. 检查系统依赖
# 2. 启动依赖服务 (Kafka, PostgreSQL, Redis)
# 3. 编译和启动后端服务
# 4. 构建和启动前端
# 5. 健康检查
###############################################################################

set -e  # 遇到错误立即退出

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/logs"
CONFIG_FILE="$PROJECT_ROOT/config.production.toml"
ENV_FILE="$PROJECT_ROOT/.env"

# 端口配置
API_PORT=3000
FRONTEND_PORT=5173
KAFKA_PORT=9092
POSTGRES_PORT=5432
REDIS_PORT=6379

###############################################################################
# 工具函数
###############################################################################

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_command() {
    if ! command -v $1 &> /dev/null; then
        log_error "$1 未安装，请先安装"
        return 1
    fi
    return 0
}

check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1 ; then
        return 0
    else
        return 1
    fi
}

wait_for_service() {
    local service=$1
    local port=$2
    local max_wait=$3
    local waited=0

    log_info "等待 $service 启动 (端口 $port)..."

    while [ $waited -lt $max_wait ]; do
        if check_port $port; then
            log_success "$service 已启动"
            return 0
        fi
        sleep 1
        ((waited++))
        echo -ne "\r等待中... ${waited}s / ${max_wait}s"
    done

    echo ""
    log_error "$service 启动超时"
    return 1
}

###############################################################################
# 1. 环境检查
###############################################################################

check_dependencies() {
    log_info "========== 检查系统依赖 =========="

    local missing_deps=0

    # 检查必需命令
    for cmd in cargo node npm docker docker-compose psql redis-cli; do
        if check_command $cmd; then
            log_success "✓ $cmd 已安装"
        else
            log_error "✗ $cmd 未安装"
            ((missing_deps++))
        fi
    done

    if [ $missing_deps -gt 0 ]; then
        log_error "缺少 $missing_deps 个依赖，请先安装"
        exit 1
    fi

    # 检查 Rust 版本
    local rust_version=$(cargo --version | awk '{print $2}')
    log_info "Rust 版本: $rust_version"

    # 检查 Node 版本
    local node_version=$(node --version)
    log_info "Node.js 版本: $node_version"

    log_success "依赖检查完成"
}

###############################################################################
# 2. 创建必要目录
###############################################################################

setup_directories() {
    log_info "========== 创建目录结构 =========="

    mkdir -p "$LOG_DIR"
    mkdir -p "$PROJECT_ROOT/data/postgres"
    mkdir -p "$PROJECT_ROOT/data/redis"
    mkdir -p "$PROJECT_ROOT/data/kafka"

    log_success "目录创建完成"
}

###############################################################################
# 3. 检查配置文件
###############################################################################

check_configuration() {
    log_info "========== 检查配置文件 =========="

    # 检查配置文件是否存在
    if [ ! -f "$CONFIG_FILE" ]; then
        if [ -f "$PROJECT_ROOT/config.example.toml" ]; then
            log_warning "配置文件不存在，从模板复制..."
            cp "$PROJECT_ROOT/config.example.toml" "$CONFIG_FILE"
            log_warning "请编辑 $CONFIG_FILE 配置正确的参数"
            read -p "按 Enter 继续，或 Ctrl+C 退出编辑配置文件..."
        else
            log_error "配置文件和模板都不存在"
            exit 1
        fi
    fi

    # 检查环境变量文件
    if [ ! -f "$ENV_FILE" ]; then
        log_warning "环境变量文件不存在，创建模板..."
        cat > "$ENV_FILE" <<EOF
# SolSniper Pro 环境变量配置

APP_ENV=production
RUST_LOG=info

# 数据库
POSTGRES_PASSWORD=change_this_password
DATABASE_URL=postgresql://solsniper:change_this_password@localhost:5432/solsniper_db

# Redis
REDIS_URL=redis://localhost:6379

# Kafka
KAFKA_BROKERS=localhost:9092

# 钱包（请配置实际路径）
WALLET_KEYPAIR_PATH=./wallet.json

# JWT
JWT_SECRET=change_this_secret_key_$(date +%s)

# RPC 端点（可选，覆盖配置文件）
# SOLANA_RPC_URL=https://your-rpc.com
# SOLANA_WS_URL=wss://your-ws.com
EOF
        log_warning "请编辑 $ENV_FILE 配置正确的参数"
        read -p "按 Enter 继续，或 Ctrl+C 退出编辑..."
    fi

    # 加载环境变量
    source "$ENV_FILE"

    log_success "配置检查完成"
}

###############################################################################
# 4. 启动依赖服务 (Docker)
###############################################################################

start_dependencies() {
    log_info "========== 启动依赖服务 =========="

    cd "$PROJECT_ROOT"

    # 检查 docker-compose.yml 是否存在
    if [ ! -f "docker-compose.yml" ]; then
        log_warning "docker-compose.yml 不存在，创建..."
        cat > docker-compose.yml <<'EOF'
version: '3.8'

services:
  zookeeper:
    image: confluentinc/cp-zookeeper:7.5.0
    container_name: solsniper-zookeeper
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    volumes:
      - ./data/zookeeper/data:/var/lib/zookeeper/data
      - ./data/zookeeper/log:/var/lib/zookeeper/log
    restart: unless-stopped

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    container_name: solsniper-kafka
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
      - ./data/kafka:/var/lib/kafka/data
    restart: unless-stopped

  postgres:
    image: postgres:16-alpine
    container_name: solsniper-postgres
    environment:
      POSTGRES_USER: solsniper
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:-change_this}
      POSTGRES_DB: solsniper_db
    ports:
      - "5432:5432"
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
      - ./scripts/init_db.sql:/docker-entrypoint-initdb.d/init.sql:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U solsniper"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    container_name: solsniper-redis
    ports:
      - "6379:6379"
    volumes:
      - ./data/redis:/data
    command: redis-server --appendonly yes --maxmemory 2gb --maxmemory-policy allkeys-lru
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 3
    restart: unless-stopped
EOF
    fi

    # 启动容器
    log_info "启动 Docker 容器..."
    docker-compose up -d

    # 等待服务启动
    wait_for_service "Kafka" $KAFKA_PORT 60 || exit 1
    wait_for_service "PostgreSQL" $POSTGRES_PORT 30 || exit 1
    wait_for_service "Redis" $REDIS_PORT 30 || exit 1

    # 验证服务
    log_info "验证服务状态..."

    # 检查 Kafka
    if docker exec solsniper-kafka kafka-topics --list --bootstrap-server localhost:9092 &>/dev/null; then
        log_success "✓ Kafka 正常"
    else
        log_error "✗ Kafka 异常"
    fi

    # 检查 PostgreSQL
    if PGPASSWORD=$POSTGRES_PASSWORD psql -h localhost -U solsniper -d solsniper_db -c "SELECT 1" &>/dev/null; then
        log_success "✓ PostgreSQL 正常"
    else
        log_warning "✗ PostgreSQL 可能需要初始化"
    fi

    # 检查 Redis
    if redis-cli ping &>/dev/null; then
        log_success "✓ Redis 正常"
    else
        log_error "✗ Redis 异常"
    fi

    log_success "依赖服务启动完成"
}

###############################################################################
# 5. 编译后端
###############################################################################

build_backend() {
    log_info "========== 编译后端代码 =========="

    cd "$PROJECT_ROOT"

    # 检查是否需要编译
    if [ -f "target/release/solsniper-api-server" ] && [ "$1" != "--force" ]; then
        log_info "发现已编译的二进制文件，跳过编译（使用 --force 强制重新编译）"
    else
        log_info "开始编译（这可能需要 5-10 分钟）..."

        # 编译
        RUST_LOG=info cargo build --release

        if [ $? -eq 0 ]; then
            log_success "后端编译完成"
        else
            log_error "后端编译失败"
            exit 1
        fi
    fi
}

###############################################################################
# 6. 启动后端服务
###############################################################################

start_backend() {
    log_info "========== 启动后端服务 =========="

    cd "$PROJECT_ROOT"

    # 加载环境变量
    source "$ENV_FILE"

    # 启动 API 服务器
    log_info "启动 API 服务器..."
    nohup ./target/release/solsniper-api-server \
        > "$LOG_DIR/api.log" 2>&1 &
    echo $! > "$LOG_DIR/api.pid"

    # 等待 API 服务器启动
    wait_for_service "API Server" $API_PORT 30 || exit 1

    # 测试 API
    if curl -s http://localhost:$API_PORT/health > /dev/null; then
        log_success "✓ API Server 正常运行"
    else
        log_warning "API Server 健康检查失败，查看日志: $LOG_DIR/api.log"
    fi

    log_success "后端服务启动完成"
}

###############################################################################
# 7. 构建和启动前端
###############################################################################

build_and_start_frontend() {
    log_info "========== 构建前端 =========="

    cd "$PROJECT_ROOT/frontend"

    # 安装依赖
    if [ ! -d "node_modules" ]; then
        log_info "安装前端依赖..."
        npm install
    fi

    # 构建生产版本
    if [ "$1" == "--production" ]; then
        log_info "构建生产版本..."
        npm run build

        # 使用简单的 HTTP 服务器启动
        log_info "启动生产前端..."
        npx serve -s dist -l $FRONTEND_PORT > "$LOG_DIR/frontend.log" 2>&1 &
        echo $! > "$LOG_DIR/frontend.pid"
    else
        # 开发模式
        log_info "启动开发服务器..."
        npm run dev > "$LOG_DIR/frontend.log" 2>&1 &
        echo $! > "$LOG_DIR/frontend.pid"
    fi

    # 等待前端启动
    wait_for_service "Frontend" $FRONTEND_PORT 30 || exit 1

    log_success "前端启动完成"
}

###############################################################################
# 8. 显示系统状态
###############################################################################

show_status() {
    log_info "========== 系统状态 =========="

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  🚀 SolSniper Pro 已启动"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "  📱 前端地址:    http://localhost:$FRONTEND_PORT"
    echo "  🔌 API 地址:    http://localhost:$API_PORT"
    echo "  📊 API 文档:    http://localhost:$API_PORT/docs"
    echo "  ❤️  健康检查:    http://localhost:$API_PORT/health"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "  服务状态:"
    echo "  ├─ Kafka:       $(check_port $KAFKA_PORT && echo '✓ 运行中' || echo '✗ 未运行')"
    echo "  ├─ PostgreSQL:  $(check_port $POSTGRES_PORT && echo '✓ 运行中' || echo '✗ 未运行')"
    echo "  ├─ Redis:       $(check_port $REDIS_PORT && echo '✓ 运行中' || echo '✗ 未运行')"
    echo "  ├─ API Server:  $(check_port $API_PORT && echo '✓ 运行中' || echo '✗ 未运行')"
    echo "  └─ Frontend:    $(check_port $FRONTEND_PORT && echo '✓ 运行中' || echo '✗ 未运行')"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "  日志文件:"
    echo "  ├─ API:         $LOG_DIR/api.log"
    echo "  ├─ Frontend:    $LOG_DIR/frontend.log"
    echo "  └─ Docker:      docker-compose logs -f"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "  📝 查看实时日志:"
    echo "     tail -f $LOG_DIR/api.log"
    echo ""
    echo "  🛑 停止所有服务:"
    echo "     ./scripts/stop.sh"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

###############################################################################
# 主流程
###############################################################################

main() {
    clear
    echo ""
    echo "╔══════════════════════════════════════════════════════╗"
    echo "║                                                      ║"
    echo "║           SolSniper Pro 启动脚本 v2.0.0             ║"
    echo "║                                                      ║"
    echo "╚══════════════════════════════════════════════════════╝"
    echo ""

    # 解析参数
    local FORCE_BUILD=false
    local PRODUCTION_MODE=false

    while [[ $# -gt 0 ]]; do
        case $1 in
            --force)
                FORCE_BUILD=true
                shift
                ;;
            --production)
                PRODUCTION_MODE=true
                shift
                ;;
            --help)
                echo "用法: $0 [选项]"
                echo ""
                echo "选项:"
                echo "  --force        强制重新编译后端"
                echo "  --production   使用生产模式启动前端"
                echo "  --help         显示此帮助信息"
                exit 0
                ;;
            *)
                log_error "未知参数: $1"
                exit 1
                ;;
        esac
    done

    # 执行启动流程
    check_dependencies
    setup_directories
    check_configuration
    start_dependencies

    if [ "$FORCE_BUILD" = true ]; then
        build_backend --force
    else
        build_backend
    fi

    start_backend

    if [ "$PRODUCTION_MODE" = true ]; then
        build_and_start_frontend --production
    else
        build_and_start_frontend
    fi

    show_status

    log_success "✨ 所有服务启动完成！"
}

# 运行主流程
main "$@"
