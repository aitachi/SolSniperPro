import client from './client'
import type { TokenInfo, TokenDetail, TokenFilters, RiskScore } from '@/types/token'
import type { PaginatedResponse } from '@/types/api'

export const tokensApi = {
  /**
   * Get list of tokens with filters
   */
  getTokens: async (filters?: TokenFilters): Promise<PaginatedResponse<TokenInfo>> => {
    return client.get('/tokens', { params: filters })
  },

  /**
   * Get single token details
   */
  getToken: async (mint: string): Promise<TokenDetail> => {
    return client.get(`/tokens/${mint}`)
  },

  /**
   * Get token risk score
   */
  getRiskScore: async (mint: string): Promise<RiskScore> => {
    return client.get(`/tokens/${mint}/risk`)
  },

  /**
   * Refresh token data from blockchain
   */
  refreshToken: async (mint: string): Promise<TokenInfo> => {
    return client.post(`/tokens/${mint}/refresh`)
  },

  /**
   * Search tokens by symbol or name
   */
  searchTokens: async (query: string): Promise<TokenInfo[]> => {
    return client.get('/tokens/search', { params: { q: query } })
  },

  /**
   * Get trending tokens
   */
  getTrending: async (limit?: number): Promise<TokenInfo[]> => {
    return client.get('/tokens/trending', { params: { limit } })
  },
}
