import { useQuery } from 'react-query'
import { tokensApi } from '@/api/tokens'
import { useTokenStore } from '@/stores/tokenStore'
import type { TokenFilters } from '@/types/token'

/**
 * Get tokens list with filters
 */
export const useTokens = (filters?: TokenFilters) => {
  const { setTokens, setLoading, setError } = useTokenStore()

  return useQuery(
    ['tokens', filters],
    async () => {
      setLoading(true)
      try {
        const response = await tokensApi.getTokens(filters)
        setTokens(response.data)
        setError(null)
        return response
      } catch (error: any) {
        setError(error.message || 'Failed to fetch tokens')
        throw error
      } finally {
        setLoading(false)
      }
    },
    {
      refetchInterval: 3000, // Refresh every 3 seconds
      staleTime: 2000,
      onError: (error: any) => {
        setError(error.message || 'Failed to fetch tokens')
      },
    }
  )
}

/**
 * Get single token by mint address
 */
export const useToken = (mint: string) => {
  return useQuery(
    ['token', mint],
    () => tokensApi.getToken(mint),
    {
      enabled: !!mint,
      staleTime: 5000,
    }
  )
}

/**
 * Get token risk score
 * Uses fallback implementation (extracts from token detail)
 */
export const useTokenRiskScore = (mint: string) => {
  return useQuery(
    ['token-risk', mint],
    () => tokensApi.getRiskScore(mint),
    {
      enabled: !!mint,
      staleTime: 10000,
    }
  )
}

// ==================== DISABLED HOOKS ====================
// The following hooks are commented out because backend endpoints
// are not implemented yet. Uncomment when backend is ready.

// /**
//  * Refresh token data from blockchain
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useRefreshToken = () => {
//   const queryClient = useQueryClient()
//
//   return useMutation(
//     (mint: string) => tokensApi.refreshToken(mint),
//     {
//       onSuccess: (_, mint) => {
//         queryClient.invalidateQueries(['token', mint])
//         queryClient.invalidateQueries(['tokens'])
//       },
//     }
//   )
// }

// /**
//  * Search tokens by symbol or name
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useSearchTokens = () => {
//   return useMutation((query: string) => tokensApi.searchTokens(query))
// }

// /**
//  * Get trending tokens
//  * DISABLED: Backend endpoint not implemented
//  */
// export const useTrendingTokens = (limit?: number) => {
//   return useQuery(
//     ['trending-tokens', limit],
//     () => tokensApi.getTrending(limit),
//     {
//       staleTime: 30000, // 30 seconds
//     }
//   )
// }
