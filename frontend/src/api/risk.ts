import client from './client'
import type { RiskConfig, RiskStats, BlacklistEntry, RiskAlert } from '@/types/risk'

export const riskApi = {
  /**
   * Get risk configuration
   */
  getConfig: async (): Promise<RiskConfig> => {
    return client.get('/risk/config')
  },

  /**
   * Update risk configuration
   */
  updateConfig: async (updates: Partial<RiskConfig>): Promise<RiskConfig> => {
    return client.put('/risk/config', updates)
  },

  /**
   * Get risk statistics
   */
  getStats: async (): Promise<RiskStats> => {
    return client.get('/risk/stats')
  },

  /**
   * Get blacklist entries
   */
  getBlacklist: async (): Promise<BlacklistEntry[]> => {
    return client.get('/risk/blacklist')
  },

  /**
   * Add address to blacklist
   */
  addToBlacklist: async (data: {
    address: string
    type: 'TOKEN' | 'CREATOR'
    reason: string
  }): Promise<BlacklistEntry> => {
    return client.post('/risk/blacklist', data)
  },

  /**
   * Remove address from blacklist
   */
  removeFromBlacklist: async (address: string): Promise<void> => {
    return client.delete(`/risk/blacklist/${address}`)
  },

  /**
   * Get risk alerts
   */
  getAlerts: async (params?: {
    acknowledged?: boolean
    severity?: 'INFO' | 'WARNING' | 'CRITICAL'
    limit?: number
  }): Promise<RiskAlert[]> => {
    return client.get('/risk/alerts', { params })
  },

  /**
   * Acknowledge risk alert
   */
  acknowledgeAlert: async (id: string): Promise<RiskAlert> => {
    return client.post(`/risk/alerts/${id}/acknowledge`)
  },

  /**
   * Clear cooldown period
   */
  clearCooldown: async (): Promise<void> => {
    return client.post('/risk/cooldown/clear')
  },

  /**
   * Validate trade against risk rules
   */
  validateTrade: async (data: {
    token_mint: string
    amount_sol: number
  }): Promise<{
    allowed: boolean
    violations: string[]
    warnings: string[]
  }> => {
    return client.post('/risk/validate', data)
  },
}
