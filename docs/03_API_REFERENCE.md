# SolSniper Pro - API 接口文档
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
**Base URL**: `http://localhost:3000/api/v1`

---

## 目录

1. [认证](#认证)
2. [通用规范](#通用规范)
3. [代币管理 API](#代币管理-api)
4. [策略管理 API](#策略管理-api)
5. [交易管理 API](#交易管理-api)
6. [风险控制 API](#风险控制-api)
7. [监控指标 API](#监控指标-api)
8. [系统配置 API](#系统配置-api)
9. [WebSocket API](#websocket-api)
10. [错误码](#错误码)

---

## 认证

### JWT Token认证

所有 API 请求需要在 Header 中携带 JWT Token：

```http
Authorization: Bearer <your_jwt_token>
```

### 获取 Token

**端点**: `POST /api/v1/auth/login`

**请求体**:
```json
{
  "username": "admin",
  "password": "your_password"
}
```

**响应**:
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 86400,
    "user": {
      "id": "user_123",
      "username": "admin",
      "role": "admin"
    }
  }
}
```

---

## 通用规范

### 请求格式

- Content-Type: `application/json`
- 所有时间使用 ISO 8601 格式: `2025-12-21T10:30:00Z`
- 所有金额使用字符串表示，避免精度丢失

### 响应格式

**成功响应**:
```json
{
  "success": true,
  "data": { /* 实际数据 */ },
  "timestamp": "2025-12-21T10:30:00Z"
}
```

**错误响应**:
```json
{
  "success": false,
  "error": {
    "code": "INVALID_PARAMETER",
    "message": "Invalid token address",
    "details": "Token address must be a valid Solana pubkey"
  },
  "timestamp": "2025-12-21T10:30:00Z"
}
```

### 分页

**请求参数**:
```
?page=1&limit=20&sort_by=created_at&order=desc
```

**响应**:
```json
{
  "success": true,
  "data": {
    "items": [ /* 数据列表 */ ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8
    }
  }
}
```

---

## 代币管理 API

### 1. 获取代币列表

**端点**: `GET /api/v1/tokens`

**查询参数**:
| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| page | integer | 否 | 页码（默认 1） |
| limit | integer | 否 | 每页数量（默认 20，最大 100） |
| min_liquidity | float | 否 | 最小流动性（SOL） |
| max_age_hours | float | 否 | 最大年龄（小时） |
| min_risk_score | float | 否 | 最小风险评分 |
| sort_by | string | 否 | 排序字段（created_at, liquidity_sol, risk_score） |
| order | string | 否 | 排序方向（asc, desc） |

**响应示例**:
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
        "symbol": "BONK",
        "name": "Bonk",
        "decimals": 9,
        "liquidity_sol": 125.5,
        "liquidity_usd": 18825.0,
        "holders_count": 1250,
        "age_hours": 2.5,
        "price_usd": "0.0000125",
        "price_change_1h": 15.5,
        "volume_1h": 5500.0,
        "risk_score": 82.5,
        "created_at": "2025-12-21T08:00:00Z",
        "updated_at": "2025-12-21T10:30:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8
    }
  }
}
```

### 2. 获取单个代币详情

**端点**: `GET /api/v1/tokens/:mint`

**路径参数**:
- `mint`: 代币 mint 地址

**响应示例**:
```json
{
  "success": true,
  "data": {
    "token_info": {
      "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
      "symbol": "BONK",
      "name": "Bonk",
      // ... 完整的 TokenInfo 字段
    },
    "risk_analysis": {
      "total": 82.5,
      "breakdown": {
        "contract": { "value": 95.0, "issues": [] },
        "liquidity": { "value": 85.0, "issues": [] },
        "holder": { "value": 78.0, "issues": ["Top 10 holds 45%"] },
        "sentiment": { "value": 80.0, "issues": [] },
        "similarity": { "value": 75.0, "issues": [] },
        "behavior": { "value": 82.0, "issues": [] }
      },
      "confidence": 0.85,
      "recommendation": "Buy"
    },
    "strategy_matches": [
      {
        "strategy_name": "liquidity_hunter",
        "position_size": 8.5,
        "expected_profit": 45.2,
        "risk_reward_ratio": 2.8,
        "confidence": 0.82
      }
    ]
  }
}
```

### 3. 刷新代币数据

**端点**: `POST /api/v1/tokens/:mint/refresh`

**响应**:
```json
{
  "success": true,
  "data": {
    "message": "Token data refresh initiated",
    "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "refresh_id": "refresh_abc123"
  }
}
```

### 4. 批量分析代币

**端点**: `POST /api/v1/tokens/batch-analyze`

**请求体**:
```json
{
  "mints": [
    "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  ]
}
```

**响应**:
```json
{
  "success": true,
  "data": {
    "job_id": "batch_xyz789",
    "status": "processing",
    "total": 2,
    "processed": 0
  }
}
```

---

## 策略管理 API

### 1. 获取策略列表

**端点**: `GET /api/v1/strategies`

**响应示例**:
```json
{
  "success": true,
  "data": {
    "strategies": [
      {
        "name": "early_bird",
        "enabled": true,
        "priority": 90,
        "min_confidence": 0.7,
        "min_risk_score": 75.0,
        "max_position_sol": 20.0,
        "stats": {
          "total_signals": 150,
          "executed_trades": 85,
          "win_rate": 0.58,
          "avg_return_pct": 42.5,
          "total_pnl": 125.5
        }
      },
      {
        "name": "liquidity_hunter",
        "enabled": true,
        "priority": 80,
        // ...
      }
    ]
  }
}
```

### 2. 更新策略配置

**端点**: `PUT /api/v1/strategies/:name`

**请求体**:
```json
{
  "enabled": true,
  "priority": 85,
  "min_confidence": 0.75,
  "min_risk_score": 70.0,
  "max_position_sol": 25.0
}
```

**响应**:
```json
{
  "success": true,
  "data": {
    "message": "Strategy configuration updated",
    "strategy": {
      "name": "early_bird",
      "enabled": true,
      "priority": 85,
      // ...
    }
  }
}
```

### 3. 启用/禁用策略

**端点**: `POST /api/v1/strategies/:name/toggle`

**请求体**:
```json
{
  "enabled": false
}
```

### 4. 获取策略性能报告

**端点**: `GET /api/v1/strategies/:name/performance`

**查询参数**:
- `period`: 时间段（1d, 7d, 30d, all）

**响应示例**:
```json
{
  "success": true,
  "data": {
    "strategy_name": "early_bird",
    "period": "7d",
    "metrics": {
      "total_trades": 45,
      "winning_trades": 28,
      "losing_trades": 17,
      "win_rate": 0.622,
      "avg_return": 38.5,
      "median_return": 32.0,
      "std_dev": 25.3,
      "best_return": 185.0,
      "worst_return": -22.0,
      "sharpe_ratio": 1.52,
      "profit_factor": 2.85,
      "total_pnl_sol": 85.5,
      "total_pnl_usd": 12825.0
    },
    "daily_breakdown": [
      {
        "date": "2025-12-21",
        "trades": 8,
        "pnl_sol": 12.5,
        "win_rate": 0.625
      }
    ]
  }
}
```

---

## 交易管理 API

### 1. 获取交易历史

**端点**: `GET /api/v1/trades`

**查询参数**:
| 参数 | 类型 | 说明 |
|------|------|------|
| page | integer | 页码 |
| limit | integer | 每页数量 |
| status | string | 状态筛选（pending, executed, failed, cancelled） |
| strategy | string | 策略筛选 |
| start_date | string | 开始日期 |
| end_date | string | 结束日期 |

**响应示例**:
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "trade_abc123",
        "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
        "symbol": "BONK",
        "strategy": "early_bird",
        "side": "buy",
        "amount_sol": 5.0,
        "amount_tokens": "500000000",
        "price_usd": "0.0000125",
        "status": "executed",
        "tx_signature": "5j7s...",
        "created_at": "2025-12-21T10:00:00Z",
        "executed_at": "2025-12-21T10:00:05Z",
        "position": {
          "entry_price": "0.0000125",
          "current_price": "0.0000145",
          "pnl_pct": 16.0,
          "pnl_sol": 0.8,
          "pnl_usd": 120.0,
          "holding_duration_secs": 1800
        }
      }
    ],
    "pagination": { /* ... */ }
  }
}
```

### 2. 创建交易（手动）

**端点**: `POST /api/v1/trades`

**请求体**:
```json
{
  "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
  "side": "buy",
  "amount_sol": 5.0,
  "strategy": "manual",
  "slippage_bps": 300,
  "priority_fee": 5000
}
```

**响应**:
```json
{
  "success": true,
  "data": {
    "trade_id": "trade_xyz789",
    "status": "pending",
    "estimated_execution_time": 5
  }
}
```

### 3. 获取交易详情

**端点**: `GET /api/v1/trades/:id`

**响应**:
```json
{
  "success": true,
  "data": {
    "id": "trade_abc123",
    // ... 完整交易信息
    "execution_details": {
      "tx_signature": "5j7s...",
      "slot": 123456789,
      "block_time": 1703145600,
      "fee_lamports": 5000,
      "compute_units_consumed": 150000,
      "logs": [ /* transaction logs */ ]
    }
  }
}
```

### 4. 关闭持仓

**端点**: `POST /api/v1/trades/:id/close`

**请求体**:
```json
{
  "exit_percentage": 1.0,
  "reason": "manual_exit"
}
```

### 5. 获取活跃持仓

**端点**: `GET /api/v1/positions`

**响应示例**:
```json
{
  "success": true,
  "data": {
    "positions": [
      {
        "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
        "symbol": "BONK",
        "entry_price": "0.0000125",
        "current_price": "0.0000145",
        "amount_sol": 5.0,
        "amount_tokens": "500000000",
        "pnl_pct": 16.0,
        "pnl_sol": 0.8,
        "unrealized_pnl_usd": 120.0,
        "entry_time": "2025-12-21T10:00:00Z",
        "holding_duration_secs": 1800,
        "exit_signals": [
          {
            "type": "trailing_stop",
            "should_exit": false,
            "distance_to_trigger": 5.5
          }
        ]
      }
    ],
    "summary": {
      "total_positions": 5,
      "total_value_sol": 25.0,
      "total_value_usd": 3750.0,
      "unrealized_pnl_sol": 2.5,
      "unrealized_pnl_usd": 375.0
    }
  }
}
```

---

## 风险控制 API

### 1. 获取风险配置

**端点**: `GET /api/v1/risk-control/config`

**响应**:
```json
{
  "success": true,
  "data": {
    "max_position_per_token": 20.0,
    "max_total_position": 100.0,
    "max_trades_per_day": 50,
    "max_trades_per_hour": 10,
    "max_daily_loss_sol": 10.0,
    "max_daily_loss_percentage": 0.05,
    "cooldown_after_loss_seconds": 300,
    "blacklisted_tokens": [],
    "blacklisted_creators": []
  }
}
```

### 2. 更新风险配置

**端点**: `PUT /api/v1/risk-control/config`

**请求体**:
```json
{
  "max_position_per_token": 25.0,
  "max_daily_loss_sol": 15.0
}
```

### 3. 添加黑名单

**端点**: `POST /api/v1/risk-control/blacklist`

**请求体**:
```json
{
  "type": "token",
  "address": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
  "reason": "Rug pull confirmed"
}
```

### 4. 获取风险统计

**端点**: `GET /api/v1/risk-control/stats`

**响应**:
```json
{
  "success": true,
  "data": {
    "daily_stats": {
      "date": "2025-12-21",
      "total_trades": 25,
      "total_pnl_sol": 5.5,
      "total_pnl_usd": 825.0,
      "largest_loss_sol": -2.0,
      "in_cooldown": false,
      "cooldown_ends_at": null
    },
    "current_exposure": {
      "total_position_sol": 45.0,
      "position_count": 8,
      "largest_position_sol": 12.0
    },
    "limits_status": {
      "position_limit_usage": 0.45,
      "daily_trade_limit_usage": 0.50,
      "loss_limit_usage": 0.55
    }
  }
}
```

---

## 监控指标 API

### 1. 获取系统健康状态

**端点**: `GET /api/v1/metrics/health`

**响应**:
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "uptime_secs": 3600,
    "components": {
      "api_server": { "status": "up", "latency_ms": 5 },
      "data_collector": { "status": "up", "events_per_min": 25 },
      "trading_engine": { "status": "up", "pending_trades": 2 },
      "rpc_manager": { "status": "up", "healthy_endpoints": 3 },
      "cache": { "status": "up", "hit_rate": 0.85 },
      "database": { "status": "up", "connections": 5 }
    }
  }
}
```

### 2. 获取交易指标

**端点**: `GET /api/v1/metrics/trading`

**响应**:
```json
{
  "success": true,
  "data": {
    "total_trades": 250,
    "successful_trades": 165,
    "failed_trades": 85,
    "win_rate": 0.66,
    "total_pnl_sol": 125.5,
    "total_pnl_usd": 18825.0,
    "avg_win": 2.5,
    "avg_loss": -1.2,
    "max_win": 15.0,
    "max_loss": -5.0,
    "profit_factor": 2.08,
    "sharpe_ratio": 1.65,
    "max_drawdown": 0.15,
    "avg_holding_duration_secs": 3600
  }
}
```

### 3. 获取策略指标

**端点**: `GET /api/v1/metrics/strategies`

**响应**:
```json
{
  "success": true,
  "data": {
    "strategies": {
      "early_bird": {
        "total_signals": 150,
        "executed_trades": 85,
        "win_rate": 0.58,
        "avg_return_pct": 42.5,
        "total_pnl": 65.5
      },
      "liquidity_hunter": {
        "total_signals": 200,
        "executed_trades": 120,
        "win_rate": 0.65,
        "avg_return_pct": 35.0,
        "total_pnl": 80.0
      }
    }
  }
}
```

### 4. 获取 RPC 端点指标

**端点**: `GET /api/v1/metrics/rpc`

**响应**:
```json
{
  "success": true,
  "data": {
    "endpoints": {
      "https://api.mainnet-beta.solana.com": {
        "total_requests": 5000,
        "successful_requests": 4800,
        "failed_requests": 200,
        "success_rate": 0.96,
        "avg_latency_ms": 250.5,
        "p95_latency_ms": 450.0,
        "p99_latency_ms": 800.0,
        "is_healthy": true
      }
    }
  }
}
```

### 5. 导出指标（Prometheus 格式）

**端点**: `GET /api/v1/metrics/export`

**响应**:
```
# HELP solsniper_total_trades Total number of trades
# TYPE solsniper_total_trades counter
solsniper_total_trades 250

# HELP solsniper_win_rate Trading win rate
# TYPE solsniper_win_rate gauge
solsniper_win_rate 0.66

# HELP solsniper_total_pnl_sol Total PnL in SOL
# TYPE solsniper_total_pnl_sol gauge
solsniper_total_pnl_sol 125.5
```

---

## 系统配置 API

### 1. 获取系统配置

**端点**: `GET /api/v1/config`

**响应**:
```json
{
  "success": true,
  "data": {
    "environment": "production",
    "version": "2.0.0",
    "enable_trading": true,
    "max_concurrent_trades": 5,
    "rpc_endpoints": [
      "https://api.mainnet-beta.solana.com",
      "https://rpc.helius.xyz"
    ],
    "position_sizing": {
      "strategy": "KellyCriterion",
      "kelly_fraction": 0.25
    },
    "exit_strategy": {
      "stop_loss_pct": 20.0,
      "take_profit_pct": 50.0
    }
  }
}
```

### 2. 更新配置

**端点**: `PUT /api/v1/config`

**请求体**:
```json
{
  "enable_trading": false,
  "max_concurrent_trades": 3
}
```

### 3. 重新加载配置

**端点**: `POST /api/v1/config/reload`

**响应**:
```json
{
  "success": true,
  "data": {
    "message": "Configuration reloaded successfully",
    "old_version": "v12abc",
    "new_version": "v34def"
  }
}
```

---

## WebSocket API

### 连接

**URL**: `ws://localhost:3000/ws`

**认证**: 在连接时通过查询参数传递 token
```
ws://localhost:3000/ws?token=<your_jwt_token>
```

### 订阅主题

**订阅消息**:
```json
{
  "type": "subscribe",
  "topics": ["tokens", "trades", "metrics"]
}
```

**取消订阅**:
```json
{
  "type": "unsubscribe",
  "topics": ["tokens"]
}
```

### 事件类型

#### 1. 新代币事件

**主题**: `tokens`

**消息**:
```json
{
  "type": "event",
  "topic": "tokens",
  "event": "new_token",
  "data": {
    "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "symbol": "BONK",
    "liquidity_sol": 15.0,
    "age_seconds": 60
  },
  "timestamp": "2025-12-21T10:30:00Z"
}
```

#### 2. 交易更新事件

**主题**: `trades`

**消息**:
```json
{
  "type": "event",
  "topic": "trades",
  "event": "trade_executed",
  "data": {
    "trade_id": "trade_abc123",
    "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "side": "buy",
    "amount_sol": 5.0,
    "status": "executed",
    "tx_signature": "5j7s..."
  },
  "timestamp": "2025-12-21T10:30:05Z"
}
```

#### 3. 持仓更新事件

**主题**: `positions`

**消息**:
```json
{
  "type": "event",
  "topic": "positions",
  "event": "position_update",
  "data": {
    "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "pnl_pct": 16.5,
    "current_price": "0.0000145"
  },
  "timestamp": "2025-12-21T10:30:10Z"
}
```

#### 4. 退出信号事件

**主题**: `signals`

**消息**:
```json
{
  "type": "event",
  "topic": "signals",
  "event": "exit_signal",
  "data": {
    "mint": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    "signal_type": "trailing_stop",
    "should_exit": true,
    "exit_percentage": 1.0,
    "reason": "Trailing stop triggered: 10% drawdown from high"
  },
  "timestamp": "2025-12-21T10:30:15Z"
}
```

#### 5. 指标更新事件

**主题**: `metrics`

**消息**:
```json
{
  "type": "event",
  "topic": "metrics",
  "event": "metrics_update",
  "data": {
    "total_trades": 251,
    "win_rate": 0.66,
    "total_pnl_sol": 126.0
  },
  "timestamp": "2025-12-21T10:30:20Z"
}
```

### Ping/Pong

服务器每 30 秒发送 ping：
```json
{
  "type": "ping"
}
```

客户端应回复 pong：
```json
{
  "type": "pong"
}
```

---

## 错误码

| 错误码 | HTTP状态 | 说明 |
|--------|---------|------|
| `SUCCESS` | 200 | 成功 |
| `INVALID_REQUEST` | 400 | 请求格式错误 |
| `INVALID_PARAMETER` | 400 | 参数无效 |
| `UNAUTHORIZED` | 401 | 未授权 |
| `FORBIDDEN` | 403 | 禁止访问 |
| `NOT_FOUND` | 404 | 资源不存在 |
| `RATE_LIMIT_EXCEEDED` | 429 | 请求频率超限 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |
| `SERVICE_UNAVAILABLE` | 503 | 服务不可用 |
| `TOKEN_NOT_FOUND` | 404 | 代币不存在 |
| `TRADE_FAILED` | 500 | 交易执行失败 |
| `INSUFFICIENT_BALANCE` | 400 | 余额不足 |
| `POSITION_LIMIT_EXCEEDED` | 400 | 持仓限制超限 |
| `RISK_CHECK_FAILED` | 403 | 风险检查未通过 |
| `BLACKLISTED` | 403 | 在黑名单中 |

---

## Rate Limiting

### 限制规则

| 端点类型 | 限制 |
|---------|------|
| 读取 API | 100 请求/分钟 |
| 写入 API | 20 请求/分钟 |
| WebSocket | 1000 消息/分钟 |

### 响应头

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1703145660
```

### 超限响应

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "API rate limit exceeded",
    "details": "Retry after 60 seconds",
    "retry_after": 60
  }
}
```

---

## SDK 示例

### JavaScript/TypeScript

```typescript
import axios from 'axios';

const client = axios.create({
  baseURL: 'http://localhost:3000/api/v1',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  }
});

// 获取代币列表
const tokens = await client.get('/tokens', {
  params: {
    min_liquidity: 50,
    min_risk_score: 70,
    limit: 20
  }
});

// 创建交易
const trade = await client.post('/trades', {
  mint: '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU',
  side: 'buy',
  amount_sol: 5.0
});

// WebSocket 连接
const ws = new WebSocket(`ws://localhost:3000/ws?token=${token}`);

ws.on('open', () => {
  ws.send(JSON.stringify({
    type: 'subscribe',
    topics: ['tokens', 'trades']
  }));
});

ws.on('message', (data) => {
  const message = JSON.parse(data);
  console.log('Received:', message);
});
```

---

**API 版本**: v1.0.0
**最后更新**: 2025-12-21
