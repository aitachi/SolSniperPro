import { useQuery, useMutation, useQueryClient } from 'react-query'
import { strategiesApi } from '@/api/strategies'
import { useStrategyStore } from '@/stores/strategyStore'
import type { StrategyConfig } from '@/types/strategy'
import toast from 'react-hot-toast'

/**
 * Get all strategies
 */
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

/**
 * Get single strategy by ID
 * Uses fallback (filters from list)
 */
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

/**
 * Create new strategy
 * NOTE: Backend requires 'type' field
 */
export const useCreateStrategy = () => {
  const queryClient = useQueryClient()

  return useMutation(
    (data: { name: string; type: string; config?: StrategyConfig; priority?: number }) =>
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

/**
 * Toggle strategy (start/pause)
 */
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

/**
 * Get strategy statistics
 * Uses /metrics/strategy/:id endpoint
 */
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

// ==================== DISABLED HOOKS ====================
// The following hooks are commented out because backend endpoints
// are not implemented yet. Uncomment when backend is ready.

// /**
//  * Update strategy configuration
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useUpdateStrategy = () => {
//   const queryClient = useQueryClient()
//
//   return useMutation(
//     ({ id, updates }: { id: string; updates: Partial<Strategy> }) =>
//       strategiesApi.updateStrategy(id, updates),
//     {
//       onSuccess: (data) => {
//         queryClient.invalidateQueries(['strategies'])
//         queryClient.invalidateQueries(['strategy', data.id])
//         toast.success('Strategy updated successfully')
//       },
//       onError: (error: any) => {
//         toast.error(error.message || 'Failed to update strategy')
//       },
//     }
//   )
// }

// /**
//  * Delete strategy
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useDeleteStrategy = () => {
//   const queryClient = useQueryClient()
//
//   return useMutation(
//     (id: string) => strategiesApi.deleteStrategy(id),
//     {
//       onSuccess: () => {
//         queryClient.invalidateQueries(['strategies'])
//         toast.success('Strategy deleted successfully')
//       },
//       onError: (error: any) => {
//         toast.error(error.message || 'Failed to delete strategy')
//       },
//     }
//   )
// }

// /**
//  * Update strategy priority
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useUpdateStrategyPriority = () => {
//   const queryClient = useQueryClient()
//
//   return useMutation(
//     ({ id, priority }: { id: string; priority: number }) =>
//       strategiesApi.updatePriority(id, priority),
//     {
//       onSuccess: () => {
//         queryClient.invalidateQueries(['strategies'])
//       },
//       onError: (error: any) => {
//         toast.error(error.message || 'Failed to update priority')
//       },
//     }
//   )
// }

// /**
//  * Get strategy performance over time
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useStrategyPerformance = (
//   id: string,
//   period: '24h' | '7d' | '30d' | 'all'
// ) => {
//   return useQuery(
//     ['strategy-performance', id, period],
//     () => strategiesApi.getPerformance(id, period),
//     {
//       enabled: !!id,
//       staleTime: 30000,
//     }
//   )
// }
