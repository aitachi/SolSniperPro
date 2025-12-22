import { useQuery, useMutation, useQueryClient } from 'react-query'
import { tokensApi } from '@/api/tokens'
import { useTokenStore } from '@/stores/tokenStore'
import type { TokenFilters } from '@/types/token'

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

export const useRefreshToken = () => {
  const queryClient = useQueryClient()

  return useMutation(
    (mint: string) => tokensApi.refreshToken(mint),
    {
      onSuccess: (_, mint) => {
        queryClient.invalidateQueries(['token', mint])
        queryClient.invalidateQueries(['tokens'])
      },
    }
  )
}

export const useSearchTokens = () => {
  return useMutation((query: string) => tokensApi.searchTokens(query))
}

export const useTrendingTokens = (limit?: number) => {
  return useQuery(
    ['trending-tokens', limit],
    () => tokensApi.getTrending(limit),
    {
      staleTime: 30000, // 30 seconds
    }
  )
}
