import React from 'react'
import { Link } from 'react-router-dom'
import {
  Bell,
  Settings,
  LogOut,
  User,
  Activity,
} from 'lucide-react'
import { useAuth } from '@/hooks/useAuth'
import { useSystemHealth } from '@/hooks/useMetrics'
import { Badge } from '@/components/common/Badge'
import { cn } from '@/utils/helpers'

export const Header: React.FC = () => {
  const { user, logout } = useAuth()
  const { data: health } = useSystemHealth()

  const getStatusColor = (status?: string) => {
    if (!status) return 'bg-gray-500'
    switch (status) {
      case 'HEALTHY':
        return 'status-online'
      case 'DEGRADED':
        return 'status-warning'
      case 'CRITICAL':
        return 'status-offline'
      default:
        return 'bg-gray-500'
    }
  }

  return (
    <header className="h-16 bg-dark-800 border-b border-dark-700 px-6 flex items-center justify-between sticky top-0 z-40">
      {/* Logo and Brand */}
      <div className="flex items-center gap-4">
        <Link to="/" className="flex items-center gap-3">
          <div className="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center">
            <Activity size={20} className="text-white" />
          </div>
          <div>
            <h1 className="text-xl font-bold text-gradient">SolSniper Pro</h1>
            <p className="text-xs text-gray-500">v2.0.0</p>
          </div>
        </Link>

        {/* System Status */}
        {health && (
          <div className="flex items-center gap-2 ml-4 px-3 py-1.5 bg-dark-700 rounded-lg">
            <div className={cn('status-dot', getStatusColor(health.status))} />
            <span className="text-xs font-medium text-gray-300">
              {health.status}
            </span>
          </div>
        )}
      </div>

      {/* Right Section */}
      <div className="flex items-center gap-4">
        {/* Notifications */}
        <button className="relative p-2 rounded-lg hover:bg-dark-700 transition-colors">
          <Bell size={20} />
          <span className="absolute top-1 right-1 w-2 h-2 bg-danger-500 rounded-full" />
        </button>

        {/* Settings */}
        <Link
          to="/settings"
          className="p-2 rounded-lg hover:bg-dark-700 transition-colors"
        >
          <Settings size={20} />
        </Link>

        {/* User Menu */}
        <div className="flex items-center gap-3 pl-4 border-l border-dark-700">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 bg-primary-600 rounded-full flex items-center justify-center">
              <User size={16} />
            </div>
            <div className="text-sm">
              <p className="font-medium">{user?.username || 'User'}</p>
              <p className="text-xs text-gray-400">
                {user?.role || 'Trader'}
              </p>
            </div>
          </div>

          <button
            onClick={logout}
            className="p-2 rounded-lg hover:bg-danger-600/20 text-danger-500 hover:text-danger-400 transition-colors"
            title="Logout"
          >
            <LogOut size={18} />
          </button>
        </div>
      </div>
    </header>
  )
}
