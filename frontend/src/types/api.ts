// API Response Types
export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: ApiError
  timestamp: string
}

export interface ApiError {
  code: string
  message: string
  details?: Record<string, any>
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: Pagination
}

export interface Pagination {
  total: number
  limit: number
  offset: number
  has_more: boolean
}

// WebSocket Types
export interface WebSocketMessage {
  type: 'event'
  topic: string
  event: string
  data: any
  timestamp: string
}

export interface WebSocketSubscription {
  type: 'subscribe' | 'unsubscribe'
  topics: string[]
}

export type WebSocketTopic =
  | 'tokens'
  | 'trades'
  | 'positions'
  | 'strategies'
  | 'risk'
  | 'metrics'
  | 'system'

export type WebSocketEvent =
  | 'new_token'
  | 'token_update'
  | 'trade_created'
  | 'trade_executed'
  | 'trade_failed'
  | 'position_opened'
  | 'position_updated'
  | 'position_closed'
  | 'strategy_triggered'
  | 'strategy_updated'
  | 'risk_alert'
  | 'risk_violation'
  | 'metrics_update'
  | 'system_status'

// Auth Types
export interface LoginRequest {
  username: string
  password: string
}

export interface LoginResponse {
  token: string
  expires_at: string
  user: User
}

export interface User {
  id: string
  username: string
  role: 'ADMIN' | 'TRADER' | 'VIEWER'
  created_at: string
}

// Settings Types
export interface AppSettings {
  theme: 'light' | 'dark'
  notifications_enabled: boolean
  sound_enabled: boolean
  auto_refresh_interval: number
  default_slippage_bps: number
  default_priority_fee: number
  currency: 'USD' | 'SOL'
  language: 'en' | 'zh'
}

// Common Utility Types
export type SortOrder = 'asc' | 'desc'

export interface SortConfig {
  field: string
  order: SortOrder
}

export interface FilterConfig {
  [key: string]: any
}

export interface TableColumn<T> {
  key: keyof T | string
  label: string
  sortable?: boolean
  render?: (value: any, row: T) => React.ReactNode
  width?: string
}
