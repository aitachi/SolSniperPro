import { useQuery, useMutation, useQueryClient } from 'react-query'
import { tradesApi } from '@/api/trades'
import { useTradeStore } from '@/stores/tradeStore'
import type { TradeFilters, TradeFormData } from '@/types/trade'
import toast from 'react-hot-toast'

export const useTrades = (filters?: TradeFilters) => {
  const { setTrades, setLoading, setError } = useTradeStore()

  return useQuery(
    ['trades', filters],
    async () => {
      setLoading(true)
      try {
        const response = await tradesApi.getTrades(filters)
        setTrades(response.data)
        setError(null)
        return response
      } catch (error: any) {
        setError(error.message || 'Failed to fetch trades')
        throw error
      } finally {
        setLoading(false)
      }
    },
    {
      refetchInterval: 5000, // Refresh every 5 seconds
      staleTime: 3000,
      onError: (error: any) => {
        setError(error.message || 'Failed to fetch trades')
      },
    }
  )
}

export const useTrade = (id: string) => {
  return useQuery(
    ['trade', id],
    () => tradesApi.getTrade(id),
    {
      enabled: !!id,
      refetchInterval: 2000,
    }
  )
}

export const useSimulateTrade = () => {
  return useMutation(
    (data: TradeFormData) => tradesApi.simulateTrade(data),
    {
      onError: (error: any) => {
        toast.error(error.message || 'Simulation failed')
      },
    }
  )
}

export const useExecuteTrade = () => {
  const queryClient = useQueryClient()

  return useMutation(
    (data: TradeFormData) => tradesApi.executeTrade(data),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['trades'])
        queryClient.invalidateQueries(['positions'])
        toast.success('Trade executed successfully')
      },
      onError: (error: any) => {
        toast.error(error.message || 'Trade execution failed')
      },
    }
  )
}

export const useCancelTrade = () => {
  const queryClient = useQueryClient()

  return useMutation(
    (id: string) => tradesApi.cancelTrade(id),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['trades'])
        toast.success('Trade cancelled')
      },
      onError: (error: any) => {
        toast.error(error.message || 'Failed to cancel trade')
      },
    }
  )
}

export const useTradeStats = (params?: {
  start_date?: string
  end_date?: string
  strategy_name?: string
}) => {
  return useQuery(
    ['trade-stats', params],
    () => tradesApi.getTradeStats(params),
    {
      staleTime: 10000,
    }
  )
}
