import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface Modal {
  id: string
  component: string
  props?: Record<string, any>
}

interface UIState {
  // Sidebar
  sidebarCollapsed: boolean
  toggleSidebar: () => void
  setSidebarCollapsed: (collapsed: boolean) => void

  // Modals
  modals: Modal[]
  openModal: (component: string, props?: Record<string, any>) => string
  closeModal: (id: string) => void
  closeAllModals: () => void

  // Loading states
  globalLoading: boolean
  setGlobalLoading: (loading: boolean) => void

  // Theme
  theme: 'light' | 'dark'
  toggleTheme: () => void
  setTheme: (theme: 'light' | 'dark') => void

  // Notifications
  soundEnabled: boolean
  notificationsEnabled: boolean
  toggleSound: () => void
  toggleNotifications: () => void

  // Auto refresh
  autoRefreshEnabled: boolean
  autoRefreshInterval: number
  toggleAutoRefresh: () => void
  setAutoRefreshInterval: (interval: number) => void

  // Currency preference
  currency: 'USD' | 'SOL'
  setCurrency: (currency: 'USD' | 'SOL') => void
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      // Sidebar
      sidebarCollapsed: false,
      toggleSidebar: () =>
        set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
      setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),

      // Modals
      modals: [],
      openModal: (component, props) => {
        const id = `modal-${Date.now()}`
        set((state) => ({
          modals: [...state.modals, { id, component, props }],
        }))
        return id
      },
      closeModal: (id) =>
        set((state) => ({
          modals: state.modals.filter((modal) => modal.id !== id),
        })),
      closeAllModals: () => set({ modals: [] }),

      // Loading
      globalLoading: false,
      setGlobalLoading: (loading) => set({ globalLoading: loading }),

      // Theme
      theme: 'dark',
      toggleTheme: () =>
        set((state) => ({
          theme: state.theme === 'dark' ? 'light' : 'dark',
        })),
      setTheme: (theme) => set({ theme }),

      // Notifications
      soundEnabled: true,
      notificationsEnabled: true,
      toggleSound: () =>
        set((state) => ({ soundEnabled: !state.soundEnabled })),
      toggleNotifications: () =>
        set((state) => ({
          notificationsEnabled: !state.notificationsEnabled,
        })),

      // Auto refresh
      autoRefreshEnabled: true,
      autoRefreshInterval: 5000,
      toggleAutoRefresh: () =>
        set((state) => ({
          autoRefreshEnabled: !state.autoRefreshEnabled,
        })),
      setAutoRefreshInterval: (interval) =>
        set({ autoRefreshInterval: interval }),

      // Currency
      currency: 'USD',
      setCurrency: (currency) => set({ currency }),
    }),
    {
      name: 'ui-storage',
      partialize: (state) => ({
        sidebarCollapsed: state.sidebarCollapsed,
        theme: state.theme,
        soundEnabled: state.soundEnabled,
        notificationsEnabled: state.notificationsEnabled,
        autoRefreshEnabled: state.autoRefreshEnabled,
        autoRefreshInterval: state.autoRefreshInterval,
        currency: state.currency,
      }),
    }
  )
)
