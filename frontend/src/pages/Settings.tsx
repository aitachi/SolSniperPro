import React from 'react'
import { Card, CardHeader } from '@/components/common/Card'
import { Input, Select } from '@/components/common/Input'
import { Button } from '@/components/common/Button'
import { Badge } from '@/components/common/Badge'
import { useUIStore } from '@/stores/uiStore'
import { useAuth } from '@/hooks/useAuth'
import {
  User,
  Bell,
  Volume2,
  RefreshCw,
  DollarSign,
  Globe,
} from 'lucide-react'

export const Settings: React.FC = () => {
  const { user } = useAuth()
  const {
    soundEnabled,
    notificationsEnabled,
    autoRefreshEnabled,
    autoRefreshInterval,
    currency,
    toggleSound,
    toggleNotifications,
    toggleAutoRefresh,
    setAutoRefreshInterval,
    setCurrency,
  } = useUIStore()

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">Settings</h1>
        <p className="text-gray-400">
          Manage your account and application preferences
        </p>
      </div>

      {/* Account Information */}
      <Card>
        <CardHeader
          title="Account Information"
          subtitle="Your profile and account details"
        />
        <div className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Input
              label="Username"
              value={user?.username || ''}
              disabled
              readOnly
            />
            <div>
              <label className="label">Role</label>
              <Badge variant="primary" className="text-sm">
                {user?.role || 'TRADER'}
              </Badge>
            </div>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Input
              label="User ID"
              value={user?.id || ''}
              disabled
              readOnly
            />
            <Input
              label="Member Since"
              value={
                user?.created_at
                  ? new Date(user.created_at).toLocaleDateString()
                  : ''
              }
              disabled
              readOnly
            />
          </div>
        </div>
      </Card>

      {/* Notifications */}
      <Card>
        <CardHeader
          title="Notifications"
          subtitle="Manage how you receive notifications"
        />
        <div className="space-y-4">
          <div className="flex items-center justify-between p-4 bg-dark-700/50 rounded-lg">
            <div className="flex items-center gap-3">
              <Bell className="text-primary-500" size={20} />
              <div>
                <p className="font-medium">Push Notifications</p>
                <p className="text-sm text-gray-400">
                  Receive notifications for trades and alerts
                </p>
              </div>
            </div>
            <button
              onClick={toggleNotifications}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                notificationsEnabled ? 'bg-primary-600' : 'bg-dark-600'
              }`}
            >
              <div
                className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform ${
                  notificationsEnabled ? 'translate-x-7' : 'translate-x-1'
                }`}
              />
            </button>
          </div>

          <div className="flex items-center justify-between p-4 bg-dark-700/50 rounded-lg">
            <div className="flex items-center gap-3">
              <Volume2 className="text-primary-500" size={20} />
              <div>
                <p className="font-medium">Sound Effects</p>
                <p className="text-sm text-gray-400">
                  Play sounds for important events
                </p>
              </div>
            </div>
            <button
              onClick={toggleSound}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                soundEnabled ? 'bg-primary-600' : 'bg-dark-600'
              }`}
            >
              <div
                className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform ${
                  soundEnabled ? 'translate-x-7' : 'translate-x-1'
                }`}
              />
            </button>
          </div>
        </div>
      </Card>

      {/* Display Preferences */}
      <Card>
        <CardHeader
          title="Display Preferences"
          subtitle="Customize how data is displayed"
        />
        <div className="space-y-4">
          <Select
            label="Preferred Currency"
            value={currency}
            onChange={(e) => setCurrency(e.target.value as 'USD' | 'SOL')}
            options={[
              { value: 'USD', label: 'USD ($)' },
              { value: 'SOL', label: 'SOL (◎)' },
            ]}
          />

          <div className="flex items-center justify-between p-4 bg-dark-700/50 rounded-lg">
            <div className="flex items-center gap-3">
              <RefreshCw className="text-primary-500" size={20} />
              <div>
                <p className="font-medium">Auto Refresh</p>
                <p className="text-sm text-gray-400">
                  Automatically refresh data
                </p>
              </div>
            </div>
            <button
              onClick={toggleAutoRefresh}
              className={`relative w-12 h-6 rounded-full transition-colors ${
                autoRefreshEnabled ? 'bg-primary-600' : 'bg-dark-600'
              }`}
            >
              <div
                className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform ${
                  autoRefreshEnabled ? 'translate-x-7' : 'translate-x-1'
                }`}
              />
            </button>
          </div>

          {autoRefreshEnabled && (
            <Select
              label="Refresh Interval"
              value={autoRefreshInterval}
              onChange={(e) => setAutoRefreshInterval(Number(e.target.value))}
              options={[
                { value: 2000, label: '2 seconds' },
                { value: 5000, label: '5 seconds' },
                { value: 10000, label: '10 seconds' },
                { value: 30000, label: '30 seconds' },
              ]}
            />
          )}
        </div>
      </Card>

      {/* Save Button */}
      <div className="flex justify-end">
        <Button variant="primary">
          Save Preferences
        </Button>
      </div>
    </div>
  )
}
