import { useQuery, useMutation, useQueryClient } from 'react-query'
import { strategiesApi } from '@/api/strategies'
import { useStrategyStore } from '@/stores/strategyStore'
import type { Strategy, StrategyConfig } from '@/types/strategy'
import toast from 'react-hot-toast'

export const useStrategies = () => {
  const { setStrategies, setLoading, setError } = useStrategyStore()

  return useQuery(
    ['strategies'],
    async () => {
      setLoading(true)
      try {
        const strategies = await strategiesApi.getStrategies()
        setStrategies(strategies)
        setError(null)
        return strategies
      } catch (error: any) {
        setError(error.message || 'Failed to fetch strategies')
        throw error
      } finally {
        setLoading(false)
      }
    },
    {
      staleTime: 10000,
      onError: (error: any) => {
        setError(error.message || 'Failed to fetch strategies')
      },
    }
  )
}

export const useStrategy = (id: string) => {
  return useQuery(
    ['strategy', id],
    () => strategiesApi.getStrategy(id),
    {
      enabled: !!id,
      staleTime: 10000,
    }
  )
}

export const useCreateStrategy = () => {
  const queryClient = useQueryClient()

  return useMutation(
    (data: { name: string; config: StrategyConfig; priority?: number }) =>
      strategiesApi.createStrategy(data),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['strategies'])
        toast.success('Strategy created successfully')
      },
      onError: (error: any) => {
        toast.error(error.message || 'Failed to create strategy')
      },
    }
  )
}

export const useUpdateStrategy = () => {
  const queryClient = useQueryClient()

  return useMutation(
    ({ id, updates }: { id: string; updates: Partial<Strategy> }) =>
      strategiesApi.updateStrategy(id, updates),
    {
      onSuccess: (data) => {
        queryClient.invalidateQueries(['strategies'])
        queryClient.invalidateQueries(['strategy', data.id])
        toast.success('Strategy updated successfully')
      },
      onError: (error: any) => {
        toast.error(error.message || 'Failed to update strategy')
      },
    }
  )
}

export const useDeleteStrategy = () => {
  const queryClient = useQueryClient()

  return useMutation(
    (id: string) => strategiesApi.deleteStrategy(id),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['strategies'])
        toast.success('Strategy deleted successfully')
      },
      onError: (error: any) => {
        toast.error(error.message || 'Failed to delete strategy')
      },
    }
  )
}

export const useToggleStrategy = () => {
  const queryClient = useQueryClient()
  const { toggleStrategy } = useStrategyStore()

  return useMutation(
    ({ id, enabled }: { id: string; enabled: boolean }) =>
      strategiesApi.toggleStrategy(id, enabled),
    {
      onMutate: ({ id }) => {
        // Optimistic update
        toggleStrategy(id)
      },
      onSuccess: () => {
        queryClient.invalidateQueries(['strategies'])
      },
      onError: (error: any, { id }) => {
        // Revert optimistic update
        toggleStrategy(id)
        toast.error(error.message || 'Failed to toggle strategy')
      },
    }
  )
}

export const useUpdateStrategyPriority = () => {
  const queryClient = useQueryClient()

  return useMutation(
    ({ id, priority }: { id: string; priority: number }) =>
      strategiesApi.updatePriority(id, priority),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['strategies'])
      },
      onError: (error: any) => {
        toast.error(error.message || 'Failed to update priority')
      },
    }
  )
}

export const useStrategyStats = (id: string) => {
  return useQuery(
    ['strategy-stats', id],
    () => strategiesApi.getStats(id),
    {
      enabled: !!id,
      refetchInterval: 5000,
    }
  )
}

export const useStrategyPerformance = (
  id: string,
  period: '24h' | '7d' | '30d' | 'all'
) => {
  return useQuery(
    ['strategy-performance', id, period],
    () => strategiesApi.getPerformance(id, period),
    {
      enabled: !!id,
      staleTime: 30000,
    }
  )
}
