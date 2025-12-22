import React from 'react'
import { useTrades } from '@/hooks/useTrades'
import { Card, CardHeader } from '@/components/common/Card'
import { Table } from '@/components/common/Table'
import { Badge } from '@/components/common/Badge'
import { Button } from '@/components/common/Button'
import type { TableColumn } from '@/types/api'
import type { Trade } from '@/types/trade'
import { formatUSD, formatSOL, formatRelativeTime } from '@/utils/format'
import { getTradeStatusColor } from '@/utils/helpers'
import { TrendingUp, TrendingDown } from 'lucide-react'

export const Trading: React.FC = () => {
  const { data, isLoading } = useTrades({ limit: 50 })

  const columns: TableColumn<Trade>[] = [
    {
      key: 'created_at',
      label: 'Time',
      render: (value) => formatRelativeTime(value as string),
    },
    {
      key: 'side',
      label: 'Side',
      render: (value) => (
        <Badge variant={value === 'BUY' ? 'success' : 'danger'}>
          {value === 'BUY' ? (
            <>
              <TrendingUp size={12} /> BUY
            </>
          ) : (
            <>
              <TrendingDown size={12} /> SELL
            </>
          )}
        </Badge>
      ),
    },
    {
      key: 'token_symbol',
      label: 'Token',
      render: (_, trade) => (
        <div>
          <p className="font-semibold">{trade.token_symbol}</p>
          <p className="text-xs text-gray-400">{trade.strategy_name}</p>
        </div>
      ),
    },
    {
      key: 'amount_usd',
      label: 'Amount',
      render: (_, trade) => (
        <div>
          <p>{formatUSD(trade.amount_usd)}</p>
          <p className="text-xs text-gray-400">{formatSOL(trade.amount_sol)}</p>
        </div>
      ),
    },
    {
      key: 'price_usd',
      label: 'Price',
      render: (value) => formatUSD(value as number, 6),
    },
    {
      key: 'status',
      label: 'Status',
      render: (value) => (
        <Badge className={getTradeStatusColor(value as string)}>
          {value}
        </Badge>
      ),
    },
    {
      key: 'pnl_usd',
      label: 'PnL',
      render: (value) => {
        if (value === undefined || value === null) return '-'
        return (
          <span className={value >= 0 ? 'text-success-500' : 'text-danger-500'}>
            {formatUSD(value as number)}
          </span>
        )
      },
    },
  ]

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold mb-2">Trading</h1>
          <p className="text-gray-400">
            View and manage your trade history
          </p>
        </div>
        <Button variant="primary">
          Manual Trade
        </Button>
      </div>

      <Card>
        <CardHeader
          title="Trade History"
          subtitle={`${data?.data?.length || 0} trades`}
        />
        <Table
          data={data?.data || []}
          columns={columns}
          loading={isLoading}
          emptyMessage="No trades yet"
        />
      </Card>
    </div>
  )
}
