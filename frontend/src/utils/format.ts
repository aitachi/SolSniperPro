import { format as dateFnsFormat, formatDistanceToNow, differenceInHours, differenceInMinutes } from 'date-fns'

/**
 * Format a number as USD currency
 */
export function formatUSD(value: number, decimals: number = 2): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value)
}

/**
 * Format a number as SOL currency
 */
export function formatSOL(value: number, decimals: number = 4): string {
  return `${formatNumber(value, decimals)} SOL`
}

/**
 * Format a number with thousand separators
 */
export function formatNumber(value: number, decimals: number = 2): string {
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value)
}

/**
 * Format a number as a percentage
 */
export function formatPercentage(value: number | undefined | null, decimals: number = 2, showSign: boolean = false): string {
  if (value === undefined || value === null || isNaN(value)) {
    return '0.00%'
  }
  const sign = showSign && value > 0 ? '+' : ''
  return `${sign}${value.toFixed(decimals)}%`
}

/**
 * Format a large number with K, M, B suffixes
 */
export function formatCompact(value: number, decimals: number = 1): string {
  const absValue = Math.abs(value)
  const sign = value < 0 ? '-' : ''

  if (absValue >= 1e9) {
    return `${sign}${(absValue / 1e9).toFixed(decimals)}B`
  } else if (absValue >= 1e6) {
    return `${sign}${(absValue / 1e6).toFixed(decimals)}M`
  } else if (absValue >= 1e3) {
    return `${sign}${(absValue / 1e3).toFixed(decimals)}K`
  }
  return `${sign}${absValue.toFixed(decimals)}`
}

/**
 * Format a date/timestamp
 */
export function formatDate(date: string | Date, formatStr: string = 'MMM dd, yyyy HH:mm:ss'): string {
  const dateObj = typeof date === 'string' ? new Date(date) : date
  return dateFnsFormat(dateObj, formatStr)
}

/**
 * Format a date as relative time (e.g., "2 hours ago")
 */
export function formatRelativeTime(date: string | Date): string {
  const dateObj = typeof date === 'string' ? new Date(date) : date
  return formatDistanceToNow(dateObj, { addSuffix: true })
}

/**
 * Format duration in hours/minutes
 */
export function formatDuration(hours: number): string {
  if (hours < 1) {
    const minutes = Math.round(hours * 60)
    return `${minutes}m`
  } else if (hours < 24) {
    const h = Math.floor(hours)
    const m = Math.round((hours - h) * 60)
    return m > 0 ? `${h}h ${m}m` : `${h}h`
  } else {
    const days = Math.floor(hours / 24)
    const h = Math.floor(hours % 24)
    return h > 0 ? `${days}d ${h}h` : `${days}d`
  }
}

/**
 * Format a Solana address (truncate middle)
 */
export function formatAddress(address: string, startChars: number = 4, endChars: number = 4): string {
  if (address.length <= startChars + endChars) {
    return address
  }
  return `${address.slice(0, startChars)}...${address.slice(-endChars)}`
}

/**
 * Format basis points to percentage
 */
export function formatBps(bps: number): string {
  return formatPercentage(bps / 100, 2)
}

/**
 * Format token amount with decimals
 */
export function formatTokenAmount(amount: number, decimals: number = 9): string {
  const actualAmount = amount / Math.pow(10, decimals)
  return formatNumber(actualAmount, Math.min(decimals, 6))
}

/**
 * Format file size
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 Bytes'

  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(decimals))} ${sizes[i]}`
}

/**
 * Get time difference in hours
 */
export function getHoursDifference(date: string | Date): number {
  const dateObj = typeof date === 'string' ? new Date(date) : date
  return differenceInHours(new Date(), dateObj)
}

/**
 * Get time difference in minutes
 */
export function getMinutesDifference(date: string | Date): number {
  const dateObj = typeof date === 'string' ? new Date(date) : date
  return differenceInMinutes(new Date(), dateObj)
}
