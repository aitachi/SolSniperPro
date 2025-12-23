# Frontend Hooks Fix Script

## 需要修复的Hooks文件

由于API端点已被注释,以下hooks需要相应调整:

### 1. useStrategies.ts - 需要修复的调用:
- createStrategy - 添加type字段
- updateStrategy - 注释掉(未实现)
- deleteStrategy - 注释掉(未实现)
- updatePriority - 注释掉(未实现)
- getPerformance - 注释掉(未实现)

### 2. useTokens.ts - 需要修复的调用:
- refreshToken - 注释掉(未实现)
- searchTokens - 注释掉(未实现)
- getTrending - 注释掉(未实现)

### 3. useTrades.ts - 需要修复的调用:
- simulateTrade - 注释掉(未实现)
- executeTrade - 注释掉(未实现)
- cancelTrade - 注释掉(未实现)

### 4. usePositions.ts - 需要修复的调用:
- getPositionHistory - 注释掉(未实现)

---

## 修复方法

对于每个未实现的功能,有两个选择:

1. **注释掉整个hook函数** (推荐) - 防止UI调用
2. **返回空数据/禁用状态** - 保持UI兼容但功能不可用

由于时间关系,我建议采用方式1:直接注释掉hook,让TypeScript编译器提醒哪些组件在使用这些功能,然后在UI中相应禁用或隐藏。
