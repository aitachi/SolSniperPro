// Metrics and Analytics Types
export interface MetricsSummary {
  trading_metrics: TradingMetrics
  strategy_metrics: Record<string, StrategyMetrics>
  system_health: SystemHealthMetrics
  rpc_endpoints: Record<string, RpcEndpointMetrics>
  updated_at: string
}

export interface TradingMetrics {
  total_trades: number
  successful_trades: number
  failed_trades: number
  win_rate: number

  total_pnl_sol: number
  total_pnl_usd: number
  total_volume_usd: number

  avg_profit_sol: number
  avg_loss_sol: number
  avg_win_sol: number
  profit_factor: number

  max_profit_sol: number
  max_loss_sol: number
  max_consecutive_wins: number
  max_consecutive_losses: number

  avg_holding_duration_hours: number
  sharpe_ratio: number
  sortino_ratio: number
  max_drawdown_pct: number

  total_gas_cost_sol: number
  avg_gas_cost_sol: number
}

export interface StrategyMetrics {
  strategy_name: string
  total_trades: number
  successful_trades: number
  win_rate: number
  total_pnl_sol: number
  total_pnl_usd: number
  avg_profit_sol: number
  sharpe_ratio: number
  max_drawdown_pct: number
  last_trade_at?: string
}

export interface SystemHealthMetrics {
  uptime_seconds: number
  cpu_usage_percent: number
  memory_usage_mb: number
  memory_total_mb: number

  active_connections: number
  pending_trades: number
  active_positions: number

  event_latency_ms: number
  data_collection_latency_ms: number
  strategy_matching_latency_ms: number
  trade_execution_latency_ms: number

  cache_hit_rate: number
  cache_size_mb: number

  errors_last_hour: number
  warnings_last_hour: number

  last_health_check: string
  status: 'HEALTHY' | 'DEGRADED' | 'CRITICAL'
}

export interface RpcEndpointMetrics {
  url: string
  total_requests: number
  successful_requests: number
  failed_requests: number
  avg_latency_ms: number
  error_rate: number
  last_error?: string
  last_success_at?: string
  is_healthy: boolean
}

export interface PerformanceData {
  period: '1h' | '24h' | '7d' | '30d' | 'all'
  data_points: PerformancePoint[]
  summary: PerformanceSummary
}

export interface PerformancePoint {
  timestamp: string
  cumulative_pnl: number
  daily_pnl: number
  trades_count: number
  win_rate: number
  drawdown_pct: number
}

export interface PerformanceSummary {
  total_pnl: number
  total_trades: number
  avg_daily_pnl: number
  best_day: number
  worst_day: number
  winning_days: number
  losing_days: number
  current_streak: number
  longest_winning_streak: number
  longest_losing_streak: number
}

export interface HeatMapData {
  hour: number
  day: number
  pnl: number
  trades: number
  win_rate: number
}

export interface StrategyComparison {
  strategies: StrategyComparisonData[]
  period: string
}

export interface StrategyComparisonData {
  name: string
  pnl: number
  trades: number
  win_rate: number
  sharpe_ratio: number
  max_drawdown: number
  roi_pct: number
}
