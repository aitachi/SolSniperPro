import React from 'react'
import { cn } from '@/utils/helpers'

interface BadgeProps {
  children: React.ReactNode
  variant?: 'success' | 'danger' | 'warning' | 'primary' | 'gray'
  className?: string
}

export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = 'gray',
  className,
}) => {
  const variantClasses = {
    success: 'badge-success',
    danger: 'badge-danger',
    warning: 'badge-warning',
    primary: 'badge-primary',
    gray: 'badge-gray',
  }

  return (
    <span className={cn('badge', variantClasses[variant], className)}>
      {children}
    </span>
  )
}
