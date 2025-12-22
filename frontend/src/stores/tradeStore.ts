import { create } from 'zustand'
import type { Trade, Position, TradeFilters } from '@/types/trade'

interface TradeState {
  trades: Trade[]
  positions: Position[]
  selectedTrade: Trade | null
  selectedPosition: Position | null
  filters: TradeFilters
  isLoading: boolean
  error: string | null

  setTrades: (trades: Trade[]) => void
  addTrade: (trade: Trade) => void
  updateTrade: (id: string, updates: Partial<Trade>) => void
  removeTrade: (id: string) => void
  selectTrade: (trade: Trade | null) => void

  setPositions: (positions: Position[]) => void
  addPosition: (position: Position) => void
  updatePosition: (id: string, updates: Partial<Position>) => void
  removePosition: (id: string) => void
  selectPosition: (position: Position | null) => void

  setFilters: (filters: Partial<TradeFilters>) => void
  resetFilters: () => void

  setLoading: (isLoading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void
}

const defaultFilters: TradeFilters = {
  status: undefined,
  side: undefined,
  strategy_name: undefined,
  sort_by: 'created_at',
  order: 'desc',
  limit: 50,
  offset: 0,
}

export const useTradeStore = create<TradeState>((set) => ({
  trades: [],
  positions: [],
  selectedTrade: null,
  selectedPosition: null,
  filters: defaultFilters,
  isLoading: false,
  error: null,

  setTrades: (trades) => set({ trades }),

  addTrade: (trade) =>
    set((state) => ({
      trades: [trade, ...state.trades],
    })),

  updateTrade: (id, updates) =>
    set((state) => ({
      trades: state.trades.map((trade) =>
        trade.id === id ? { ...trade, ...updates } : trade
      ),
      selectedTrade:
        state.selectedTrade?.id === id
          ? { ...state.selectedTrade, ...updates }
          : state.selectedTrade,
    })),

  removeTrade: (id) =>
    set((state) => ({
      trades: state.trades.filter((trade) => trade.id !== id),
      selectedTrade:
        state.selectedTrade?.id === id ? null : state.selectedTrade,
    })),

  selectTrade: (trade) => set({ selectedTrade: trade }),

  setPositions: (positions) => set({ positions }),

  addPosition: (position) =>
    set((state) => ({
      positions: [position, ...state.positions],
    })),

  updatePosition: (id, updates) =>
    set((state) => ({
      positions: state.positions.map((position) =>
        position.id === id ? { ...position, ...updates } : position
      ),
      selectedPosition:
        state.selectedPosition?.id === id
          ? { ...state.selectedPosition, ...updates }
          : state.selectedPosition,
    })),

  removePosition: (id) =>
    set((state) => ({
      positions: state.positions.filter((position) => position.id !== id),
      selectedPosition:
        state.selectedPosition?.id === id ? null : state.selectedPosition,
    })),

  selectPosition: (position) => set({ selectedPosition: position }),

  setFilters: (filters) =>
    set((state) => ({
      filters: { ...state.filters, ...filters },
    })),

  resetFilters: () => set({ filters: defaultFilters }),

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) => set({ error }),

  clearError: () => set({ error: null }),
}))
