import React from 'react'
import { Card, CardHeader } from '@/components/common/Card'
import { Input } from '@/components/common/Input'
import { Button } from '@/components/common/Button'
import { Shield } from 'lucide-react'

export const RiskControl: React.FC = () => {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">Risk Control</h1>
        <p className="text-gray-400">
          Configure and monitor risk management settings
        </p>
      </div>

      {/* Risk Limits */}
      <Card>
        <CardHeader
          title="Position Limits"
          subtitle="Configure maximum position sizes and exposure"
        />
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Input
            label="Max Position Size (SOL)"
            type="number"
            defaultValue="10"
            step="0.1"
          />
          <Input
            label="Max Position Size (%)"
            type="number"
            defaultValue="20"
            min="0"
            max="100"
          />
          <Input
            label="Max Total Exposure (SOL)"
            type="number"
            defaultValue="100"
            step="1"
          />
          <Input
            label="Max Positions Count"
            type="number"
            defaultValue="10"
            min="1"
          />
        </div>
      </Card>

      {/* Loss Limits */}
      <Card>
        <CardHeader
          title="Loss Limits"
          subtitle="Configure stop-loss and drawdown limits"
        />
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Input
            label="Max Loss Per Trade (SOL)"
            type="number"
            defaultValue="2"
            step="0.1"
          />
          <Input
            label="Max Daily Loss (SOL)"
            type="number"
            defaultValue="10"
            step="0.1"
          />
          <Input
            label="Max Drawdown (%)"
            type="number"
            defaultValue="20"
            min="0"
            max="100"
          />
        </div>
      </Card>

      {/* Risk Scoring */}
      <Card>
        <CardHeader
          title="Risk Scoring"
          subtitle="Configure risk score thresholds"
        />
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Input
            label="Min Risk Score"
            type="number"
            defaultValue="70"
            min="0"
            max="100"
          />
          <Input
            label="Max Risk Score"
            type="number"
            defaultValue="95"
            min="0"
            max="100"
          />
          <div className="flex items-center gap-2">
            <input type="checkbox" id="block-extreme" />
            <label htmlFor="block-extreme" className="text-sm">
              Block trades with EXTREME risk
            </label>
          </div>
        </div>
      </Card>

      {/* Save Button */}
      <div className="flex justify-end">
        <Button variant="primary">
          <Shield size={16} />
          Save Risk Settings
        </Button>
      </div>
    </div>
  )
}
