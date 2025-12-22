import clsx, { ClassValue } from 'clsx'

/**
 * Merge class names using clsx
 */
export function cn(...classes: ClassValue[]): string {
  return clsx(classes)
}

/**
 * Sleep/delay function
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

/**
 * Debounce function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null

  return function (...args: Parameters<T>) {
    if (timeout) clearTimeout(timeout)
    timeout = setTimeout(() => func(...args), wait)
  }
}

/**
 * Throttle function
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean = false

  return function (...args: Parameters<T>) {
    if (!inThrottle) {
      func(...args)
      inThrottle = true
      setTimeout(() => (inThrottle = false), limit)
    }
  }
}

/**
 * Deep clone an object
 */
export function deepClone<T>(obj: T): T {
  return JSON.parse(JSON.stringify(obj))
}

/**
 * Check if object is empty
 */
export function isEmpty(obj: any): boolean {
  if (obj === null || obj === undefined) return true
  if (typeof obj === 'string') return obj.trim().length === 0
  if (Array.isArray(obj)) return obj.length === 0
  if (typeof obj === 'object') return Object.keys(obj).length === 0
  return false
}

/**
 * Get color for PnL value
 */
export function getPnLColor(pnl: number): string {
  if (pnl > 0) return 'text-success-500'
  if (pnl < 0) return 'text-danger-500'
  return 'text-gray-400'
}

/**
 * Get color for risk level
 */
export function getRiskLevelColor(level: string): string {
  const colorMap: Record<string, string> = {
    LOW: 'text-success-500',
    MEDIUM: 'text-warning-500',
    HIGH: 'text-danger-500',
    EXTREME: 'text-danger-600',
  }
  return colorMap[level] || 'text-gray-400'
}

/**
 * Get badge color for trade status
 */
export function getTradeStatusColor(status: string): string {
  const colorMap: Record<string, string> = {
    PENDING: 'badge-gray',
    SIMULATING: 'badge-primary',
    EXECUTING: 'badge-warning',
    COMPLETED: 'badge-success',
    FAILED: 'badge-danger',
    CANCELLED: 'badge-gray',
  }
  return colorMap[status] || 'badge-gray'
}

/**
 * Get badge color for position status
 */
export function getPositionStatusColor(status: string): string {
  const colorMap: Record<string, string> = {
    ACTIVE: 'badge-success',
    CLOSING: 'badge-warning',
    CLOSED: 'badge-gray',
  }
  return colorMap[status] || 'badge-gray'
}

/**
 * Calculate percentage change
 */
export function calculatePercentageChange(oldValue: number, newValue: number): number {
  if (oldValue === 0) return 0
  return ((newValue - oldValue) / oldValue) * 100
}

/**
 * Copy text to clipboard
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch {
    return false
  }
}

/**
 * Open URL in new tab
 */
export function openInNewTab(url: string): void {
  window.open(url, '_blank', 'noopener,noreferrer')
}

/**
 * Get Solscan URL for address
 */
export function getSolscanUrl(address: string, type: 'address' | 'tx' = 'address'): string {
  return `https://solscan.io/${type}/${address}`
}

/**
 * Group array by key
 */
export function groupBy<T>(array: T[], key: keyof T): Record<string, T[]> {
  return array.reduce((result, item) => {
    const groupKey = String(item[key])
    if (!result[groupKey]) {
      result[groupKey] = []
    }
    result[groupKey].push(item)
    return result
  }, {} as Record<string, T[]>)
}

/**
 * Sort array by key
 */
export function sortBy<T>(
  array: T[],
  key: keyof T,
  order: 'asc' | 'desc' = 'asc'
): T[] {
  return [...array].sort((a, b) => {
    const aVal = a[key]
    const bVal = b[key]

    if (aVal === bVal) return 0

    let comparison = 0
    if (typeof aVal === 'string' && typeof bVal === 'string') {
      comparison = aVal.localeCompare(bVal)
    } else if (typeof aVal === 'number' && typeof bVal === 'number') {
      comparison = aVal - bVal
    } else {
      comparison = String(aVal).localeCompare(String(bVal))
    }

    return order === 'asc' ? comparison : -comparison
  })
}

/**
 * Filter array by multiple conditions
 */
export function filterBy<T>(
  array: T[],
  filters: Partial<Record<keyof T, any>>
): T[] {
  return array.filter(item =>
    Object.entries(filters).every(([key, value]) => {
      if (value === undefined || value === null || value === '') return true
      return item[key as keyof T] === value
    })
  )
}

/**
 * Paginate array
 */
export function paginate<T>(
  array: T[],
  page: number,
  pageSize: number
): { data: T[]; total: number; hasMore: boolean } {
  const start = (page - 1) * pageSize
  const end = start + pageSize
  const data = array.slice(start, end)

  return {
    data,
    total: array.length,
    hasMore: end < array.length,
  }
}

/**
 * Generate unique ID
 */
export function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
}

/**
 * Safe JSON parse
 */
export function safeJsonParse<T>(json: string, fallback: T): T {
  try {
    return JSON.parse(json)
  } catch {
    return fallback
  }
}

/**
 * Calculate win rate
 */
export function calculateWinRate(wins: number, total: number): number {
  if (total === 0) return 0
  return (wins / total) * 100
}

/**
 * Calculate profit factor
 */
export function calculateProfitFactor(totalProfit: number, totalLoss: number): number {
  if (totalLoss === 0) return totalProfit > 0 ? Infinity : 0
  return Math.abs(totalProfit / totalLoss)
}

/**
 * Clamp number between min and max
 */
export function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max)
}

/**
 * Round to decimal places
 */
export function roundTo(value: number, decimals: number): number {
  const multiplier = Math.pow(10, decimals)
  return Math.round(value * multiplier) / multiplier
}

/**
 * Check if value is between range
 */
export function isBetween(value: number, min: number, max: number): boolean {
  return value >= min && value <= max
}
