import React from 'react'
import { cn } from '@/utils/helpers'

interface LoadingProps {
  size?: 'sm' | 'md' | 'lg'
  fullscreen?: boolean
  message?: string
}

export const Loading: React.FC<LoadingProps> = ({
  size = 'md',
  fullscreen = false,
  message,
}) => {
  const sizeClasses = {
    sm: 'spinner',
    md: 'spinner-lg',
    lg: 'w-12 h-12 border-4',
  }

  const content = (
    <div className="flex flex-col items-center justify-center gap-3">
      <div className={cn('spinner', sizeClasses[size])} />
      {message && <p className="text-gray-400 text-sm">{message}</p>}
    </div>
  )

  if (fullscreen) {
    return (
      <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
        {content}
      </div>
    )
  }

  return content
}

interface SkeletonProps {
  className?: string
  count?: number
}

export const Skeleton: React.FC<SkeletonProps> = ({
  className,
  count = 1,
}) => {
  return (
    <>
      {Array.from({ length: count }).map((_, i) => (
        <div
          key={i}
          className={cn(
            'animate-pulse bg-dark-700 rounded',
            className
          )}
        />
      ))}
    </>
  )
}
