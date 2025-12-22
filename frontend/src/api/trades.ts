import client from './client'
import type {
  Trade,
  Position,
  TradeFilters,
  TradeFormData,
  TradeSimulation,
} from '@/types/trade'
import type { PaginatedResponse } from '@/types/api'

export const tradesApi = {
  /**
   * Get trade history with filters
   */
  getTrades: async (filters?: TradeFilters): Promise<PaginatedResponse<Trade>> => {
    return client.get('/trades', { params: filters })
  },

  /**
   * Get single trade details
   */
  getTrade: async (id: string): Promise<Trade> => {
    return client.get(`/trades/${id}`)
  },

  /**
   * Simulate trade before execution
   */
  simulateTrade: async (data: TradeFormData): Promise<TradeSimulation> => {
    return client.post('/trades/simulate', data)
  },

  /**
   * Execute manual trade
   */
  executeTrade: async (data: TradeFormData): Promise<Trade> => {
    return client.post('/trades', data)
  },

  /**
   * Cancel pending trade
   */
  cancelTrade: async (id: string): Promise<void> => {
    return client.post(`/trades/${id}/cancel`)
  },

  /**
   * Get active positions
   */
  getPositions: async (): Promise<Position[]> => {
    return client.get('/positions')
  },

  /**
   * Get single position
   */
  getPosition: async (id: string): Promise<Position> => {
    return client.get(`/positions/${id}`)
  },

  /**
   * Close position (sell tokens)
   */
  closePosition: async (
    id: string,
    params?: { percentage?: number; slippage_bps?: number }
  ): Promise<Trade> => {
    return client.post(`/positions/${id}/close`, params)
  },

  /**
   * Get position history for a token
   */
  getPositionHistory: async (tokenMint: string): Promise<Position[]> => {
    return client.get('/positions/history', { params: { token_mint: tokenMint } })
  },

  /**
   * Get trade statistics
   */
  getTradeStats: async (params?: {
    start_date?: string
    end_date?: string
    strategy_name?: string
  }): Promise<{
    total_trades: number
    total_pnl: number
    win_rate: number
    avg_profit: number
    max_profit: number
    max_loss: number
  }> => {
    return client.get('/trades/stats', { params })
  },
}
