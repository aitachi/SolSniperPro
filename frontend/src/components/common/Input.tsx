import React, { forwardRef } from 'react'
import { cn } from '@/utils/helpers'

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string
  error?: string
  helperText?: string
  required?: boolean
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, helperText, required, className, ...props }, ref) => {
    return (
      <div className="w-full">
        {label && (
          <label className={cn('label', required && 'label-required')}>
            {label}
          </label>
        )}
        <input
          ref={ref}
          className={cn('input', error && 'input-error', className)}
          {...props}
        />
        {error && <p className="mt-1 text-sm text-danger-500">{error}</p>}
        {helperText && !error && (
          <p className="mt-1 text-sm text-gray-500">{helperText}</p>
        )}
      </div>
    )
  }
)

Input.displayName = 'Input'

interface TextareaProps
  extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string
  error?: string
  helperText?: string
  required?: boolean
}

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ label, error, helperText, required, className, ...props }, ref) => {
    return (
      <div className="w-full">
        {label && (
          <label className={cn('label', required && 'label-required')}>
            {label}
          </label>
        )}
        <textarea
          ref={ref}
          className={cn(
            'input resize-none',
            error && 'input-error',
            className
          )}
          {...props}
        />
        {error && <p className="mt-1 text-sm text-danger-500">{error}</p>}
        {helperText && !error && (
          <p className="mt-1 text-sm text-gray-500">{helperText}</p>
        )}
      </div>
    )
  }
)

Textarea.displayName = 'Textarea'

interface SelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
  label?: string
  error?: string
  helperText?: string
  required?: boolean
  options: Array<{ value: string | number; label: string }>
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(
  (
    { label, error, helperText, required, options, className, ...props },
    ref
  ) => {
    return (
      <div className="w-full">
        {label && (
          <label className={cn('label', required && 'label-required')}>
            {label}
          </label>
        )}
        <select
          ref={ref}
          className={cn('input', error && 'input-error', className)}
          {...props}
        >
          {options.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
        {error && <p className="mt-1 text-sm text-danger-500">{error}</p>}
        {helperText && !error && (
          <p className="mt-1 text-sm text-gray-500">{helperText}</p>
        )}
      </div>
    )
  }
)

Select.displayName = 'Select'
