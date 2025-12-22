import React from 'react'
import { usePositions, useClosePosition } from '@/hooks/usePositions'
import { Card, CardHeader } from '@/components/common/Card'
import { Table } from '@/components/common/Table'
import { Button } from '@/components/common/Button'
import type { TableColumn } from '@/types/api'
import type { Position } from '@/types/trade'
import { formatUSD, formatPercentage, formatDuration } from '@/utils/format'
import { getPnLColor } from '@/utils/helpers'

export const Positions: React.FC = () => {
  const { data: positions, isLoading } = usePositions()
  const closeMutation = useClosePosition()

  const handleClose = (id: string) => {
    if (confirm('Are you sure you want to close this position?')) {
      closeMutation.mutate({ id })
    }
  }

  const columns: TableColumn<Position>[] = [
    {
      key: 'token_symbol',
      label: 'Token',
      render: (_, position) => (
        <div>
          <p className="font-semibold">{position.token_symbol}</p>
          <p className="text-xs text-gray-400">{position.strategy_name}</p>
        </div>
      ),
    },
    {
      key: 'entry_price_usd',
      label: 'Entry Price',
      render: (value) => formatUSD(value as number, 6),
    },
    {
      key: 'current_price_usd',
      label: 'Current Price',
      render: (value) => formatUSD(value as number, 6),
    },
    {
      key: 'invested_usd',
      label: 'Invested',
      render: (value) => formatUSD(value as number),
    },
    {
      key: 'current_value_usd',
      label: 'Current Value',
      render: (value) => formatUSD(value as number),
    },
    {
      key: 'pnl_usd',
      label: 'PnL',
      render: (value, position) => (
        <div>
          <p className={getPnLColor(value as number)}>
            {formatUSD(value as number)}
          </p>
          <p className={`text-xs ${getPnLColor(position.pnl_percentage)}`}>
            {formatPercentage(position.pnl_percentage, 2, true)}
          </p>
        </div>
      ),
    },
    {
      key: 'holding_hours',
      label: 'Holding Time',
      render: (value) => formatDuration(value as number),
    },
    {
      key: 'id',
      label: 'Actions',
      render: (_, position) => (
        <Button
          size="sm"
          variant="danger"
          onClick={() => handleClose(position.id)}
          disabled={position.status !== 'ACTIVE'}
        >
          Close
        </Button>
      ),
    },
  ]

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">Positions</h1>
        <p className="text-gray-400">
          Monitor and manage your open positions
        </p>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <p className="metric-label">Total Positions</p>
          <p className="metric-value">{positions?.length || 0}</p>
        </Card>
        <Card>
          <p className="metric-label">Total Value</p>
          <p className="metric-value">
            {formatUSD(positions?.reduce((sum, p) => sum + p.current_value_usd, 0) || 0)}
          </p>
        </Card>
        <Card>
          <p className="metric-label">Unrealized PnL</p>
          <p className={`metric-value ${getPnLColor(positions?.reduce((sum, p) => sum + p.pnl_usd, 0) || 0)}`}>
            {formatUSD(positions?.reduce((sum, p) => sum + p.pnl_usd, 0) || 0)}
          </p>
        </Card>
      </div>

      {/* Positions Table */}
      <Card>
        <CardHeader
          title="Active Positions"
          subtitle={`${positions?.filter(p => p.status === 'ACTIVE').length || 0} active`}
        />
        <Table
          data={positions || []}
          columns={columns}
          loading={isLoading}
          emptyMessage="No open positions"
        />
      </Card>
    </div>
  )
}
