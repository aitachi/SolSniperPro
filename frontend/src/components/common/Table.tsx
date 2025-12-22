import React from 'react'
import { cn } from '@/utils/helpers'
import type { TableColumn } from '@/types/api'

interface TableProps<T> {
  data: T[]
  columns: TableColumn<T>[]
  loading?: boolean
  emptyMessage?: string
  onRowClick?: (row: T) => void
  className?: string
}

export function Table<T extends Record<string, any>>({
  data,
  columns,
  loading = false,
  emptyMessage = 'No data available',
  onRowClick,
  className,
}: TableProps<T>) {
  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="spinner-lg" />
      </div>
    )
  }

  if (data.length === 0) {
    return (
      <div className="flex items-center justify-center py-12 text-gray-400">
        {emptyMessage}
      </div>
    )
  }

  return (
    <div className={cn('overflow-x-auto custom-scrollbar', className)}>
      <table className="table">
        <thead>
          <tr className="table-header">
            {columns.map((column) => (
              <th
                key={String(column.key)}
                className="table-cell"
                style={{ width: column.width }}
              >
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.map((row, rowIndex) => (
            <tr
              key={rowIndex}
              className={cn(
                'table-row',
                onRowClick && 'cursor-pointer'
              )}
              onClick={() => onRowClick?.(row)}
            >
              {columns.map((column) => {
                const value = column.key.toString().includes('.')
                  ? column.key.toString().split('.').reduce((obj, key) => obj?.[key], row)
                  : row[column.key]

                return (
                  <td key={String(column.key)} className="table-cell">
                    {column.render ? column.render(value, row) : value}
                  </td>
                )
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
