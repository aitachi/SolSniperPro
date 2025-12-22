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

// Auth endpoints
app.post('/api/v1/auth/login', (req, res) => {
  const { username, password } = req.body;
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

// Token endpoints
app.get('/api/v1/tokens', (req, res) => {
  res.json({
    success: true,
    data: mockTokens,
  });
});

// Strategy endpoints
app.get('/api/v1/strategies', (req, res) => {
  res.json({
    success: true,
    data: mockStrategies,
  });
});

app.post('/api/v1/strategies/:id/start', (req, res) => {
  res.json({
    success: true,
    data: { message: 'Strategy started' },
  });
});

app.post('/api/v1/strategies/:id/pause', (req, res) => {
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

app.post('/api/v1/positions/:id/close', (req, res) => {
  res.json({
    success: true,
    data: { message: 'Position closed' },
  });
});

// Trade endpoints
app.get('/api/v1/trades', (req, res) => {
  const limit = parseInt(req.query.limit) || 10;
  res.json({
    success: true,
    data: mockTrades.slice(0, limit),
    pagination: {
      total: mockTrades.length,
      page: 1,
      limit: limit,
      totalPages: Math.ceil(mockTrades.length / limit),
    },
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
  res.json({
    success: true,
    data: req.body,
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
  console.log(`📱 Ready to serve frontend at http://localhost:5176`);
  console.log('\n✅ Available HTTP endpoints:');
  console.log('   - POST /api/v1/auth/login');
  console.log('   - GET  /api/v1/tokens');
  console.log('   - GET  /api/v1/strategies');
  console.log('   - POST /api/v1/strategies/:id/start');
  console.log('   - POST /api/v1/strategies/:id/pause');
  console.log('   - GET  /api/v1/positions');
  console.log('   - POST /api/v1/positions/:id/close');
  console.log('   - GET  /api/v1/trades');
  console.log('   - GET  /api/v1/metrics/summary');
  console.log('   - GET  /api/v1/metrics/system');
  console.log('   - GET  /api/v1/risk/limits');
  console.log('   - PUT  /api/v1/risk/limits');
  console.log('   - GET  /api/v1/health');
  console.log('\n✅ WebSocket endpoints:');
  console.log('   - WS   /ws?token=YOUR_TOKEN\n');
});
