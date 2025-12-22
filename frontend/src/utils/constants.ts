/**
 * Application constants
 */

// API Configuration
export const API_BASE_URL = '/api/v1'
export const WS_BASE_URL = 'ws://localhost:3000/ws'
export const API_TIMEOUT = 30000 // 30 seconds

// Pagination
export const DEFAULT_PAGE_SIZE = 20
export const MAX_PAGE_SIZE = 100

// Refresh Intervals (milliseconds)
export const METRICS_REFRESH_INTERVAL = 5000 // 5 seconds
export const TOKENS_REFRESH_INTERVAL = 3000 // 3 seconds
export const POSITIONS_REFRESH_INTERVAL = 2000 // 2 seconds
export const TRADES_REFRESH_INTERVAL = 5000 // 5 seconds

// Trading Defaults
export const DEFAULT_SLIPPAGE_BPS = 100 // 1%
export const MAX_SLIPPAGE_BPS = 1000 // 10%
export const DEFAULT_PRIORITY_FEE = 10000 // 0.00001 SOL
export const MAX_PRIORITY_FEE = 1000000 // 0.001 SOL

// Risk Limits
export const MIN_RISK_SCORE = 0
export const MAX_RISK_SCORE = 100
export const DEFAULT_MIN_RISK_SCORE = 70

// Position Sizing
export const MIN_POSITION_SIZE_SOL = 0.01
export const MAX_POSITION_SIZE_SOL = 100
export const DEFAULT_POSITION_SIZE_SOL = 0.1

// Time Periods
export const TIME_PERIODS = {
  '1H': { label: '1 Hour', hours: 1 },
  '24H': { label: '24 Hours', hours: 24 },
  '7D': { label: '7 Days', hours: 168 },
  '30D': { label: '30 Days', hours: 720 },
  'ALL': { label: 'All Time', hours: null },
} as const

// Chart Colors
export const CHART_COLORS = {
  primary: '#0ea5e9',
  success: '#10b981',
  danger: '#ef4444',
  warning: '#f59e0b',
  gray: '#64748b',
  purple: '#8b5cf6',
  pink: '#ec4899',
  indigo: '#6366f1',
}

// Risk Levels
export const RISK_LEVELS = {
  LOW: { color: '#10b981', label: 'Low Risk', max: 70 },
  MEDIUM: { color: '#f59e0b', label: 'Medium Risk', max: 85 },
  HIGH: { color: '#ef4444', label: 'High Risk', max: 95 },
  EXTREME: { color: '#dc2626', label: 'Extreme Risk', max: 100 },
} as const

// Trade Status
export const TRADE_STATUS_COLORS = {
  PENDING: '#64748b',
  SIMULATING: '#0ea5e9',
  EXECUTING: '#f59e0b',
  COMPLETED: '#10b981',
  FAILED: '#ef4444',
  CANCELLED: '#94a3b8',
} as const

// Position Status
export const POSITION_STATUS_COLORS = {
  ACTIVE: '#10b981',
  CLOSING: '#f59e0b',
  CLOSED: '#64748b',
} as const

// Strategy Types
export const STRATEGY_TYPES = [
  'EarlyBird',
  'LiquidityHunter',
  'VolumeExplosion',
  'ValueInvesting',
  'ContrarianArbitrage',
  'TimeBased',
] as const

// Position Sizing Strategies
export const POSITION_SIZING_STRATEGIES = [
  { value: 'FixedAmount', label: 'Fixed Amount', description: 'Fixed SOL amount per trade' },
  { value: 'FixedPercentage', label: 'Fixed Percentage', description: 'Fixed % of account balance' },
  { value: 'VolatilityBased', label: 'Volatility Based', description: 'Adjust based on volatility' },
  { value: 'KellyCriterion', label: 'Kelly Criterion', description: 'Optimal bet sizing formula' },
  { value: 'RiskParity', label: 'Risk Parity', description: 'Equal risk contribution' },
  { value: 'Martingale', label: 'Martingale', description: 'Increase after losses' },
  { value: 'AntiMartingale', label: 'Anti-Martingale', description: 'Increase after wins' },
] as const

// Exit Strategy Types
export const EXIT_STRATEGY_TYPES = [
  { value: 'StopLoss', label: 'Stop Loss', description: 'Exit on loss threshold' },
  { value: 'TakeProfit', label: 'Take Profit', description: 'Exit on profit target' },
  { value: 'TrailingStop', label: 'Trailing Stop', description: 'Follow price movements' },
  { value: 'TimeBased', label: 'Time Based', description: 'Exit after time limit' },
  { value: 'LiquidityBased', label: 'Liquidity Based', description: 'Exit on liquidity drop' },
  { value: 'BreakEven', label: 'Break Even', description: 'Move SL to breakeven' },
  { value: 'ScaleOut', label: 'Scale Out', description: 'Partial profit taking' },
] as const

// Toast Durations
export const TOAST_DURATION = {
  SHORT: 2000,
  MEDIUM: 4000,
  LONG: 6000,
} as const

// Local Storage Keys
export const STORAGE_KEYS = {
  AUTH_TOKEN: 'auth-storage',
  THEME: 'app-theme',
  SETTINGS: 'app-settings',
  FILTERS: 'app-filters',
} as const

// Chart Configuration
export const CHART_CONFIG = {
  gridColor: '#334155',
  textColor: '#94a3b8',
  tooltipBg: '#1e293b',
  axisColor: '#475569',
} as const

// Performance Metrics Thresholds
export const PERFORMANCE_THRESHOLDS = {
  EXCELLENT_WIN_RATE: 80,
  GOOD_WIN_RATE: 70,
  FAIR_WIN_RATE: 60,
  EXCELLENT_SHARPE: 2.0,
  GOOD_SHARPE: 1.5,
  FAIR_SHARPE: 1.0,
  MAX_ACCEPTABLE_DRAWDOWN: 20,
} as const

// System Health Thresholds
export const HEALTH_THRESHOLDS = {
  CPU_WARNING: 70,
  CPU_CRITICAL: 90,
  MEMORY_WARNING: 70,
  MEMORY_CRITICAL: 90,
  ERROR_RATE_WARNING: 0.05,
  ERROR_RATE_CRITICAL: 0.1,
  LATENCY_WARNING: 1000,
  LATENCY_CRITICAL: 3000,
} as const
