import React, { useState } from 'react'
import { usePerformance } from '@/hooks/useMetrics'
import { Card, CardHeader } from '@/components/common/Card'
import { Select } from '@/components/common/Input'
import { Loading } from '@/components/common/Loading'
import { formatUSD } from '@/utils/format'

export const Analytics: React.FC = () => {
  const [period, setPeriod] = useState<'1h' | '24h' | '7d' | '30d' | 'all'>('24h')
  const { data, isLoading } = usePerformance(period)

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold mb-2">Analytics</h1>
          <p className="text-gray-400">
            Analyze your trading performance and metrics
          </p>
        </div>
        <Select
          value={period}
          onChange={(e) => setPeriod(e.target.value as any)}
          options={[
            { value: '1h', label: '1 Hour' },
            { value: '24h', label: '24 Hours' },
            { value: '7d', label: '7 Days' },
            { value: '30d', label: '30 Days' },
            { value: 'all', label: 'All Time' },
          ]}
        />
      </div>

      {isLoading ? (
        <Loading message="Loading analytics..." />
      ) : (
        <>
          {/* Performance Summary */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <Card>
              <p className="metric-label">Total PnL</p>
              <p className={`metric-value ${(data?.summary?.total_pnl ?? 0) >= 0 ? 'text-success-500' : 'text-danger-500'}`}>
                {formatUSD(data?.summary?.total_pnl ?? 0)}
              </p>
            </Card>
            <Card>
              <p className="metric-label">Total Trades</p>
              <p className="metric-value">{data?.summary?.total_trades ?? 0}</p>
            </Card>
            <Card>
              <p className="metric-label">Best Day</p>
              <p className="metric-value text-success-500">
                {formatUSD(data?.summary?.best_day ?? 0)}
              </p>
            </Card>
            <Card>
              <p className="metric-label">Worst Day</p>
              <p className="metric-value text-danger-500">
                {formatUSD(data?.summary?.worst_day ?? 0)}
              </p>
            </Card>
          </div>

          {/* Performance Chart Placeholder */}
          <Card>
            <CardHeader title="Performance Over Time" />
            <div className="h-64 flex items-center justify-center text-gray-400">
              Chart visualization will be displayed here
              <br />
              {data?.data_points?.length || 0} data points available
            </div>
          </Card>
        </>
      )}
    </div>
  )
}
