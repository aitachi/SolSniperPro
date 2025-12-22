#!/bin/bash

###############################################################################
# SolSniper Pro - 停止脚本
# 版本: v2.0.0
#
# Author: Aitachi
# Email: 44158892@qq.com
# Wechat: 18116011230
###############################################################################

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/logs"

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

echo ""
echo "╔══════════════════════════════════════════════════════╗"
echo "║                                                      ║"
echo "║           SolSniper Pro 停止脚本 v2.0.0             ║"
echo "║                                                      ║"
echo "╚══════════════════════════════════════════════════════╝"
echo ""

log_info "========== 停止服务 =========="

# 停止前端
if [ -f "$LOG_DIR/frontend.pid" ]; then
    PID=$(cat "$LOG_DIR/frontend.pid")
    if ps -p $PID > /dev/null; then
        log_info "停止前端服务 (PID: $PID)..."
        kill $PID
        rm "$LOG_DIR/frontend.pid"
        log_success "✓ 前端已停止"
    else
        log_warning "前端进程不存在"
        rm "$LOG_DIR/frontend.pid"
    fi
fi

# 停止 API 服务器
if [ -f "$LOG_DIR/api.pid" ]; then
    PID=$(cat "$LOG_DIR/api.pid")
    if ps -p $PID > /dev/null; then
        log_info "停止 API 服务器 (PID: $PID)..."
        kill $PID
        rm "$LOG_DIR/api.pid"
        log_success "✓ API 服务器已停止"
    else
        log_warning "API 服务器进程不存在"
        rm "$LOG_DIR/api.pid"
    fi
fi

# 停止 Docker 容器
log_info "停止 Docker 容器..."
cd "$PROJECT_ROOT"
docker-compose down

log_success "✓ Docker 容器已停止"

echo ""
log_success "✨ 所有服务已停止"
echo ""
