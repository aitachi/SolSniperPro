import React, { useState } from 'react'
import { useTokens } from '@/hooks/useTokens'
import { Card, CardHeader } from '@/components/common/Card'
import { Table } from '@/components/common/Table'
import { Badge } from '@/components/common/Badge'
import { Input } from '@/components/common/Input'
import type { TableColumn } from '@/types/api'
import type { TokenInfo } from '@/types/token'
import { formatUSD, formatCompact, formatPercentage } from '@/utils/format'

export const Tokens: React.FC = () => {
  const [filters, setFilters] = useState({
    min_liquidity: 50,
    min_risk_score: 70,
    max_age_hours: 24,
  })

  const { data, isLoading } = useTokens(filters)

  const columns: TableColumn<TokenInfo>[] = [
    {
      key: 'symbol',
      label: 'Token',
      render: (_, token) => (
        <div>
          <p className="font-semibold">{token.symbol}</p>
          <p className="text-xs text-gray-400">{token.name}</p>
        </div>
      ),
    },
    {
      key: 'liquidity_usd',
      label: 'Liquidity',
      render: (value) => formatUSD(value as number),
    },
    {
      key: 'holders_count',
      label: 'Holders',
      render: (value) => formatCompact(value as number, 0),
    },
    {
      key: 'price_usd',
      label: 'Price',
      render: (value) => formatUSD(value as number, 6),
    },
    {
      key: 'price_change_1h',
      label: '1h Change',
      render: (value) => (
        <span className={value >= 0 ? 'text-success-500' : 'text-danger-500'}>
          {formatPercentage(value as number, 2, true)}
        </span>
      ),
    },
    {
      key: 'age_hours',
      label: 'Age',
      render: (value) => `${Math.floor(value as number)}h`,
    },
    {
      key: 'is_renounced',
      label: 'Status',
      render: (_, token) => (
        <div className="flex gap-1">
          {token.is_renounced && <Badge variant="success">Renounced</Badge>}
          {!token.is_mutable && <Badge variant="primary">Immutable</Badge>}
        </div>
      ),
    },
  ]

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">Token Monitor</h1>
        <p className="text-gray-400">
          Real-time monitoring of Solana tokens
        </p>
      </div>

      {/* Filters */}
      <Card>
        <CardHeader title="Filters" />
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Input
            label="Min Liquidity (SOL)"
            type="number"
            value={filters.min_liquidity}
            onChange={(e) => setFilters({ ...filters, min_liquidity: Number(e.target.value) })}
          />
          <Input
            label="Min Risk Score"
            type="number"
            value={filters.min_risk_score}
            onChange={(e) => setFilters({ ...filters, min_risk_score: Number(e.target.value) })}
            min={0}
            max={100}
          />
          <Input
            label="Max Age (hours)"
            type="number"
            value={filters.max_age_hours}
            onChange={(e) => setFilters({ ...filters, max_age_hours: Number(e.target.value) })}
          />
        </div>
      </Card>

      {/* Token List */}
      <Card>
        <CardHeader
          title="Token List"
          subtitle={`${data?.data?.length || 0} tokens found`}
        />
        <Table
          data={data?.data || []}
          columns={columns}
          loading={isLoading}
          emptyMessage="No tokens found matching your filters"
        />
      </Card>
    </div>
  )
}
