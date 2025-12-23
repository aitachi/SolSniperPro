import client from './client'
import type {
  Trade,
  Position,
  TradeFilters,
  // TradeFormData,
  // TradeSimulation,
} from '@/types/trade'
import type { PaginatedResponse } from '@/types/api'

export const tradesApi = {
  /**
   * Get trade history with filters
   * Backend endpoint: GET /api/v1/trades
   */
  getTrades: async (filters?: TradeFilters): Promise<PaginatedResponse<Trade>> => {
    return client.get('/trades', { params: filters })
  },

  /**
   * Get single trade details
   * Backend endpoint: GET /api/v1/trades/:id
   */
  getTrade: async (id: string): Promise<Trade> => {
    return client.get(`/trades/${id}`)
  },

  /**
   * Get active positions
   * Backend endpoint: GET /api/v1/positions
   */
  getPositions: async (): Promise<Position[]> => {
    return client.get('/positions')
  },

  /**
   * Get single position
   * Backend endpoint: GET /api/v1/positions/:id
   */
  getPosition: async (id: string): Promise<Position> => {
    return client.get(`/positions/${id}`)
  },

  /**
   * Close position (sell tokens)
   * Backend endpoint: POST /api/v1/positions/:id/close
   */
  closePosition: async (
    id: string,
    params?: { percentage?: number; slippage_bps?: number }
  ): Promise<Trade> => {
    return client.post(`/positions/${id}/close`, params)
  },

  // Note: The following endpoints are not implemented in backend yet
  // Commenting out to prevent 404 errors

  // /**
  //  * Simulate trade before execution
  //  */
  // simulateTrade: async (data: TradeFormData): Promise<TradeSimulation> => {
  //   return client.post('/trades/simulate', data)
  // },

  // /**
  //  * Execute manual trade
  //  */
  // executeTrade: async (data: TradeFormData): Promise<Trade> => {
  //   return client.post('/trades', data)
  // },

  // /**
  //  * Cancel pending trade
  //  */
  // cancelTrade: async (id: string): Promise<void> => {
  //   return client.post(`/trades/${id}/cancel`)
  // },

  // /**
  //  * Get position history for a token
  //  */
  // getPositionHistory: async (tokenMint: string): Promise<Position[]> => {
  //   return client.get('/positions/history', { params: { token_mint: tokenMint } })
  // },

  /**
   * Get trade statistics
   * Uses /metrics/summary endpoint instead
   */
  getTradeStats: async (_params?: {
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
    // Fallback to metrics summary
    const summary: any = await client.get('/metrics/summary')
    return {
      total_trades: summary.trading_metrics?.total_trades || 0,
      total_pnl: summary.trading_metrics?.total_pnl_usd || 0,
      win_rate: summary.trading_metrics?.win_rate || 0,
      avg_profit: 0,
      max_profit: 0,
      max_loss: 0,
    }
  },
}
