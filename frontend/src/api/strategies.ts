import client from './client'
import type { Strategy, StrategyConfig, StrategyStats, StrategyPerformance } from '@/types/strategy'

export const strategiesApi = {
  /**
   * Get all strategies
   */
  getStrategies: async (): Promise<Strategy[]> => {
    return client.get('/strategies')
  },

  /**
   * Get single strategy
   */
  getStrategy: async (id: string): Promise<Strategy> => {
    return client.get(`/strategies/${id}`)
  },

  /**
   * Create new strategy
   */
  createStrategy: async (data: {
    name: string
    config: StrategyConfig
    priority?: number
  }): Promise<Strategy> => {
    return client.post('/strategies', data)
  },

  /**
   * Update strategy configuration
   */
  updateStrategy: async (id: string, updates: Partial<Strategy>): Promise<Strategy> => {
    return client.put(`/strategies/${id}`, updates)
  },

  /**
   * Delete strategy
   */
  deleteStrategy: async (id: string): Promise<void> => {
    return client.delete(`/strategies/${id}`)
  },

  /**
   * Toggle strategy enabled state
   */
  toggleStrategy: async (id: string, enabled: boolean): Promise<Strategy> => {
    return client.patch(`/strategies/${id}/toggle`, { enabled })
  },

  /**
   * Update strategy priority
   */
  updatePriority: async (id: string, priority: number): Promise<Strategy> => {
    return client.patch(`/strategies/${id}/priority`, { priority })
  },

  /**
   * Get strategy statistics
   */
  getStats: async (id: string): Promise<StrategyStats> => {
    return client.get(`/strategies/${id}/stats`)
  },

  /**
   * Get strategy performance
   */
  getPerformance: async (
    id: string,
    period: '24h' | '7d' | '30d' | 'all'
  ): Promise<StrategyPerformance> => {
    return client.get(`/strategies/${id}/performance`, { params: { period } })
  },

  /**
   * Backtest strategy
   */
  backtest: async (
    config: StrategyConfig,
    params: { start_date: string; end_date: string }
  ): Promise<StrategyPerformance> => {
    return client.post('/strategies/backtest', { config, ...params })
  },
}
