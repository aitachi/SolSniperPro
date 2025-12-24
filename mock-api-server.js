// Mock API Server for SolSniper Pro Frontend Testing
// Author: Aitachi
// Email: 44158892@qq.com
// Wechat: 18116011230

const express = require('express');
const cors = require('cors');
const { createServer } = require('http');
const { WebSocketServer } = require('ws');

const app = express();
const PORT = 3000;

app.use(cors());
app.use(express.json());

// Create HTTP server
const server = createServer(app);

// Create WebSocket server
const wss = new WebSocketServer({ server, path: '/ws' });

// WebSocket connection handling
wss.on('connection', (ws, req) => {
  const token = new URL(req.url, 'http://localhost').searchParams.get('token');
  console.log(`WebSocket client connected with token: ${token}`);

  // Send welcome message
  ws.send(JSON.stringify({
    type: 'connection',
    data: { status: 'connected', timestamp: Date.now() }
  }));

  // Handle messages from client
  ws.on('message', (message) => {
    try {
      const data = JSON.parse(message.toString());
      console.log('Received:', data);

      // Echo back or handle specific message types
      ws.send(JSON.stringify({
        type: 'echo',
        data: data
      }));
    } catch (error) {
      console.error('WebSocket message error:', error);
    }
  });

  // Handle client disconnect
  ws.on('close', () => {
    console.log('WebSocket client disconnected');
  });

  // Send periodic updates (mock real-time data)
  const interval = setInterval(() => {
    if (ws.readyState === ws.OPEN) {
      ws.send(JSON.stringify({
        type: 'price_update',
        data: {
          symbol: 'BONK',
          price: (Math.random() * 0.00005).toFixed(8),
          timestamp: Date.now()
        }
      }));
    }
  }, 5000);

  ws.on('close', () => {
    clearInterval(interval);
  });
});

console.log('WebSocket server initialized on /ws');

// Mock data
const mockTokens = [
  {
    id: 'token1',
    symbol: 'BONK',
    name: 'Bonk Token',
    mint: '5t1dC...abc123',
    liquidity: 150000,
    holders: 1500,
    price: 0.000025,
    priceChange1h: 12.5,
    age: 5,
    riskScore: 85,
    isRenounced: true,
    isImmutable: false,
  },
  {
    id: 'token2',
    symbol: 'WIF',
    name: 'dogwifhat',
    mint: '8x2yN...def456',
    liquidity: 250000,
    holders: 2500,
    price: 0.000045,
    priceChange1h: -5.3,
    age: 12,
    riskScore: 78,
    isRenounced: true,
    isImmutable: true,
  },
];

const mockStrategies = [
  {
    id: 'strategy1',
    name: 'Early Bird',
    type: 'EarlyBird',
    is_active: true,
    priority: 90,
    stats: {
      totalTrades: 156,
      winRate: 68.5,
      totalPnl: 245.67,
      sharpeRatio: 1.85,
    },
  },
  {
    id: 'strategy2',
    name: 'Liquidity Hunter',
    type: 'LiquidityHunter',
    is_active: true,
    priority: 80,
    stats: {
      totalTrades: 89,
      winRate: 72.3,
      totalPnl: 189.34,
      sharpeRatio: 2.12,
    },
  },
];

const mockPositions = [
  {
    id: 'pos1',
    token_symbol: 'BONK',
    token_mint: '5t1dC...abc123',
    strategy_name: 'Early Bird',
    entry_price_usd: 0.000020,
    current_price_usd: 0.000025,
    amount_sol: 2.5,
    invested_usd: 50,
    current_value_usd: 62.5,
    pnl_usd: 12.5,
    pnl_percentage: 25.0,
    holding_time: 7200000, // 2 hours in ms
    created_at: new Date(Date.now() - 7200000).toISOString(),
  },
  {
    id: 'pos2',
    token_symbol: 'WIF',
    token_mint: '8x2yN...def456',
    strategy_name: 'Liquidity Hunter',
    entry_price_usd: 0.000048,
    current_price_usd: 0.000045,
    amount_sol: 1.5,
    invested_usd: 75,
    current_value_usd: 70.7,
    pnl_usd: -4.3,
    pnl_percentage: -5.7,
    holding_time: 3600000, // 1 hour in ms
    created_at: new Date(Date.now() - 3600000).toISOString(),
  },
];

const mockTrades = [
  {
    id: 'trade1',
    created_at: new Date(Date.now() - 300000).toISOString(), // 5 minutes ago
    side: 'BUY',
    token_symbol: 'BONK',
    token_mint: '5t1dC...abc123',
    strategy_name: 'Early Bird',
    amount_usd: 50,
    amount_sol: 2.5,
    price_usd: 0.000020,
    status: 'COMPLETED',
    pnl_usd: null,
    tx_hash: '4kX...abc',
  },
  {
    id: 'trade2',
    created_at: new Date(Date.now() - 720000).toISOString(), // 12 minutes ago
    side: 'SELL',
    token_symbol: 'WIF',
    token_mint: '8x2yN...def456',
    strategy_name: 'Liquidity Hunter',
    amount_usd: 75,
    amount_sol: 1.5,
    price_usd: 0.000048,
    status: 'COMPLETED',
    pnl_usd: 15.5,
    tx_hash: '7yZ...def',
  },
  {
    id: 'trade3',
    created_at: new Date(Date.now() - 1800000).toISOString(), // 30 minutes ago
    side: 'BUY',
    token_symbol: 'SAMO',
    token_mint: '9aB...ghi',
    strategy_name: 'Volume Explosion',
    amount_usd: 100,
    amount_sol: 5.0,
    price_usd: 0.00012,
    status: 'COMPLETED',
    pnl_usd: null,
    tx_hash: '2mN...ghi',
  },
];

const mockMetrics = {
  totalPnl: 345.67,
  totalPnlPercent: 34.56,
  winRate: 70.5,
  activePositions: 1,
  totalPositionsValue: 62.5,
  totalTrades: 245,
  bestDay: 45.67,
  worstDay: -12.34,
  sharpeRatio: 1.95,
  maxDrawdown: 15.2,
};

// Tracking closed positions for idempotency
const closedPositions = new Set();

// Auth endpoints
app.post('/api/v1/auth/login', (req, res) => {
  const { username, password } = req.body;

  // Validation: Check required fields
  if (!username) {
    return res.status(400).json({
      success: false,
      error: {
        code: 'MISSING_FIELD',
        message: 'username field is required',
      },
    });
  }

  if (!password) {
    return res.status(400).json({
      success: false,
      error: {
        code: 'MISSING_FIELD',
        message: 'password field is required',
      },
    });
  }

  if (username === 'admin' && password === 'admin123') {
    res.json({
      success: true,
      data: {
        token: 'mock_jwt_token_12345',
        user: {
          id: 'user1',
          username: 'admin',
          role: 'ADMIN',
        },
      },
    });
  } else {
    res.status(401).json({
      success: false,
      error: {
        code: 'INVALID_CREDENTIALS',
        message: 'Invalid username or password',
      },
    });
  }
});

// NEW: Logout endpoint
app.post('/api/v1/auth/logout', (req, res) => {
  res.json({
    success: true,
    data: {
      message: 'Logged out successfully',
    },
  });
});

// Token endpoints
app.get('/api/v1/tokens', (req, res) => {
  res.json({
    success: true,
    data: mockTokens,
  });
});

// NEW: Get single token by mint address
app.get('/api/v1/tokens/:mint', (req, res) => {
  const token = mockTokens.find(t => t.mint === req.params.mint);
  if (!token) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Token not found',
      },
    });
  }
  res.json({
    success: true,
    data: token,
  });
});

// Strategy endpoints
app.get('/api/v1/strategies', (req, res) => {
  res.json({
    success: true,
    data: mockStrategies,
  });
});

// NEW: Create new strategy
app.post('/api/v1/strategies', (req, res) => {
  const { name, type, priority } = req.body;

  if (!name || !type) {
    return res.status(400).json({
      success: false,
      error: {
        code: 'MISSING_FIELD',
        message: 'name and type fields are required',
      },
    });
  }

  const newStrategy = {
    id: `strategy${mockStrategies.length + 1}`,
    name,
    type,
    is_active: false,
    priority: priority || 50,
    stats: {
      totalTrades: 0,
      winRate: 0,
      totalPnl: 0,
      sharpeRatio: 0,
    },
  };

  mockStrategies.push(newStrategy);

  res.json({
    success: true,
    data: newStrategy,
  });
});

app.post('/api/v1/strategies/:id/start', (req, res) => {
  // Validation: Check if strategy exists
  const strategy = mockStrategies.find(s => s.id === req.params.id);
  if (!strategy) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Strategy not found',
      },
    });
  }

  strategy.is_active = true;
  res.json({
    success: true,
    data: { message: 'Strategy started' },
  });
});

app.post('/api/v1/strategies/:id/pause', (req, res) => {
  // Validation: Check if strategy exists
  const strategy = mockStrategies.find(s => s.id === req.params.id);
  if (!strategy) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Strategy not found',
      },
    });
  }

  strategy.is_active = false;
  res.json({
    success: true,
    data: { message: 'Strategy paused' },
  });
});

// Position endpoints
app.get('/api/v1/positions', (req, res) => {
  res.json({
    success: true,
    data: mockPositions,
  });
});

// NEW: Get single position by ID
app.get('/api/v1/positions/:id', (req, res) => {
  const position = mockPositions.find(p => p.id === req.params.id);
  if (!position) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Position not found',
      },
    });
  }
  res.json({
    success: true,
    data: position,
  });
});

app.post('/api/v1/positions/:id/close', (req, res) => {
  // Validation: Check if position exists
  const position = mockPositions.find(p => p.id === req.params.id);
  if (!position) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Position not found',
      },
    });
  }

  // Idempotency check: Don't close same position twice
  if (closedPositions.has(req.params.id)) {
    return res.status(400).json({
      success: false,
      error: {
        code: 'ALREADY_CLOSED',
        message: 'Position already closed',
      },
    });
  }

  closedPositions.add(req.params.id);
  res.json({
    success: true,
    data: { message: 'Position closed' },
  });
});

// Trade endpoints
app.get('/api/v1/trades', (req, res) => {
  const page = parseInt(req.query.page) || 1;
  const limit = parseInt(req.query.limit) || 10;

  // Validation: Positive pagination values
  if (page < 1) {
    return res.status(400).json({
      success: false,
      error: {
        code: 'INVALID_PARAMETER',
        message: 'page must be >= 1',
      },
    });
  }

  if (limit < 1 || limit > 100) {
    return res.status(400).json({
      success: false,
      error: {
        code: 'INVALID_PARAMETER',
        message: 'limit must be between 1 and 100',
      },
    });
  }

  res.json({
    success: true,
    data: mockTrades.slice(0, limit),
    pagination: {
      total: mockTrades.length,
      page: page,
      limit: limit,
      totalPages: Math.ceil(mockTrades.length / limit),
    },
  });
});

// NEW: Get single trade by ID
app.get('/api/v1/trades/:id', (req, res) => {
  const trade = mockTrades.find(t => t.id === req.params.id);
  if (!trade) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Trade not found',
      },
    });
  }
  res.json({
    success: true,
    data: trade,
  });
});

// Metrics endpoints
app.get('/api/v1/metrics/summary', (req, res) => {
  res.json({
    success: true,
    data: {
      trading_metrics: {
        total_pnl_usd: mockMetrics.totalPnl,
        total_pnl_sol: mockMetrics.totalPnl / 100, // Mock SOL conversion
        win_rate: mockMetrics.winRate,
        total_trades: mockMetrics.totalTrades,
        successful_trades: Math.floor(mockMetrics.totalTrades * mockMetrics.winRate / 100),
        failed_trades: Math.floor(mockMetrics.totalTrades * (1 - mockMetrics.winRate / 100)),
        profit_factor: 2.5,
      }
    },
  });
});

// NEW: Get metrics for specific strategy
app.get('/api/v1/metrics/strategy/:id', (req, res) => {
  const strategy = mockStrategies.find(s => s.id === req.params.id);
  if (!strategy) {
    return res.status(404).json({
      success: false,
      error: {
        code: 'NOT_FOUND',
        message: 'Strategy not found',
      },
    });
  }

  res.json({
    success: true,
    data: {
      strategy_id: req.params.id,
      strategy_name: strategy.name,
      total_trades: strategy.stats.totalTrades,
      win_rate: strategy.stats.winRate,
      total_pnl: strategy.stats.totalPnl,
      sharpe_ratio: strategy.stats.sharpeRatio,
      avg_profit_per_trade: strategy.stats.totalTrades > 0 ? strategy.stats.totalPnl / strategy.stats.totalTrades : 0,
    },
  });
});

app.get('/api/v1/metrics/system', (req, res) => {
  res.json({
    success: true,
    data: {
      cpu_usage: Math.random() * 30 + 20, // 20-50%
      memory_usage: Math.random() * 40 + 40, // 40-80%
      uptime: Date.now() - 3600000, // 1 hour ago
      api_latency: Math.random() * 50 + 10, // 10-60ms
      websocket_connections: wss.clients.size,
      status: 'healthy',
      timestamp: Date.now(),
    },
  });
});

// Risk control endpoints
app.get('/api/v1/risk/limits', (req, res) => {
  res.json({
    success: true,
    data: {
      maxPositionSizeSol: 10,
      maxPositionSizePercent: 20,
      maxTotalExposureSol: 100,
      maxPositions: 10,
      maxLossPerTradeSol: 2,
      maxDailyLossSol: 10,
      maxDrawdownPercent: 20,
      minRiskScore: 70,
      maxRiskScore: 95,
      blockExtremeRisk: true,
    },
  });
});

app.put('/api/v1/risk/limits', (req, res) => {
  const {
    maxPositionSizeSol,
    maxPositionSizePercent,
    maxTotalExposureSol,
    maxPositions,
    maxLossPerTradeSol,
    maxDailyLossSol,
    maxDrawdownPercent,
  } = req.body;

  // Validation: All numeric values must be positive
  const numericFields = {
    maxPositionSizeSol,
    maxPositionSizePercent,
    maxTotalExposureSol,
    maxPositions,
    maxLossPerTradeSol,
    maxDailyLossSol,
    maxDrawdownPercent,
  };

  for (const [field, value] of Object.entries(numericFields)) {
    if (value !== undefined && value <= 0) {
      return res.status(400).json({
        success: false,
        error: {
          code: 'INVALID_PARAMETER',
          message: `${field} must be positive`,
        },
      });
    }
  }

  res.json({
    success: true,
    data: req.body,
  });
});

// NEW: Get current risk status
app.get('/api/v1/risk/status', (req, res) => {
  const currentExposure = mockPositions.reduce((sum, p) => sum + p.invested_usd, 0);
  const dailyPnl = mockTrades
    .filter(t => new Date(t.created_at) > new Date(Date.now() - 86400000))
    .reduce((sum, t) => sum + (t.pnl_usd || 0), 0);

  res.json({
    success: true,
    data: {
      current_exposure_sol: currentExposure / 20, // Mock SOL conversion
      current_positions: mockPositions.length,
      daily_pnl_sol: dailyPnl / 20,
      daily_loss_sol: dailyPnl < 0 ? Math.abs(dailyPnl / 20) : 0,
      current_drawdown_percent: 5.2,
      is_within_limits: true,
      warnings: [],
      timestamp: Date.now(),
    },
  });
});

// Health check
app.get('/api/v1/health', (req, res) => {
  res.json({
    success: true,
    data: {
      status: 'healthy',
      timestamp: Date.now(),
      websocket_connections: wss.clients.size,
    },
  });
});

// Start server (HTTP + WebSocket)
server.listen(PORT, () => {
  console.log(`\n🚀 Mock API Server running on http://localhost:${PORT}`);
  console.log(`🔌 WebSocket server running on ws://localhost:${PORT}/ws`);
  console.log(`📱 Ready to serve frontend at http://localhost:5173`);
  console.log('\n✅ Available HTTP endpoints:');
  console.log('   📝 Authentication:');
  console.log('      - POST /api/v1/auth/login');
  console.log('      - POST /api/v1/auth/logout');
  console.log('   🪙 Tokens:');
  console.log('      - GET  /api/v1/tokens');
  console.log('      - GET  /api/v1/tokens/:mint');
  console.log('   🎯 Strategies:');
  console.log('      - GET  /api/v1/strategies');
  console.log('      - POST /api/v1/strategies');
  console.log('      - POST /api/v1/strategies/:id/start');
  console.log('      - POST /api/v1/strategies/:id/pause');
  console.log('   💼 Positions:');
  console.log('      - GET  /api/v1/positions');
  console.log('      - GET  /api/v1/positions/:id');
  console.log('      - POST /api/v1/positions/:id/close');
  console.log('   📊 Trades:');
  console.log('      - GET  /api/v1/trades');
  console.log('      - GET  /api/v1/trades/:id');
  console.log('   📈 Metrics:');
  console.log('      - GET  /api/v1/metrics/summary');
  console.log('      - GET  /api/v1/metrics/strategy/:id');
  console.log('      - GET  /api/v1/metrics/system');
  console.log('   🛡️  Risk Control:');
  console.log('      - GET  /api/v1/risk/limits');
  console.log('      - PUT  /api/v1/risk/limits');
  console.log('      - GET  /api/v1/risk/status');
  console.log('   ❤️  Health:');
  console.log('      - GET  /api/v1/health');
  console.log('\n✅ WebSocket endpoints:');
  console.log('   - WS   /ws?token=YOUR_TOKEN');
  console.log('\n🎉 All 21 endpoints ready! (7 NEW endpoints added)\n');
});
