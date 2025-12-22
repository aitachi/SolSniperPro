import React from 'react'
import { useStrategies, useToggleStrategy } from '@/hooks/useStrategies'
import { Card, CardHeader } from '@/components/common/Card'
import { Table } from '@/components/common/Table'
import { Badge } from '@/components/common/Badge'
import { Button } from '@/components/common/Button'
import type { TableColumn } from '@/types/api'
import type { Strategy } from '@/types/strategy'
import { formatNumber, formatPercentage } from '@/utils/format'
import { Play, Pause } from 'lucide-react'

export const Strategies: React.FC = () => {
  const { data: strategies, isLoading } = useStrategies()
  const toggleMutation = useToggleStrategy()

  const handleToggle = (id: string, enabled: boolean) => {
    toggleMutation.mutate({ id, enabled: !enabled })
  }

  const columns: TableColumn<Strategy>[] = [
    {
      key: 'name',
      label: 'Strategy',
      render: (_, strategy) => (
        <div>
          <p className="font-semibold">{strategy.name}</p>
          <Badge variant={strategy.enabled ? 'success' : 'gray'}>
            {strategy.enabled ? 'Active' : 'Inactive'}
          </Badge>
        </div>
      ),
    },
    {
      key: 'priority',
      label: 'Priority',
      render: (value) => <Badge variant="primary">{value}</Badge>,
    },
    {
      key: 'stats.total_trades',
      label: 'Trades',
      render: (value) => value || 0,
    },
    {
      key: 'stats.win_rate',
      label: 'Win Rate',
      render: (value) => (
        <span className={value >= 70 ? 'text-success-500' : value >= 50 ? 'text-warning-500' : 'text-danger-500'}>
          {formatPercentage(value || 0, 1)}
        </span>
      ),
    },
    {
      key: 'stats.total_pnl_usd',
      label: 'Total PnL',
      render: (value) => (
        <span className={value >= 0 ? 'text-success-500' : 'text-danger-500'}>
          ${formatNumber(value || 0, 2)}
        </span>
      ),
    },
    {
      key: 'stats.sharpe_ratio',
      label: 'Sharpe Ratio',
      render: (value) => formatNumber(value || 0, 2),
    },
    {
      key: 'id',
      label: 'Actions',
      render: (_, strategy) => (
        <Button
          size="sm"
          variant={strategy.enabled ? 'danger' : 'success'}
          onClick={() => handleToggle(strategy.id, strategy.enabled)}
        >
          {strategy.enabled ? (
            <>
              <Pause size={14} /> Pause
            </>
          ) : (
            <>
              <Play size={14} /> Start
            </>
          )}
        </Button>
      ),
    },
  ]

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold mb-2">Strategies</h1>
          <p className="text-gray-400">
            Manage and configure your trading strategies
          </p>
        </div>
        <Button variant="primary">
          Create Strategy
        </Button>
      </div>

      <Card>
        <CardHeader
          title="Active Strategies"
          subtitle={`${strategies?.filter(s => s.enabled).length || 0} of ${strategies?.length || 0} strategies active`}
        />
        <Table
          data={strategies || []}
          columns={columns}
          loading={isLoading}
          emptyMessage="No strategies configured"
        />
      </Card>
    </div>
  )
}
