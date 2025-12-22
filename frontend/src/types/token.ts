// Token Information Types
import type { Trade } from './trade'

export interface TokenInfo {
  mint: string
  symbol: string
  name: string
  decimals: number
  created_at: string
  liquidity_sol: number
  liquidity_usd: number
  holders_count: number
  top10_holder_percentage: number
  price_usd: number
  price_change_1h: number
  price_change_24h: number
  volume_1h: number
  volume_24h: number
  market_cap: number
  age_hours: number
  creator_address: string
  is_mutable: boolean
  is_renounced: boolean
  has_freeze_authority: boolean
  data: TokenMetadata
}

export interface TokenMetadata {
  uri?: string
  image_url?: string
  description?: string
  twitter?: string
  telegram?: string
  website?: string
  risk_flags?: string[]
}

export interface TokenFilters {
  min_liquidity?: number
  max_liquidity?: number
  min_holders?: number
  max_age_hours?: number
  min_risk_score?: number
  max_risk_score?: number
  only_renounced?: boolean
  only_immutable?: boolean
  search?: string
  sort_by?: 'liquidity' | 'holders' | 'age' | 'risk_score' | 'volume'
  order?: 'asc' | 'desc'
  limit?: number
  offset?: number
}

export interface RiskScore {
  total_score: number
  liquidity_score: number
  holder_score: number
  contract_score: number
  market_score: number
  risk_level: 'LOW' | 'MEDIUM' | 'HIGH' | 'EXTREME'
  risk_flags: string[]
  warnings: string[]
}

export interface TokenDetail extends TokenInfo {
  risk_score: RiskScore
  matched_strategies: StrategyMatch[]
  recent_trades: Trade[]
  price_history: PricePoint[]
}

export interface PricePoint {
  timestamp: string
  price: number
  volume: number
}

export interface StrategyMatch {
  strategy_name: string
  priority: number
  expected_profit: number
  confidence: number
  entry_conditions_met: string[]
}
