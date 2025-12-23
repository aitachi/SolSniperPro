import { useQuery, useMutation, useQueryClient } from 'react-query'
import { tradesApi } from '@/api/trades'
import { useTradeStore } from '@/stores/tradeStore'
import toast from 'react-hot-toast'

/**
 * Get all active positions
 */
export const usePositions = () => {
  const { setPositions, setLoading, setError } = useTradeStore()

  return useQuery(
    ['positions'],
    async () => {
      setLoading(true)
      try {
        const positions = await tradesApi.getPositions()
        setPositions(positions)
        setError(null)
        return positions
      } catch (error: any) {
        setError(error.message || 'Failed to fetch positions')
        throw error
      } finally {
        setLoading(false)
      }
    },
    {
      refetchInterval: 2000, // Refresh every 2 seconds
      staleTime: 1000,
      onError: (error: any) => {
        setError(error.message || 'Failed to fetch positions')
      },
    }
  )
}

/**
 * Get single position by ID
 */
export const usePosition = (id: string) => {
  return useQuery(
    ['position', id],
    () => tradesApi.getPosition(id),
    {
      enabled: !!id,
      refetchInterval: 2000,
    }
  )
}

/**
 * Close position (sell tokens)
 */
export const useClosePosition = () => {
  const queryClient = useQueryClient()

  return useMutation(
    ({
      id,
      params,
    }: {
      id: string
      params?: { percentage?: number; slippage_bps?: number }
    }) => tradesApi.closePosition(id, params),
    {
      onSuccess: () => {
        queryClient.invalidateQueries(['positions'])
        queryClient.invalidateQueries(['trades'])
        toast.success('Position closed successfully')
      },
      onError: (error: any) => {
        toast.error(error.message || 'Failed to close position')
      },
    }
  )
}

// ==================== DISABLED HOOKS ====================
// The following hooks are commented out because backend endpoints
// are not implemented yet. Uncomment when backend is ready.

// /**
//  * Get position history for a token
//  * DISABLED: Backend endpoint not implemented
//  */
// export const usePositionHistory = (tokenMint: string) => {
//   return useQuery(
//     ['position-history', tokenMint],
//     () => tradesApi.getPositionHistory(tokenMint),
//     {
//       enabled: !!tokenMint,
//       staleTime: 30000,
//     }
//   )
// }
