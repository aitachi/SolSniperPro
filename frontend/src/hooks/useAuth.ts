import { useAuthStore } from '@/stores/authStore'
import type { LoginRequest } from '@/types/api'

export const useAuth = () => {
  const {
    token,
    user,
    isAuthenticated,
    isLoading,
    error,
    login,
    logout,
    clearError,
  } = useAuthStore()

  const handleLogin = async (credentials: LoginRequest) => {
    clearError()
    await login(credentials)
  }

  const handleLogout = () => {
    logout()
  }

  return {
    // State
    token,
    user,
    isAuthenticated,
    isLoading,
    error,

    // Actions
    login: handleLogin,
    logout: handleLogout,
    clearError,
  }
}
