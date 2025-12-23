import client from './client'
import type { Strategy, StrategyConfig, StrategyStats } from '@/types/strategy'
// import type { StrategyPerformance } from '@/types/strategy'

export const strategiesApi = {
  /**
   * Get all strategies
   * Backend endpoint: GET /api/v1/strategies
   */
  getStrategies: async (): Promise<Strategy[]> => {
    return client.get('/strategies')
  },

  /**
   * Get single strategy
   * Fallback: Filter from list since GET /strategies/:id not implemented
   */
  getStrategy: async (id: string): Promise<Strategy> => {
    const strategies = await strategiesApi.getStrategies()
    const strategy = strategies.find(s => s.id === id)
    if (!strategy) {
      throw new Error(`Strategy ${id} not found`)
    }
    return strategy
  },

  /**
   * Create new strategy
   * Backend endpoint: POST /api/v1/strategies
   */
  createStrategy: async (data: {
    name: string
    type: string
    config?: StrategyConfig
    priority?: number
  }): Promise<Strategy> => {
    return client.post('/strategies', data)
  },

  /**
   * Start strategy
   * Backend endpoint: POST /api/v1/strategies/:id/start
   */
  startStrategy: async (id: string): Promise<Strategy> => {
    await client.post(`/strategies/${id}/start`)
    return strategiesApi.getStrategy(id)
  },

  /**
   * Pause strategy
   * Backend endpoint: POST /api/v1/strategies/:id/pause
   */
  pauseStrategy: async (id: string): Promise<Strategy> => {
    await client.post(`/strategies/${id}/pause`)
    return strategiesApi.getStrategy(id)
  },

  /**
   * Toggle strategy enabled state
   * Maps to start/pause endpoints
   */
  toggleStrategy: async (id: string, enabled: boolean): Promise<Strategy> => {
    if (enabled) {
      return strategiesApi.startStrategy(id)
    } else {
      return strategiesApi.pauseStrategy(id)
    }
  },

  // Note: The following endpoints are not implemented in backend yet
  // Commenting out to prevent 404 errors

  // /**
  //  * Update strategy configuration
  //  */
  // updateStrategy: async (id: string, updates: Partial<Strategy>): Promise<Strategy> => {
  //   return client.put(`/strategies/${id}`, updates)
  // },

  // /**
  //  * Delete strategy
  //  */
  // deleteStrategy: async (id: string): Promise<void> => {
  //   return client.delete(`/strategies/${id}`)
  // },

  // /**
  //  * Update strategy priority
  //  */
  // updatePriority: async (id: string, priority: number): Promise<Strategy> => {
  //   return client.patch(`/strategies/${id}/priority`, { priority })
  // },

  /**
   * Get strategy statistics
   * Uses /metrics/strategy/:id endpoint
   */
  getStats: async (id: string): Promise<StrategyStats> => {
    return client.get(`/metrics/strategy/${id}`)
  },

  // /**
  //  * Get strategy performance
  //  */
  // getPerformance: async (
  //   id: string,
  //   period: '24h' | '7d' | '30d' | 'all'
  // ): Promise<StrategyPerformance> => {
  //   return client.get(`/strategies/${id}/performance`, { params: { period } })
  // },

  // /**
  //  * Backtest strategy
  //  */
  // backtest: async (
  //   config: StrategyConfig,
  //   params: { start_date: string; end_date: string }
  // ): Promise<StrategyPerformance> => {
  //   return client.post('/strategies/backtest', { config, ...params })
  // },
}
