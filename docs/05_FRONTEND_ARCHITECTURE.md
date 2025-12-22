# SolSniper Pro - 前端架构文档

---

**Author**: Aitachi  
**Email**: 44158892@qq.com  
**Wechat**: 18116011230

---

**版本**: v2.0.0
**日期**: 2025-12-21
**技术栈**: React 18 + TypeScript + Vite + TailwindCSS
## 目录结构

```
frontend/
├── public/                    # 静态资源
│   ├── logo.svg
│   └── favicon.ico
│
├── src/
│   ├── assets/                # 资源文件
│   │   ├── images/
│   │   └── icons/
│   │
│   ├── components/            # 组件
│   │   ├── common/            # 通用组件
│   │   │   ├── Button.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Table.tsx
│   │   │   ├── Modal.tsx
│   │   │   ├── Loading.tsx
│   │   │   ├── Badge.tsx
│   │   │   └── Input.tsx
│   │   │
│   │   ├── layout/            # 布局组件
│   │   │   ├── Header.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Footer.tsx
│   │   │   └── Layout.tsx
│   │   │
│   │   ├── dashboard/         # 仪表板组件
│   │   │   ├── Overview.tsx
│   │   │   ├── MetricsCards.tsx
│   │   │   ├── RecentTrades.tsx
│   │   │   ├── ActivePositions.tsx
│   │   │   └── PerformanceChart.tsx
│   │   │
│   │   ├── tokens/            # 代币相关组件
│   │   │   ├── TokenList.tsx
│   │   │   ├── TokenCard.tsx
│   │   │   ├── TokenDetail.tsx
│   │   │   ├── TokenFilters.tsx
│   │   │   └── RiskScoreBar.tsx
│   │   │
│   │   ├── strategies/        # 策略相关组件
│   │   │   ├── StrategyList.tsx
│   │   │   ├── StrategyCard.tsx
│   │   │   ├── StrategyConfig.tsx
│   │   │   ├── PositionSizing.tsx
│   │   │   ├── ExitStrategy.tsx
│   │   │   └── StrategyPriority.tsx
│   │   │
│   │   ├── trading/           # 交易相关组件
│   │   │   ├── TradeHistory.tsx
│   │   │   ├── TradeForm.tsx
│   │   │   ├── PositionTable.tsx
│   │   │   └── OrderBook.tsx
│   │   │
│   │   ├── risk/              # 风险控制组件
│   │   │   ├── RiskSettings.tsx
│   │   │   ├── BlacklistManager.tsx
│   │   │   ├── RiskStats.tsx
│   │   │   └── LimitGauges.tsx
│   │   │
│   │   └── analytics/         # 分析组件
│   │       ├── PerformanceMetrics.tsx
│   │       ├── ProfitChart.tsx
│   │       ├── StrategyComparison.tsx
│   │       └── HeatMap.tsx
│   │
│   ├── pages/                 # 页面
│   │   ├── Dashboard.tsx      # 主仪表板
│   │   ├── Tokens.tsx         # 代币列表页
│   │   ├── Strategies.tsx     # 策略管理页
│   │   ├── Trading.tsx        # 交易页
│   │   ├── Positions.tsx      # 持仓页
│   │   ├── RiskControl.tsx    # 风险控制页
│   │   ├── Analytics.tsx      # 分析页
│   │   ├── Settings.tsx       # 设置页
│   │   └── Login.tsx          # 登录页
│   │
│   ├── api/                   # API 客户端
│   │   ├── client.ts          # Axios 实例
│   │   ├── tokens.ts          # 代币 API
│   │   ├── strategies.ts      # 策略 API
│   │   ├── trades.ts          # 交易 API
│   │   ├── risk.ts            # 风险控制 API
│   │   ├── metrics.ts         # 指标 API
│   │   └── auth.ts            # 认证 API
│   │
│   ├── hooks/                 # 自定义 Hooks
│   │   ├── useTokens.ts       # 代币数据 Hook
│   │   ├── useStrategies.ts   # 策略 Hook
│   │   ├── useTrades.ts       # 交易 Hook
│   │   ├── usePositions.ts    # 持仓 Hook
│   │   ├── useMetrics.ts      # 指标 Hook
│   │   ├── useWebSocket.ts    # WebSocket Hook
│   │   └── useAuth.ts         # 认证 Hook
│   │
│   ├── stores/                # Zustand 状态管理
│   │   ├── authStore.ts       # 认证状态
│   │   ├── tokenStore.ts      # 代币状态
│   │   ├── strategyStore.ts   # 策略状态
│   │   ├── tradeStore.ts      # 交易状态
│   │   └── uiStore.ts         # UI 状态
│   │
│   ├── types/                 # 类型定义
│   │   ├── token.ts
│   │   ├── strategy.ts
│   │   ├── trade.ts
│   │   ├── risk.ts
│   │   ├── metrics.ts
│   │   └── api.ts
│   │
│   ├── utils/                 # 工具函数
│   │   ├── format.ts          # 格式化函数
│   │   ├── validation.ts      # 验证函数
│   │   ├── constants.ts       # 常量
│   │   └── helpers.ts         # 辅助函数
│   │
│   ├── styles/                # 样式
│   │   ├── index.css          # 全局样式
│   │   └── variables.css      # CSS 变量
│   │
│   ├── App.tsx                # 主应用组件
│   ├── main.tsx               # 入口文件
│   └── router.tsx             # 路由配置
│
├── index.html                 # HTML 模板
├── package.json               # 依赖配置
├── vite.config.ts             # Vite 配置
├── tailwind.config.js         # Tailwind 配置
├── tsconfig.json              # TypeScript 配置
└── README.md                  # 说明文档
```
## 核心功能模块

### 1. 仪表板模块 (Dashboard)

**功能**:
- 实时交易统计卡片
- 活跃持仓概览
- 最近交易列表
- 性能图表（盈亏曲线）
- 策略表现对比
- 系统健康状态

**关键组件**:
```tsx
// src/pages/Dashboard.tsx
import { MetricsCards } from '@/components/dashboard/MetricsCards'
import { ActivePositions } from '@/components/dashboard/ActivePositions'
import { RecentTrades } from '@/components/dashboard/RecentTrades'
import { PerformanceChart } from '@/components/dashboard/PerformanceChart'

export const Dashboard = () => {
  const { data: metrics } = useMetrics()
  const { data: positions } = usePositions()
  const { data: trades } = useTrades({ limit: 10 })

  return (
    <div className="space-y-6">
      <MetricsCards metrics={metrics} />
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <ActivePositions positions={positions} />
        <RecentTrades trades={trades} />
      </div>
      <PerformanceChart />
    </div>
  )
}
```

### 2. 代币监控模块 (Tokens)

**功能**:
- 实时代币列表（WebSocket 更新）
- 高级筛选（流动性、风险评分、年龄）
- 代币详情查看
- 风险分析可视化
- 策略匹配显示
- 一键交易

**关键组件**:
```tsx
// src/pages/Tokens.tsx
import { useState } from 'react'
import { TokenList } from '@/components/tokens/TokenList'
import { TokenFilters } from '@/components/tokens/TokenFilters'
import { TokenDetail } from '@/components/tokens/TokenDetail'
import { useTokens } from '@/hooks/useTokens'

export const Tokens = () => {
  const [filters, setFilters] = useState({
    min_liquidity: 50,
    min_risk_score: 70,
    max_age_hours: 24
  })

  const { data, loading } = useTokens(filters)
  const [selectedToken, setSelectedToken] = useState(null)

  return (
    <div className="flex gap-6">
      <div className="flex-1">
        <TokenFilters filters={filters} onChange={setFilters} />
        <TokenList
          tokens={data}
          loading={loading}
          onSelect={setSelectedToken}
        />
      </div>
      {selectedToken && (
        <div className="w-96">
          <TokenDetail token={selectedToken} />
        </div>
      )}
    </div>
  )
}
```

### 3. 策略管理模块 (Strategies)

**功能**:
- 策略列表展示
- 策略启用/禁用
- 策略配置编辑（参数调整）
- 优先级管理
- 性能统计
- 仓位管理配置
- 退出策略配置

**关键组件**:
```tsx
// src/pages/Strategies.tsx
import { StrategyList } from '@/components/strategies/StrategyList'
import { StrategyConfig } from '@/components/strategies/StrategyConfig'
import { PositionSizing } from '@/components/strategies/PositionSizing'
import { ExitStrategy } from '@/components/strategies/ExitStrategy'

export const Strategies = () => {
  const { data: strategies, update } = useStrategies()

  return (
    <div className="space-y-8">
      {/* 策略列表和配置 */}
      <section>
        <h2>交易策略</h2>
        <StrategyList strategies={strategies} onUpdate={update} />
      </section>

      {/* 仓位管理 */}
      <section>
        <h2>仓位管理</h2>
        <PositionSizing />
      </section>

      {/* 退出策略 */}
      <section>
        <h2>退出策略</h2>
        <ExitStrategy />
      </section>
    </div>
  )
}
```

### 4. 交易管理模块 (Trading)

**功能**:
- 交易历史列表
- 交易详情查看
- 手动下单
- 持仓管理
- 平仓操作
- 交易状态实时更新

**关键组件**:
```tsx
// src/pages/Trading.tsx
import { TradeHistory } from '@/components/trading/TradeHistory'
import { TradeForm } from '@/components/trading/TradeForm'
import { PositionTable } from '@/components/trading/PositionTable'

export const Trading = () => {
  const { data: trades } = useTrades()
  const { data: positions } = usePositions()

  return (
    <div className="grid grid-cols-1 xl:grid-cols-3 gap-6">
      <div className="xl:col-span-2">
        <PositionTable positions={positions} />
        <TradeHistory trades={trades} />
      </div>
      <div>
        <TradeForm />
      </div>
    </div>
  )
}
```

### 5. 风险控制模块 (RiskControl)

**功能**:
- 风险配置设置
- 黑名单管理
- 风险统计展示
- 限制使用率仪表盘
- 冷却期状态

**关键组件**:
```tsx
// src/pages/RiskControl.tsx
import { RiskSettings } from '@/components/risk/RiskSettings'
import { BlacklistManager } from '@/components/risk/BlacklistManager'
import { RiskStats } from '@/components/risk/RiskStats'

export const RiskControl = () => {
  const { data: config, update } = useRiskConfig()
  const { data: stats } = useRiskStats()

  return (
    <div className="space-y-6">
      <RiskStats stats={stats} />
      <RiskSettings config={config} onUpdate={update} />
      <BlacklistManager />
    </div>
  )
}
```

### 6. 数据分析模块 (Analytics)

**功能**:
- 性能指标总览
- 盈亏图表（日/周/月）
- 策略对比分析
- 热力图（收益分布）
- 交易时段分析
- 风险调整后收益
## WebSocket 实时更新

### useWebSocket Hook

```typescript
// src/hooks/useWebSocket.ts
import { useEffect, useRef, useState } from 'react'
import { useAuthStore } from '@/stores/authStore'

interface WebSocketMessage {
  type: 'event'
  topic: string
  event: string
  data: any
  timestamp: string
}

export const useWebSocket = (topics: string[]) => {
  const [isConnected, setIsConnected] = useState(false)
  const [messages, setMessages] = useState<WebSocketMessage[]>([])
  const ws = useRef<WebSocket | null>(null)
  const { token } = useAuthStore()

  useEffect(() => {
    if (!token) return

    // 连接 WebSocket
    const wsUrl = `ws://localhost:3000/ws?token=${token}`
    ws.current = new WebSocket(wsUrl)

    ws.current.onopen = () => {
      setIsConnected(true)

      // 订阅主题
      ws.current?.send(JSON.stringify({
        type: 'subscribe',
        topics
      }))
    }

    ws.current.onmessage = (event) => {
      const message = JSON.parse(event.data)

      if (message.type === 'event') {
        setMessages(prev => [...prev, message])
      }
    }

    ws.current.onclose = () => {
      setIsConnected(false)
    }

    return () => {
      ws.current?.close()
    }
  }, [token, topics])

  return { isConnected, messages }
}
```

### 使用示例

```tsx
// 在组件中使用
const TokenList = () => {
  const { messages } = useWebSocket(['tokens', 'trades'])

  useEffect(() => {
    messages.forEach(msg => {
      if (msg.event === 'new_token') {
        // 添加新代币到列表
        console.log('New token:', msg.data)
      }
    })
  }, [messages])
}
```
## API 客户端

### Axios 配置

```typescript
// src/api/client.ts
import axios from 'axios'
import { useAuthStore } from '@/stores/authStore'

const client = axios.create({
  baseURL: '/api/v1',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器：添加 Token
client.interceptors.request.use((config) => {
  const { token } = useAuthStore.getState()
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// 响应拦截器：处理错误
client.interceptors.response.use(
  (response) => response.data,
  (error) => {
    if (error.response?.status === 401) {
      // Token 过期，跳转登录
      useAuthStore.getState().logout()
    }
    return Promise.reject(error)
  }
)

export default client
```

### API 模块示例

```typescript
// src/api/tokens.ts
import client from './client'
import type { TokenInfo, TokenFilters } from '@/types/token'

export const tokensApi = {
  // 获取代币列表
  getTokens: async (filters: TokenFilters) => {
    return client.get<{ data: TokenInfo[] }>('/tokens', {
      params: filters
    })
  },

  // 获取单个代币
  getToken: async (mint: string) => {
    return client.get(`/tokens/${mint}`)
  },

  // 刷新代币数据
  refreshToken: async (mint: string) => {
    return client.post(`/tokens/${mint}/refresh`)
  },
}
```
## 状态管理 (Zustand)

### 认证状态

```typescript
// src/stores/authStore.ts
import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface AuthState {
  token: string | null
  user: User | null
  login: (username: string, password: string) => Promise<void>
  logout: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      user: null,

      login: async (username, password) => {
        const response = await authApi.login({ username, password })
        set({ token: response.token, user: response.user })
      },

      logout: () => {
        set({ token: null, user: null })
      },
    }),
    {
      name: 'auth-storage',
    }
  )
)
```
## 样式系统

### 全局样式

```css
/* src/styles/index.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-dark-950 text-gray-100;
  }

  h1 {
    @apply text-4xl font-bold;
  }

  h2 {
    @apply text-2xl font-semibold;
  }
}

@layer components {
  .btn {
    @apply px-4 py-2 rounded-lg font-medium transition-colors;
  }

  .btn-primary {
    @apply bg-primary-600 hover:bg-primary-700 text-white;
  }

  .card {
    @apply bg-dark-800 rounded-xl p-6 shadow-lg;
  }

  .badge-success {
    @apply bg-success-500/20 text-success-500 px-2 py-1 rounded-md text-xs font-medium;
  }

  .badge-danger {
    @apply bg-danger-500/20 text-danger-500 px-2 py-1 rounded-md text-xs font-medium;
  }
}
```
## 部署说明

### 开发模式

```bash
cd frontend
npm install
npm run dev
```

访问: http://localhost:5173

### 生产构建

```bash
npm run build
```

构建产物在 `dist/` 目录

### Nginx 配置

```nginx
server {
    listen 80;
    server_name your-domain.com;

    root /var/www/solsniper-pro/frontend/dist;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    location /ws {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }
}
```
**文档版本**: v2.0.0
**最后更新**: 2025-12-21