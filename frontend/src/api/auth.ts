import client from './client'
import type { LoginRequest, LoginResponse } from '@/types/api'

export const authApi = {
  /**
   * Login with credentials
   */
  login: async (credentials: LoginRequest): Promise<LoginResponse> => {
    return client.post('/auth/login', credentials)
  },

  /**
   * Logout (invalidate token)
   */
  logout: async (): Promise<void> => {
    return client.post('/auth/logout')
  },

  /**
   * Refresh access token
   */
  refresh: async (): Promise<LoginResponse> => {
    return client.post('/auth/refresh')
  },

  /**
   * Verify token validity
   */
  verify: async (): Promise<{ valid: boolean }> => {
    return client.get('/auth/verify')
  },
}
