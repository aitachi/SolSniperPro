import { create } from 'zustand'
import type { TokenInfo, TokenFilters } from '@/types/token'

interface TokenState {
  tokens: TokenInfo[]
  selectedToken: TokenInfo | null
  filters: TokenFilters
  isLoading: boolean
  error: string | null

  setTokens: (tokens: TokenInfo[]) => void
  addToken: (token: TokenInfo) => void
  updateToken: (mint: string, updates: Partial<TokenInfo>) => void
  removeToken: (mint: string) => void
  selectToken: (token: TokenInfo | null) => void
  setFilters: (filters: Partial<TokenFilters>) => void
  resetFilters: () => void
  setLoading: (isLoading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void
}

const defaultFilters: TokenFilters = {
  min_liquidity: 50,
  min_risk_score: 70,
  max_age_hours: 24,
  only_renounced: false,
  only_immutable: false,
  sort_by: 'liquidity',
  order: 'desc',
  limit: 50,
  offset: 0,
}

export const useTokenStore = create<TokenState>((set) => ({
  tokens: [],
  selectedToken: null,
  filters: defaultFilters,
  isLoading: false,
  error: null,

  setTokens: (tokens) => set({ tokens }),

  addToken: (token) =>
    set((state) => ({
      tokens: [token, ...state.tokens],
    })),

  updateToken: (mint, updates) =>
    set((state) => ({
      tokens: state.tokens.map((token) =>
        token.mint === mint ? { ...token, ...updates } : token
      ),
      selectedToken:
        state.selectedToken?.mint === mint
          ? { ...state.selectedToken, ...updates }
          : state.selectedToken,
    })),

  removeToken: (mint) =>
    set((state) => ({
      tokens: state.tokens.filter((token) => token.mint !== mint),
      selectedToken:
        state.selectedToken?.mint === mint ? null : state.selectedToken,
    })),

  selectToken: (token) => set({ selectedToken: token }),

  setFilters: (filters) =>
    set((state) => ({
      filters: { ...state.filters, ...filters },
    })),

  resetFilters: () => set({ filters: defaultFilters }),

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) => set({ error }),

  clearError: () => set({ error: null }),
}))
