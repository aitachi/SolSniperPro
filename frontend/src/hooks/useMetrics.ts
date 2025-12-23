import { useQuery } from 'react-query'
import { metricsApi } from '@/api/metrics'

export const useMetricsSummary = () => {
  return useQuery(
    ['metrics-summary'],
    () => metricsApi.getSummary(),
    {
      refetchInterval: 5000, // Refresh every 5 seconds
      staleTime: 3000,
    }
  )
}

// Note: getTradingMetrics not implemented in backend yet
// export const useTradingMetrics = (params?: {
//   start_date?: string
//   end_date?: string
// }) => {
//   return useQuery(
//     ['trading-metrics', params],
//     () => metricsApi.getTradingMetrics(params),
//     {
//       staleTime: 10000,
//     }
//   )
// }

export const useStrategyMetrics = (strategyId?: string) => {
  return useQuery(
    ['strategy-metrics', strategyId],
    () => strategyId ? metricsApi.getStrategyMetrics(strategyId) : Promise.resolve(null),
    {
      enabled: !!strategyId,
      refetchInterval: 10000,
      staleTime: 5000,
    }
  )
}

export const useSystemHealth = () => {
  return useQuery(
    ['system-health'],
    () => metricsApi.getSystemHealth(),
    {
      refetchInterval: 3000,
      staleTime: 2000,
    }
  )
}

// Note: The following hooks are disabled because endpoints are not implemented yet
// Uncomment when backend implements these features

// export const useRpcMetrics = () => {
//   return useQuery(
//     ['rpc-metrics'],
//     () => metricsApi.getRpcMetrics(),
//     {
//       refetchInterval: 10000,
//       staleTime: 5000,
//     }
//   )
// }

// export const usePerformance = (period: '1h' | '24h' | '7d' | '30d' | 'all') => {
//   return useQuery(
//     ['performance', period],
//     () => metricsApi.getPerformance(period),
//     {
//       staleTime: 30000,
//     }
//   )
// }

// export const useHeatMap = (params?: {
//   start_date?: string
//   end_date?: string
// }) => {
//   return useQuery(
//     ['heatmap', params],
//     () => metricsApi.getHeatMap(params),
//     {
//       staleTime: 60000, // 1 minute
//     }
//   )
// }

// export const useStrategyComparison = (
//   period: '24h' | '7d' | '30d' | 'all'
// ) => {
//   return useQuery(
//     ['strategy-comparison', period],
//     () => metricsApi.getStrategyComparison(period),
//     {
//       staleTime: 30000,
//     }
//   )
// }
