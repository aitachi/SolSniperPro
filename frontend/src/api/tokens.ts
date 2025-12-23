import client from './client'
import type { TokenInfo, TokenDetail, TokenFilters, RiskScore } from '@/types/token'
import type { PaginatedResponse } from '@/types/api'

export const tokensApi = {
  /**
   * Get list of tokens with filters
   * Backend endpoint: GET /api/v1/tokens
   */
  getTokens: async (filters?: TokenFilters): Promise<PaginatedResponse<TokenInfo>> => {
    return client.get('/tokens', { params: filters })
  },

  /**
   * Get single token details
   * Backend endpoint: GET /api/v1/tokens/:mint
   */
  getToken: async (mint: string): Promise<TokenDetail> => {
    return client.get(`/tokens/${mint}`)
  },

  // Note: The following endpoints are not implemented in backend yet
  // Using fallbacks or commenting out to prevent errors

  /**
   * Get token risk score
   * Fallback: Get from token detail instead
   */
  getRiskScore: async (mint: string): Promise<RiskScore> => {
    // Fallback: get from token detail
    const token = await tokensApi.getToken(mint)
    // If token already has a full risk_score object, return it
    if (token.risk_score && typeof token.risk_score === 'object') {
      return token.risk_score
    }
    // Otherwise create a minimal risk score
    return {
      total_score: 0,
      liquidity_score: 0,
      holder_score: 0,
      contract_score: 0,
      market_score: 0,
      risk_level: 'MEDIUM',
      risk_flags: [],
      warnings: [],
    }
  },

  // /**
  //  * Refresh token data from blockchain
  //  */
  // refreshToken: async (mint: string): Promise<TokenInfo> => {
  //   return client.post(`/tokens/${mint}/refresh`)
  // },

  // /**
  //  * Search tokens by symbol or name
  //  */
  // searchTokens: async (query: string): Promise<TokenInfo[]> => {
  //   return client.get('/tokens/search', { params: { q: query } })
  // },

  // /**
  //  * Get trending tokens
  //  */
  // getTrending: async (limit?: number): Promise<TokenInfo[]> => {
  //   return client.get('/tokens/trending', { params: { limit } })
  // },
}
