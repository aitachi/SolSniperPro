import client from './client'
import type {
  MetricsSummary,
  PerformanceData,
  HeatMapData,
  StrategyComparison,
} from '@/types/metrics'

export const metricsApi = {
  /**
   * Get complete metrics summary
   */
  getSummary: async (): Promise<MetricsSummary> => {
    return client.get('/metrics/summary')
  },

  /**
   * Get trading metrics
   */
  getTradingMetrics: async (params?: {
    start_date?: string
    end_date?: string
  }) => {
    return client.get('/metrics/trading', { params })
  },

  /**
   * Get strategy metrics
   */
  getStrategyMetrics: async (strategyName?: string) => {
    return client.get('/metrics/strategies', {
      params: { strategy_name: strategyName },
    })
  },

  /**
   * Get system health metrics
   */
  getSystemHealth: async () => {
    return client.get('/metrics/system')
  },

  /**
   * Get RPC endpoint metrics
   */
  getRpcMetrics: async () => {
    return client.get('/metrics/rpc')
  },

  /**
   * Get performance data over time
   */
  getPerformance: async (
    period: '1h' | '24h' | '7d' | '30d' | 'all'
  ): Promise<PerformanceData> => {
    return client.get('/metrics/performance', { params: { period } })
  },

  /**
   * Get trading heatmap data
   */
  getHeatMap: async (params?: {
    start_date?: string
    end_date?: string
  }): Promise<HeatMapData[]> => {
    return client.get('/metrics/heatmap', { params })
  },

  /**
   * Get strategy comparison
   */
  getStrategyComparison: async (
    period: '24h' | '7d' | '30d' | 'all'
  ): Promise<StrategyComparison> => {
    return client.get('/metrics/strategy-comparison', { params: { period } })
  },

  /**
   * Export metrics to CSV
   */
  exportMetrics: async (params: {
    start_date: string
    end_date: string
    include_trades?: boolean
    include_positions?: boolean
  }): Promise<Blob> => {
    return client.get('/metrics/export', {
      params,
      responseType: 'blob',
    })
  },
}
