import React from 'react'
import { useMetricsSummary, useSystemHealth } from '@/hooks/useMetrics'
import { Card } from '@/components/common/Card'
import { Loading } from '@/components/common/Loading'
import { formatUSD, formatPercentage } from '@/utils/format'

export const Analytics: React.FC = () => {
  const { data: metrics, isLoading: metricsLoading } = useMetricsSummary()
  const { data: system, isLoading: systemLoading } = useSystemHealth()

  if (metricsLoading || systemLoading) {
    return <Loading message="Loading analytics..." />
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">Analytics</h1>
        <p className="text-gray-400">
          Trading performance and system metrics
        </p>
      </div>

      {/* Trading Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card>
          <p className="text-sm text-gray-400 mb-2">Total PnL (USD)</p>
          <p className={`text-2xl font-bold ${(metrics?.trading_metrics?.total_pnl_usd ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}`}>
            {formatUSD(metrics?.trading_metrics?.total_pnl_usd ?? 0)}
          </p>
        </Card>

        <Card>
          <p className="text-sm text-gray-400 mb-2">Total PnL (SOL)</p>
          <p className={`text-2xl font-bold ${(metrics?.trading_metrics?.total_pnl_sol ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}`}>
            {(metrics?.trading_metrics?.total_pnl_sol ?? 0).toFixed(4)} SOL
          </p>
        </Card>

        <Card>
          <p className="text-sm text-gray-400 mb-2">Win Rate</p>
          <p className="text-2xl font-bold text-blue-500">
            {formatPercentage(metrics?.trading_metrics?.win_rate ?? 0)}
          </p>
        </Card>

        <Card>
          <p className="text-sm text-gray-400 mb-2">Total Trades</p>
          <p className="text-2xl font-bold">
            {metrics?.trading_metrics?.total_trades ?? 0}
          </p>
        </Card>
      </div>

      {/* Trade Breakdown */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <p className="text-sm text-gray-400 mb-2">Successful Trades</p>
          <p className="text-2xl font-bold text-green-500">
            {metrics?.trading_metrics?.successful_trades ?? 0}
          </p>
        </Card>

        <Card>
          <p className="text-sm text-gray-400 mb-2">Failed Trades</p>
          <p className="text-2xl font-bold text-red-500">
            {metrics?.trading_metrics?.failed_trades ?? 0}
          </p>
        </Card>

        <Card>
          <p className="text-sm text-gray-400 mb-2">Profit Factor</p>
          <p className="text-2xl font-bold">
            {(metrics?.trading_metrics?.profit_factor ?? 0).toFixed(2)}
          </p>
        </Card>
      </div>

      {/* System Metrics */}
      <div>
        <h2 className="text-2xl font-bold mb-4">System Health</h2>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          <Card>
            <p className="text-sm text-gray-400 mb-2">CPU Usage</p>
            <p className="text-2xl font-bold">
              {(system?.cpu_usage_percent ?? 0).toFixed(1)}%
            </p>
          </Card>

          <Card>
            <p className="text-sm text-gray-400 mb-2">Memory Usage</p>
            <p className="text-2xl font-bold">
              {system?.memory_usage_mb && system?.memory_total_mb
                ? ((system.memory_usage_mb / system.memory_total_mb) * 100).toFixed(1)
                : '0.0'}%
            </p>
          </Card>

          <Card>
            <p className="text-sm text-gray-400 mb-2">Event Latency</p>
            <p className="text-2xl font-bold">
              {(system?.event_latency_ms ?? 0).toFixed(1)} ms
            </p>
          </Card>

          <Card>
            <p className="text-sm text-gray-400 mb-2">Active Connections</p>
            <p className="text-2xl font-bold">
              {system?.active_connections ?? 0}
            </p>
          </Card>
        </div>
      </div>

      {/* Note about advanced features */}
      <Card>
        <div className="text-center py-8 text-gray-400">
          <p className="text-lg font-semibold mb-2">Advanced Analytics</p>
          <p>Performance charts and heatmaps will be available when backend implements these features.</p>
        </div>
      </Card>
    </div>
  )
}
