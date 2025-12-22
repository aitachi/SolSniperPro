import React from 'react'
import { useMetricsSummary } from '@/hooks/useMetrics'
import { usePositions } from '@/hooks/usePositions'
import { useTrades } from '@/hooks/useTrades'
import { Card, CardHeader } from '@/components/common/Card'
import { Loading } from '@/components/common/Loading'
import { Badge } from '@/components/common/Badge'
import {
  TrendingUp,
  TrendingDown,
  DollarSign,
  Activity,
  Wallet,
  Target,
} from 'lucide-react'
import { formatUSD, formatSOL, formatPercentage, formatRelativeTime } from '@/utils/format'
import { getPnLColor } from '@/utils/helpers'

export const Dashboard: React.FC = () => {
  const { data: metrics, isLoading: metricsLoading } = useMetricsSummary()
  const { data: positions } = usePositions()
  const { data: recentTrades } = useTrades({ limit: 10 })

  if (metricsLoading) {
    return <Loading fullscreen message="Loading dashboard..." />
  }

  const tradingMetrics = metrics?.trading_metrics

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold mb-2">Dashboard</h1>
        <p className="text-gray-400">
          Welcome back! Here's your trading overview
        </p>
      </div>

      {/* Metrics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total PnL</p>
              <p className={`metric-value ${getPnLColor(tradingMetrics?.total_pnl_usd || 0)}`}>
                {formatUSD(tradingMetrics?.total_pnl_usd || 0)}
              </p>
              <p className="text-sm text-gray-400 mt-1">
                {formatSOL(tradingMetrics?.total_pnl_sol || 0)}
              </p>
            </div>
            <div className={`p-3 rounded-lg ${tradingMetrics?.total_pnl_usd > 0 ? 'bg-success-500/20' : 'bg-danger-500/20'}`}>
              <DollarSign size={24} className={tradingMetrics?.total_pnl_usd > 0 ? 'text-success-500' : 'text-danger-500'} />
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Win Rate</p>
              <p className="metric-value text-primary-500">
                {formatPercentage(tradingMetrics?.win_rate || 0, 1)}
              </p>
              <p className="text-sm text-gray-400 mt-1">
                {tradingMetrics?.successful_trades || 0} / {tradingMetrics?.total_trades || 0} trades
              </p>
            </div>
            <div className="p-3 rounded-lg bg-primary-500/20">
              <Target size={24} className="text-primary-500" />
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Active Positions</p>
              <p className="metric-value text-white">
                {positions?.length || 0}
              </p>
              <p className="text-sm text-gray-400 mt-1">
                {formatUSD(positions?.reduce((sum, p) => sum + p.current_value_usd, 0) || 0)}
              </p>
            </div>
            <div className="p-3 rounded-lg bg-warning-500/20">
              <Wallet size={24} className="text-warning-500" />
            </div>
          </div>
        </Card>

        <Card>
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total Trades</p>
              <p className="metric-value text-white">
                {tradingMetrics?.total_trades || 0}
              </p>
              <p className="text-sm text-gray-400 mt-1">
                Profit Factor: {tradingMetrics?.profit_factor?.toFixed(2) || '0.00'}
              </p>
            </div>
            <div className="p-3 rounded-lg bg-purple-500/20">
              <Activity size={24} className="text-purple-500" />
            </div>
          </div>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Active Positions */}
        <Card>
          <CardHeader title="Active Positions" subtitle={`${positions?.length || 0} open positions`} />
          <div className="space-y-3">
            {positions?.slice(0, 5).map((position) => (
              <div
                key={position.id}
                className="p-3 bg-dark-700/50 rounded-lg hover:bg-dark-700 transition-colors"
              >
                <div className="flex items-center justify-between mb-2">
                  <div>
                    <p className="font-semibold">{position.token_symbol}</p>
                    <p className="text-xs text-gray-400">{position.strategy_name}</p>
                  </div>
                  <Badge
                    variant={position.pnl_usd >= 0 ? 'success' : 'danger'}
                  >
                    {formatPercentage(position.pnl_percentage, 1, true)}
                  </Badge>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">PnL:</span>
                  <span className={getPnLColor(position.pnl_usd)}>
                    {formatUSD(position.pnl_usd)}
                  </span>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Value:</span>
                  <span>{formatUSD(position.current_value_usd)}</span>
                </div>
              </div>
            ))}
            {(!positions || positions.length === 0) && (
              <p className="text-center text-gray-400 py-8">
                No active positions
              </p>
            )}
          </div>
        </Card>

        {/* Recent Trades */}
        <Card>
          <CardHeader title="Recent Trades" subtitle="Latest 10 trades" />
          <div className="space-y-3">
            {recentTrades?.data?.slice(0, 5).map((trade) => (
              <div
                key={trade.id}
                className="p-3 bg-dark-700/50 rounded-lg hover:bg-dark-700 transition-colors"
              >
                <div className="flex items-center justify-between mb-2">
                  <div>
                    <p className="font-semibold">
                      {trade.side === 'BUY' ? (
                        <TrendingUp className="inline text-success-500 mr-1" size={16} />
                      ) : (
                        <TrendingDown className="inline text-danger-500 mr-1" size={16} />
                      )}
                      {trade.token_symbol}
                    </p>
                    <p className="text-xs text-gray-400">
                      {formatRelativeTime(trade.created_at)}
                    </p>
                  </div>
                  <Badge variant={trade.status === 'COMPLETED' ? 'success' : trade.status === 'FAILED' ? 'danger' : 'gray'}>
                    {trade.status}
                  </Badge>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Amount:</span>
                  <span>{formatUSD(trade.amount_usd)}</span>
                </div>
                {trade.pnl_usd !== undefined && (
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-gray-400">PnL:</span>
                    <span className={getPnLColor(trade.pnl_usd)}>
                      {formatUSD(trade.pnl_usd)}
                    </span>
                  </div>
                )}
              </div>
            ))}
            {(!recentTrades?.data || recentTrades.data.length === 0) && (
              <p className="text-center text-gray-400 py-8">
                No recent trades
              </p>
            )}
          </div>
        </Card>
      </div>
    </div>
  )
}
