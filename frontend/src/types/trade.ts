// Trade Types
export interface Trade {
  id: string
  token_mint: string
  token_symbol: string
  strategy_name: string
  side: 'BUY' | 'SELL'
  amount_sol: number
  amount_usd: number
  amount_tokens: number
  price_usd: number
  status: TradeStatus
  tx_signature?: string
  slippage_bps: number
  priority_fee_lamports: number
  gas_cost_sol: number
  pnl_sol?: number
  pnl_usd?: number
  pnl_percentage?: number
  error_message?: string
  created_at: string
  executed_at?: string
}

export type TradeStatus =
  | 'PENDING'
  | 'SIMULATING'
  | 'EXECUTING'
  | 'COMPLETED'
  | 'FAILED'
  | 'CANCELLED'

export interface TradeFilters {
  strategy_name?: string
  token_mint?: string
  status?: TradeStatus
  side?: 'BUY' | 'SELL'
  min_pnl_usd?: number
  max_pnl_usd?: number
  start_date?: string
  end_date?: string
  limit?: number
  offset?: number
  sort_by?: 'created_at' | 'pnl_usd' | 'amount_usd'
  order?: 'asc' | 'desc'
}

export interface Position {
  id: string
  token_mint: string
  token_symbol: string
  token_name: string
  strategy_name: string
  entry_price_usd: number
  current_price_usd: number
  amount_tokens: number
  invested_sol: number
  invested_usd: number
  current_value_usd: number
  pnl_sol: number
  pnl_usd: number
  pnl_percentage: number
  unrealized_pnl_usd: number
  peak_pnl_usd: number
  drawdown_from_peak_pct: number
  holding_hours: number
  entry_tx: string
  entry_at: string
  status: PositionStatus
  exit_signals: ExitSignal[]
}

export type PositionStatus = 'ACTIVE' | 'CLOSING' | 'CLOSED'

export interface ExitSignal {
  strategy_type: string
  should_exit: boolean
  exit_percentage: number
  reason: string
  urgency: 'LOW' | 'MEDIUM' | 'HIGH'
}

export interface TradeFormData {
  token_mint: string
  side: 'BUY' | 'SELL'
  amount_sol?: number
  amount_tokens?: number
  slippage_bps: number
  use_jito: boolean
  priority_fee_lamports: number
  strategy_name?: string
}

export interface TradeSimulation {
  estimated_tokens: number
  estimated_price: number
  estimated_slippage_bps: number
  price_impact_pct: number
  total_cost_sol: number
  success_probability: number
  warnings: string[]
}
