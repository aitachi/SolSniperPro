import React, { useState } from 'react'
import { useMetricsSummary } from '@/hooks/useMetrics'
import { useStrategies } from '@/hooks/useStrategies'
import { usePositions } from '@/hooks/usePositions'
import { useTrades } from '@/hooks/useTrades'
import { Card, CardHeader } from '@/components/common/Card'
import { Badge } from '@/components/common/Badge'
import { Button } from '@/components/common/Button'
import {
  Activity,
  TrendingUp,
  TrendingDown,
  Play,
  Pause,
  CheckCircle,
  XCircle,
  RefreshCw,
  DollarSign,
  Wallet,
  Target,
  Clock,
  Cpu,
  Database,
  Wifi,
} from 'lucide-react'
import { formatUSD, formatPercentage, formatRelativeTime } from '@/utils/format'
import { getPnLColor } from '@/utils/helpers'

export const CommandCenter: React.FC = () => {
  const { data: metrics } = useMetricsSummary()
  const { data: strategies } = useStrategies()
  const { data: positions } = usePositions()
  const { data: recentTrades } = useTrades({ limit: 5 })
  const [autoRefresh, setAutoRefresh] = useState(true)

  const activeStrategies = strategies?.filter(s => s.is_active) || []
  const tradingMetrics = metrics?.trading_metrics

  return (
    <div className="space-y-6 p-6 bg-dark-950">
      {/* Header with System Status */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-4xl font-bold text-gradient mb-2">🎯 Control Center</h1>
          <p className="text-gray-400 text-lg">Command & Monitor Your Trading Operations</p>
        </div>
        <div className="flex items-center gap-3">
          <Button
            variant={autoRefresh ? 'primary' : 'secondary'}
            size="sm"
            onClick={() => setAutoRefresh(!autoRefresh)}
          >
            <RefreshCw size={16} className={autoRefresh ? 'animate-spin' : ''} />
            {autoRefresh ? 'Auto-Refresh ON' : 'Auto-Refresh OFF'}
          </Button>
        </div>
      </div>

      {/* System Status Bar */}
      <div className="grid grid-cols-4 gap-4">
        <Card className="bg-gradient-to-br from-success-500/20 to-dark-800 border-success-500/30">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-success-500/30 rounded-lg">
              <Cpu size={24} className="text-success-500" />
            </div>
            <div>
              <p className="text-xs text-success-300">System Status</p>
              <p className="text-lg font-bold text-success-500">ONLINE</p>
            </div>
            <div className="ml-auto">
              <div className="w-3 h-3 bg-success-500 rounded-full animate-pulse"></div>
            </div>
          </div>
        </Card>

        <Card className="bg-gradient-to-br from-primary-500/20 to-dark-800 border-primary-500/30">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-primary-500/30 rounded-lg">
              <Database size={24} className="text-primary-500" />
            </div>
            <div>
              <p className="text-xs text-primary-300">API Status</p>
              <p className="text-lg font-bold text-primary-500">CONNECTED</p>
            </div>
            <div className="ml-auto">
              <CheckCircle size={20} className="text-primary-500" />
            </div>
          </div>
        </Card>

        <Card className="bg-gradient-to-br from-warning-500/20 to-dark-800 border-warning-500/30">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-warning-500/30 rounded-lg">
              <Wifi size={24} className="text-warning-500" />
            </div>
            <div>
              <p className="text-xs text-warning-300">Network</p>
              <p className="text-lg font-bold text-warning-500">STABLE</p>
            </div>
            <div className="ml-auto">
              <div className="flex gap-1">
                <div className="w-1 h-4 bg-warning-500 rounded"></div>
                <div className="w-1 h-5 bg-warning-500 rounded"></div>
                <div className="w-1 h-6 bg-warning-500 rounded"></div>
              </div>
            </div>
          </div>
        </Card>

        <Card className="bg-gradient-to-br from-purple-500/20 to-dark-800 border-purple-500/30">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-purple-500/30 rounded-lg">
              <Target size={24} className="text-purple-500" />
            </div>
            <div>
              <p className="text-xs text-purple-300">Active Strategies</p>
              <p className="text-lg font-bold text-purple-500">{activeStrategies.length}/{strategies?.length || 0}</p>
            </div>
          </div>
        </Card>
      </div>

      {/* Main Control Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Performance Metrics */}
        <div className="lg:col-span-2 space-y-6">
          {/* Key Metrics */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <Card className="bg-gradient-to-br from-dark-800 to-dark-900 border-primary-500/30 hover:shadow-glow transition-all">
              <div className="text-center">
                <DollarSign size={32} className="mx-auto mb-2 text-primary-500" />
                <p className="text-xs text-gray-400 uppercase mb-1">Total PnL</p>
                <p className={`text-2xl font-bold ${getPnLColor(tradingMetrics?.total_pnl_usd || 0)}`}>
                  {formatUSD(tradingMetrics?.total_pnl_usd || 0)}
                </p>
              </div>
            </Card>

            <Card className="bg-gradient-to-br from-dark-800 to-dark-900 border-success-500/30 hover:shadow-glow transition-all">
              <div className="text-center">
                <Target size={32} className="mx-auto mb-2 text-success-500" />
                <p className="text-xs text-gray-400 uppercase mb-1">Win Rate</p>
                <p className="text-2xl font-bold text-success-500">
                  {formatPercentage(tradingMetrics?.win_rate || 0, 1)}
                </p>
              </div>
            </Card>

            <Card className="bg-gradient-to-br from-dark-800 to-dark-900 border-warning-500/30 hover:shadow-glow transition-all">
              <div className="text-center">
                <Wallet size={32} className="mx-auto mb-2 text-warning-500" />
                <p className="text-xs text-gray-400 uppercase mb-1">Positions</p>
                <p className="text-2xl font-bold text-warning-500">{positions?.length || 0}</p>
              </div>
            </Card>

            <Card className="bg-gradient-to-br from-dark-800 to-dark-900 border-purple-500/30 hover:shadow-glow transition-all">
              <div className="text-center">
                <Activity size={32} className="mx-auto mb-2 text-purple-500" />
                <p className="text-xs text-gray-400 uppercase mb-1">Trades</p>
                <p className="text-2xl font-bold text-purple-500">
                  {tradingMetrics?.total_trades || 0}
                </p>
              </div>
            </Card>
          </div>

          {/* Active Positions Monitor */}
          <Card>
            <CardHeader
              title="🎯 Active Positions Monitor"
              subtitle="Real-time position tracking"
            />
            <div className="space-y-2">
              {positions?.slice(0, 4).map((position) => (
                <div
                  key={position.id}
                  className="p-4 bg-gradient-to-r from-dark-700/50 to-dark-800/50 rounded-lg border border-dark-600 hover:border-primary-500/50 transition-all"
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-3">
                      <div className={`w-2 h-2 rounded-full ${position.pnl_usd >= 0 ? 'bg-success-500' : 'bg-danger-500'} animate-pulse`}></div>
                      <div>
                        <p className="font-bold text-lg">{position.token_symbol}</p>
                        <p className="text-xs text-gray-400">{position.strategy_name}</p>
                      </div>
                    </div>
                    <Badge variant={position.pnl_usd >= 0 ? 'success' : 'danger'} className="text-lg px-4 py-2">
                      {formatPercentage(position.pnl_percentage, 1, true)}
                    </Badge>
                  </div>
                  <div className="grid grid-cols-3 gap-4 text-sm">
                    <div>
                      <p className="text-gray-400">Entry</p>
                      <p className="font-semibold">{formatUSD(position.entry_price_usd)}</p>
                    </div>
                    <div>
                      <p className="text-gray-400">Current</p>
                      <p className="font-semibold">{formatUSD(position.current_price_usd)}</p>
                    </div>
                    <div>
                      <p className="text-gray-400">PnL</p>
                      <p className={`font-bold ${getPnLColor(position.pnl_usd)}`}>
                        {formatUSD(position.pnl_usd)}
                      </p>
                    </div>
                  </div>
                </div>
              ))}
              {(!positions || positions.length === 0) && (
                <div className="text-center py-12 text-gray-400">
                  <Wallet size={48} className="mx-auto mb-3 opacity-50" />
                  <p>No active positions</p>
                </div>
              )}
            </div>
          </Card>

          {/* Recent Trades Stream */}
          <Card>
            <CardHeader title="📊 Live Trade Stream" subtitle="Real-time execution feed" />
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {recentTrades?.data?.map((trade) => (
                <div
                  key={trade.id}
                  className="p-3 bg-dark-700/30 rounded-lg border-l-4 hover:bg-dark-700/50 transition-all"
                  style={{
                    borderLeftColor: trade.side === 'BUY' ? '#10b981' : '#ef4444'
                  }}
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      {trade.side === 'BUY' ? (
                        <TrendingUp className="text-success-500" size={20} />
                      ) : (
                        <TrendingDown className="text-danger-500" size={20} />
                      )}
                      <div>
                        <p className="font-semibold">
                          {trade.side} {trade.token_symbol}
                        </p>
                        <p className="text-xs text-gray-400 flex items-center gap-1">
                          <Clock size={12} />
                          {formatRelativeTime(trade.created_at)}
                        </p>
                      </div>
                    </div>
                    <div className="text-right">
                      <p className="font-semibold">{formatUSD(trade.amount_usd)}</p>
                      <Badge variant={trade.status === 'COMPLETED' ? 'success' : 'gray'}>
                        {trade.status}
                      </Badge>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </div>

        {/* Strategy Control Panel */}
        <div className="space-y-6">
          {/* Quick Actions */}
          <Card className="bg-gradient-to-br from-primary-900/30 to-dark-800 border-primary-500/30">
            <CardHeader title="⚡ Quick Actions" subtitle="Rapid fire controls" />
            <div className="space-y-3">
              <Button variant="primary" className="w-full justify-start" size="lg">
                <Play size={20} />
                Start All Strategies
              </Button>
              <Button variant="secondary" className="w-full justify-start" size="lg">
                <Pause size={20} />
                Pause All Strategies
              </Button>
              <Button variant="danger" className="w-full justify-start" size="lg">
                <XCircle size={20} />
                Emergency Stop
              </Button>
              <Button variant="success" className="w-full justify-start" size="lg">
                <CheckCircle size={20} />
                Close All Positions
              </Button>
            </div>
          </Card>

          {/* Strategy Status */}
          <Card>
            <CardHeader title="🎲 Strategy Status" subtitle={`${activeStrategies.length} active`} />
            <div className="space-y-3">
              {strategies?.slice(0, 6).map((strategy) => (
                <div
                  key={strategy.id}
                  className="p-3 bg-dark-700/30 rounded-lg border border-dark-600 hover:border-primary-500/50 transition-all"
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-2">
                      {strategy.is_active ? (
                        <div className="w-2 h-2 bg-success-500 rounded-full animate-pulse"></div>
                      ) : (
                        <div className="w-2 h-2 bg-gray-500 rounded-full"></div>
                      )}
                      <p className="font-semibold text-sm">{strategy.name}</p>
                    </div>
                    <Badge variant={strategy.is_active ? 'success' : 'gray'}>
                      {strategy.is_active ? 'ACTIVE' : 'PAUSED'}
                    </Badge>
                  </div>
                  <div className="flex items-center justify-between text-xs">
                    <span className="text-gray-400">Win Rate:</span>
                    <span className="text-success-500 font-semibold">
                      {formatPercentage(strategy.stats?.win_rate || 0, 1)}
                    </span>
                  </div>
                  <div className="flex items-center justify-between text-xs">
                    <span className="text-gray-400">Trades:</span>
                    <span className="font-semibold">{strategy.stats?.total_trades || 0}</span>
                  </div>
                </div>
              ))}
            </div>
          </Card>

          {/* Risk Monitor */}
          <Card className="bg-gradient-to-br from-danger-900/30 to-dark-800 border-danger-500/30">
            <CardHeader title="🛡️ Risk Monitor" subtitle="Safety first" />
            <div className="space-y-3">
              <div className="p-3 bg-dark-700/50 rounded-lg">
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm text-gray-400">Position Exposure</span>
                  <span className="text-sm font-semibold text-warning-500">45%</span>
                </div>
                <div className="w-full h-2 bg-dark-600 rounded-full overflow-hidden">
                  <div className="h-full bg-gradient-to-r from-warning-500 to-warning-600" style={{ width: '45%' }}></div>
                </div>
              </div>

              <div className="p-3 bg-dark-700/50 rounded-lg">
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm text-gray-400">Daily Loss Limit</span>
                  <span className="text-sm font-semibold text-success-500">22%</span>
                </div>
                <div className="w-full h-2 bg-dark-600 rounded-full overflow-hidden">
                  <div className="h-full bg-gradient-to-r from-success-500 to-success-600" style={{ width: '22%' }}></div>
                </div>
              </div>

              <div className="p-3 bg-dark-700/50 rounded-lg">
                <div className="flex items-center justify-between mb-1">
                  <span className="text-sm text-gray-400">Max Drawdown</span>
                  <span className="text-sm font-semibold text-primary-500">8%</span>
                </div>
                <div className="w-full h-2 bg-dark-600 rounded-full overflow-hidden">
                  <div className="h-full bg-gradient-to-r from-primary-500 to-primary-600" style={{ width: '8%' }}></div>
                </div>
              </div>

              <div className="flex items-center gap-2 p-3 bg-success-500/10 border border-success-500/30 rounded-lg">
                <CheckCircle size={16} className="text-success-500" />
                <span className="text-sm text-success-500 font-semibold">All Systems Safe</span>
              </div>
            </div>
          </Card>
        </div>
      </div>
    </div>
  )
}
