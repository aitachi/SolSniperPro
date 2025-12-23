import client from './client'
import type { RiskConfig, RiskStats } from '@/types/risk'
// import type { BlacklistEntry, RiskAlert } from '@/types/risk'

export const riskApi = {
  /**
   * Get risk limits configuration
   * Backend endpoint: GET /api/v1/risk/limits
   */
  getConfig: async (): Promise<RiskConfig> => {
    return client.get('/risk/limits')
  },

  /**
   * Update risk limits configuration
   * Backend endpoint: PUT /api/v1/risk/limits
   */
  updateConfig: async (updates: Partial<RiskConfig>): Promise<RiskConfig> => {
    return client.put('/risk/limits', updates)
  },

  /**
   * Get current risk status
   * Backend endpoint: GET /api/v1/risk/status
   */
  getStats: async (): Promise<RiskStats> => {
    return client.get('/risk/status')
  },

  // Note: The following endpoints are not implemented in backend yet
  // Commenting out to prevent 404 errors

  // /**
  //  * Get blacklist entries
  //  */
  // getBlacklist: async (): Promise<BlacklistEntry[]> => {
  //   return client.get('/risk/blacklist')
  // },

  // /**
  //  * Add address to blacklist
  //  */
  // addToBlacklist: async (data: {
  //   address: string
  //   type: 'TOKEN' | 'CREATOR'
  //   reason: string
  // }): Promise<BlacklistEntry> => {
  //   return client.post('/risk/blacklist', data)
  // },

  // /**
  //  * Remove address from blacklist
  //  */
  // removeFromBlacklist: async (address: string): Promise<void> => {
  //   return client.delete(`/risk/blacklist/${address}`)
  // },

  // /**
  //  * Get risk alerts
  //  */
  // getAlerts: async (params?: {
  //   acknowledged?: boolean
  //   severity?: 'INFO' | 'WARNING' | 'CRITICAL'
  //   limit?: number
  // }): Promise<RiskAlert[]> => {
  //   return client.get('/risk/alerts', { params })
  // },

  // /**
  //  * Acknowledge risk alert
  //  */
  // acknowledgeAlert: async (id: string): Promise<RiskAlert> => {
  //   return client.post(`/risk/alerts/${id}/acknowledge`)
  // },

  // /**
  //  * Clear cooldown period
  //  */
  // clearCooldown: async (): Promise<void> => {
  //   return client.post('/risk/cooldown/clear')
  // },

  // /**
  //  * Validate trade against risk rules
  //  */
  // validateTrade: async (data: {
  //   token_mint: string
  //   amount_sol: number
  // }): Promise<{
  //   allowed: boolean
  //   violations: string[]
  //   warnings: string[]
  // }> => {
  //   return client.post('/risk/validate', data)
  // },
}
