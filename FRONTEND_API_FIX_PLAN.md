# Frontend API 修正方案

## API端点对照表 (Frontend期望 vs Backend实际实现)

### ✅ 已实现且匹配的端点

| Frontend调用 | Backend端点 | 状态 |
|-------------|------------|------|
| authApi.login() | POST /api/v1/auth/login | ✅ |
| tokensApi.getTokens() | GET /api/v1/tokens | ✅ |
| strategiesApi.getStrategies() | GET /api/v1/strategies | ✅ |
| tradesApi.getTrades() | GET /api/v1/trades | ✅ |
| tradesApi.getPositions() | GET /api/v1/positions | ✅ |
| metricsApi.getSummary() | GET /api/v1/metrics/summary | ✅ |
| metricsApi.getSystemHealth() | GET /api/v1/metrics/system | ✅ |

### ⚠️ 需要修正的端点

| Frontend调用 | 期望路径 | 实际Backend路径 | 修正操作 |
|-------------|---------|----------------|----------|
| authApi.logout() | /auth/logout | ✅ POST /api/v1/auth/logout | 保留(Backend已实现) |
| tokensApi.getToken() | /tokens/:mint | ✅ GET /api/v1/tokens/:mint | 保留(Backend已实现) |
| tradesApi.getTrade() | /trades/:id | ✅ GET /api/v1/trades/:id | 保留(Backend已实现) |
| tradesApi.getPosition() | /positions/:id | ✅ GET /api/v1/positions/:id | 保留(Backend已实现) |
| tradesApi.closePosition() | /positions/:id/close | ✅ POST /api/v1/positions/:id/close | 保留(Backend已实现) |
| riskApi.getConfig() | /risk/config | /risk/limits | **修改路径** |
| riskApi.updateConfig() | /risk/config | /risk/limits | **修改路径** |
| riskApi.getStats() | /risk/stats | /risk/status | **修改路径** |
| metricsApi.getStrategyMetrics() | /metrics/strategies | /metrics/strategy/:id | **修改路径和参数** |
| strategiesApi.toggleStrategy() | /strategies/:id/toggle | /strategies/:id/start 或 pause | **修改逻辑** |

### ❌ Backend未实现的端点 (需要禁用或添加fallback)

| Frontend调用 | 期望路径 | 处理方式 |
|-------------|---------|---------|
| authApi.refresh() | /auth/refresh | **注释掉或返回模拟数据** |
| authApi.verify() | /auth/verify | **注释掉或返回模拟数据** |
| tokensApi.getRiskScore() | /tokens/:mint/risk | **注释掉或从getToken中获取** |
| tokensApi.refreshToken() | /tokens/:mint/refresh | **注释掉** |
| tokensApi.searchTokens() | /tokens/search | **注释掉** |
| tokensApi.getTrending() | /tokens/trending | **注释掉** |
| strategiesApi.getStrategy() | /strategies/:id | **注释掉或从list中查找** |
| strategiesApi.createStrategy() | /strategies | ✅ Backend已实现! |
| strategiesApi.updateStrategy() | /strategies/:id | **注释掉** |
| strategiesApi.deleteStrategy() | /strategies/:id | **注释掉** |
| strategiesApi.updatePriority() | /strategies/:id/priority | **注释掉** |
| strategiesApi.getStats() | /strategies/:id/stats | **使用/metrics/strategy/:id** |
| strategiesApi.getPerformance() | /strategies/:id/performance | **注释掉** |
| strategiesApi.backtest() | /strategies/backtest | **注释掉** |
| tradesApi.simulateTrade() | /trades/simulate | **注释掉** |
| tradesApi.executeTrade() | /trades | **注释掉** |
| tradesApi.cancelTrade() | /trades/:id/cancel | **注释掉** |
| tradesApi.getPositionHistory() | /positions/history | **注释掉** |
| tradesApi.getTradeStats() | /trades/stats | **使用/metrics/summary** |
| metricsApi.getTradingMetrics() | /metrics/trading | **注释掉** |
| metricsApi.getRpcMetrics() | /metrics/rpc | **注释掉** |
| metricsApi.getPerformance() | /metrics/performance | **注释掉** |
| metricsApi.getHeatMap() | /metrics/heatmap | **注释掉** |
| metricsApi.getStrategyComparison() | /metrics/strategy-comparison | **注释掉** |
| metricsApi.exportMetrics() | /metrics/export | **注释掉** |
| riskApi.getBlacklist() | /risk/blacklist | **注释掉** |
| riskApi.addToBlacklist() | /risk/blacklist | **注释掉** |
| riskApi.removeFromBlacklist() | /risk/blacklist/:address | **注释掉** |
| riskApi.getAlerts() | /risk/alerts | **注释掉** |
| riskApi.acknowledgeAlert() | /risk/alerts/:id/acknowledge | **注释掉** |
| riskApi.clearCooldown() | /risk/cooldown/clear | **注释掉** |
| riskApi.validateTrade() | /risk/validate | **注释掉** |

---

## 修正策略

### 1. 立即修正 (保持现有功能)
- 修正路径错误的端点
- 禁用未实现的功能(注释或返回空数据)
- 保证不会因为调用不存在的端点而crash

### 2. 渐进增强 (后续扩展)
- 在Rust backend中逐步实现缺失的高级功能
- 或者在frontend中使用现有端点组合实现相同效果

---

## Backend实际实现的21个端点

根据我们的Rust Backend实现 (crates/api-server):

1. ✅ GET  /api/v1/health
2. ✅ POST /api/v1/auth/login
3. ✅ POST /api/v1/auth/logout (Mock缺失,Rust已实现)
4. ✅ GET  /api/v1/tokens
5. ✅ GET  /api/v1/tokens/:mint (Mock缺失,Rust已实现)
6. ✅ GET  /api/v1/strategies
7. ✅ POST /api/v1/strategies (Mock缺失,Rust已实现)
8. ✅ POST /api/v1/strategies/:id/start
9. ✅ POST /api/v1/strategies/:id/pause
10. ✅ GET  /api/v1/positions
11. ✅ GET  /api/v1/positions/:id (Mock缺失,Rust已实现)
12. ✅ POST /api/v1/positions/:id/close
13. ✅ GET  /api/v1/trades
14. ✅ GET  /api/v1/trades/:id (Mock缺失,Rust已实现)
15. ✅ GET  /api/v1/metrics/summary
16. ✅ GET  /api/v1/metrics/system
17. ✅ GET  /api/v1/metrics/strategy/:id (Mock缺失,Rust已实现)
18. ✅ GET  /api/v1/risk/limits
19. ✅ PUT  /api/v1/risk/limits
20. ✅ GET  /api/v1/risk/status (Mock缺失,Rust已实现)
21. ✅ WS   /ws

---

## 修正文件清单

需要修改的文件:
1. ✅ src/api/auth.ts - 移除refresh/verify,保留login/logout
2. ✅ src/api/tokens.ts - 简化为基础功能
3. ✅ src/api/strategies.ts - 简化CRUD操作
4. ✅ src/api/trades.ts - 移除未实现的功能
5. ✅ src/api/metrics.ts - 修正路径
6. ✅ src/api/risk.ts - 修正路径(/config → /limits)
7. ⚠️ 相关hooks可能需要相应调整

---

**修正原则**:
- 优先保证现有核心功能正常工作
- 不破坏现有UI组件
- 对未实现功能优雅降级(返回空数据或显示"功能开发中")
