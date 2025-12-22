/**
 * Validation utility functions
 */

/**
 * Validate Solana address
 */
export function isValidSolanaAddress(address: string): boolean {
  // Basic validation: base58, 32-44 characters
  const base58Regex = /^[1-9A-HJ-NP-Za-km-z]{32,44}$/
  return base58Regex.test(address)
}

/**
 * Validate number is positive
 */
export function isPositiveNumber(value: number): boolean {
  return typeof value === 'number' && !isNaN(value) && value > 0
}

/**
 * Validate number is in range
 */
export function isInRange(value: number, min: number, max: number): boolean {
  return typeof value === 'number' && !isNaN(value) && value >= min && value <= max
}

/**
 * Validate percentage (0-100)
 */
export function isValidPercentage(value: number): boolean {
  return isInRange(value, 0, 100)
}

/**
 * Validate basis points (0-10000)
 */
export function isValidBps(value: number): boolean {
  return isInRange(value, 0, 10000)
}

/**
 * Validate email address
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

/**
 * Validate URL
 */
export function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

/**
 * Validate strategy name
 */
export function isValidStrategyName(name: string): boolean {
  // 3-50 characters, alphanumeric with spaces, hyphens, underscores
  const nameRegex = /^[a-zA-Z0-9\s\-_]{3,50}$/
  return nameRegex.test(name)
}

/**
 * Validate token symbol
 */
export function isValidTokenSymbol(symbol: string): boolean {
  // 1-10 characters, uppercase letters and numbers
  const symbolRegex = /^[A-Z0-9]{1,10}$/
  return symbolRegex.test(symbol)
}

/**
 * Validate required field
 */
export function isRequired(value: any): boolean {
  if (value === null || value === undefined) return false
  if (typeof value === 'string') return value.trim().length > 0
  if (typeof value === 'number') return !isNaN(value)
  if (Array.isArray(value)) return value.length > 0
  return true
}

/**
 * Validate minimum length
 */
export function minLength(value: string, min: number): boolean {
  return typeof value === 'string' && value.length >= min
}

/**
 * Validate maximum length
 */
export function maxLength(value: string, max: number): boolean {
  return typeof value === 'string' && value.length <= max
}

/**
 * Validate password strength
 */
export function isStrongPassword(password: string): boolean {
  // At least 8 characters, 1 uppercase, 1 lowercase, 1 number
  if (password.length < 8) return false
  if (!/[A-Z]/.test(password)) return false
  if (!/[a-z]/.test(password)) return false
  if (!/[0-9]/.test(password)) return false
  return true
}

/**
 * Validate object has required keys
 */
export function hasRequiredKeys<T extends object>(
  obj: T,
  requiredKeys: (keyof T)[]
): boolean {
  return requiredKeys.every(key => key in obj && isRequired(obj[key]))
}

/**
 * Sanitize string input
 */
export function sanitizeString(value: string): string {
  return value.trim().replace(/[<>]/g, '')
}

/**
 * Validate JSON string
 */
export function isValidJSON(value: string): boolean {
  try {
    JSON.parse(value)
    return true
  } catch {
    return false
  }
}
