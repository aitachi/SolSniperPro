// Risk Management Types
export interface RiskConfig {
  // Position Limits
  max_position_size_sol: number
  max_position_size_percentage: number
  max_total_exposure_sol: number
  max_positions_count: number

  // Loss Limits
  max_loss_per_trade_sol: number
  max_loss_per_trade_percentage: number
  max_daily_loss_sol: number
  max_drawdown_percentage: number

  // Risk Scoring
  min_risk_score: number
  max_risk_score: number
  block_extreme_risk: boolean

  // Blacklist
  enable_blacklist: boolean
  blacklisted_tokens: string[]
  blacklisted_creators: string[]

  // Cooldown
  enable_cooldown: boolean
  cooldown_after_loss_count: number
  cooldown_duration_minutes: number

  // Slippage Protection
  max_slippage_bps: number
  auto_adjust_slippage: boolean

  // MEV Protection
  enable_mev_protection: boolean
  detect_sandwich: boolean
  detect_frontrunning: boolean
  use_jito_bundle: boolean

  updated_at: string
}

export interface RiskStats {
  current_positions_count: number
  current_exposure_sol: number
  current_exposure_usd: number
  exposure_percentage: number

  daily_pnl_sol: number
  daily_pnl_usd: number
  daily_loss_sol: number
  daily_trades: number
  daily_losses: number

  current_drawdown_pct: number
  max_drawdown_pct: number

  in_cooldown: boolean
  cooldown_ends_at?: string
  consecutive_losses: number

  risk_limit_usage: RiskLimitUsage
  recent_violations: RiskViolation[]
}

export interface RiskLimitUsage {
  position_size: number // 0-100
  total_exposure: number // 0-100
  positions_count: number // 0-100
  daily_loss: number // 0-100
  drawdown: number // 0-100
}

export interface RiskViolation {
  type: string
  severity: 'LOW' | 'MEDIUM' | 'HIGH'
  message: string
  timestamp: string
  action_taken: string
}

export interface BlacklistEntry {
  address: string
  type: 'TOKEN' | 'CREATOR'
  reason: string
  added_at: string
  added_by: string
}

export interface RiskAlert {
  id: string
  type: 'POSITION_LIMIT' | 'LOSS_LIMIT' | 'DRAWDOWN' | 'BLACKLIST' | 'MEV' | 'SLIPPAGE'
  severity: 'INFO' | 'WARNING' | 'CRITICAL'
  message: string
  details: Record<string, any>
  created_at: string
  acknowledged: boolean
}
