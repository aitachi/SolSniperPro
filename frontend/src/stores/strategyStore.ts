import { create } from 'zustand'
import type { Strategy } from '@/types/strategy'

interface StrategyState {
  strategies: Strategy[]
  selectedStrategy: Strategy | null
  isLoading: boolean
  error: string | null

  setStrategies: (strategies: Strategy[]) => void
  addStrategy: (strategy: Strategy) => void
  updateStrategy: (id: string, updates: Partial<Strategy>) => void
  removeStrategy: (id: string) => void
  toggleStrategy: (id: string) => void
  selectStrategy: (strategy: Strategy | null) => void
  updateStrategyPriority: (id: string, priority: number) => void
  setLoading: (isLoading: boolean) => void
  setError: (error: string | null) => void
  clearError: () => void
}

export const useStrategyStore = create<StrategyState>((set) => ({
  strategies: [],
  selectedStrategy: null,
  isLoading: false,
  error: null,

  setStrategies: (strategies) =>
    set({
      strategies: strategies.sort((a, b) => b.priority - a.priority),
    }),

  addStrategy: (strategy) =>
    set((state) => ({
      strategies: [...state.strategies, strategy].sort(
        (a, b) => b.priority - a.priority
      ),
    })),

  updateStrategy: (id, updates) =>
    set((state) => ({
      strategies: state.strategies
        .map((strategy) =>
          strategy.id === id ? { ...strategy, ...updates } : strategy
        )
        .sort((a, b) => b.priority - a.priority),
      selectedStrategy:
        state.selectedStrategy?.id === id
          ? { ...state.selectedStrategy, ...updates }
          : state.selectedStrategy,
    })),

  removeStrategy: (id) =>
    set((state) => ({
      strategies: state.strategies.filter((strategy) => strategy.id !== id),
      selectedStrategy:
        state.selectedStrategy?.id === id ? null : state.selectedStrategy,
    })),

  toggleStrategy: (id) =>
    set((state) => ({
      strategies: state.strategies.map((strategy) =>
        strategy.id === id
          ? { ...strategy, enabled: !strategy.enabled }
          : strategy
      ),
    })),

  selectStrategy: (strategy) => set({ selectedStrategy: strategy }),

  updateStrategyPriority: (id, priority) =>
    set((state) => ({
      strategies: state.strategies
        .map((strategy) =>
          strategy.id === id ? { ...strategy, priority } : strategy
        )
        .sort((a, b) => b.priority - a.priority),
    })),

  setLoading: (isLoading) => set({ isLoading }),

  setError: (error) => set({ error }),

  clearError: () => set({ error: null }),
}))
