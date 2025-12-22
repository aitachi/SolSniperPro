// Strategy Types
export interface Strategy {
  id: string
  name: string
  enabled: boolean
  priority: number
  config: StrategyConfig
  stats: StrategyStats
  created_at: string
  updated_at: string
}

export interface StrategyConfig {
  // Entry Conditions
  min_liquidity_sol?: number
  max_liquidity_sol?: number
  min_holders?: number
  min_age_hours?: number
  max_age_hours?: number
  min_risk_score?: number
  min_volume_1h?: number
  min_price_change_1h?: number
  max_top10_ratio?: number
  only_renounced?: boolean
  only_immutable?: boolean

  // Position Sizing
  position_sizing_strategy: PositionSizingStrategy
  position_size_sol?: number
  position_size_percentage?: number
  max_position_size_sol?: number
  volatility_lookback_hours?: number
  kelly_fraction?: number

  // Exit Strategy
  exit_strategies: ExitStrategyConfig[]

  // Risk Management
  max_slippage_bps?: number
  use_jito?: boolean
  priority_fee_lamports?: number
}

export type PositionSizingStrategy =
  | 'FixedAmount'
  | 'FixedPercentage'
  | 'VolatilityBased'
  | 'KellyCriterion'
  | 'RiskParity'
  | 'Martingale'
  | 'AntiMartingale'

export interface ExitStrategyConfig {
  type: ExitStrategyType
  enabled: boolean
  // Stop Loss
  stop_loss_pct?: number
  // Take Profit
  take_profit_pct?: number
  // Trailing Stop
  trailing_activation_pct?: number
  trailing_stop_pct?: number
  // Time-based
  max_holding_hours?: number
  // Liquidity-based
  min_liquidity_threshold_pct?: number
  // Scale Out
  scale_out_levels?: ScaleOutLevel[]
}

export type ExitStrategyType =
  | 'StopLoss'
  | 'TakeProfit'
  | 'TrailingStop'
  | 'TimeBased'
  | 'LiquidityBased'
  | 'BreakEven'
  | 'ScaleOut'

export interface ScaleOutLevel {
  profit_pct: number
  exit_pct: number
}

export interface StrategyStats {
  total_trades: number
  successful_trades: number
  failed_trades: number
  win_rate: number
  total_pnl_sol: number
  total_pnl_usd: number
  avg_profit_sol: number
  avg_loss_sol: number
  max_profit_sol: number
  max_loss_sol: number
  avg_holding_hours: number
  sharpe_ratio: number
  max_drawdown_pct: number
  roi_pct: number
  last_trade_at?: string
}

export interface StrategyPriority {
  strategy_id: string
  priority: number
  weight: number
  conditions: string[]
}

export interface StrategyPerformance {
  strategy_name: string
  period: '24h' | '7d' | '30d' | 'all'
  stats: StrategyStats
  equity_curve: EquityPoint[]
  recent_trades: Trade[]
}

export interface EquityPoint {
  timestamp: string
  equity: number
  cumulative_pnl: number
}
