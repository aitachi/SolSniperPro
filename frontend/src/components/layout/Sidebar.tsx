import React from 'react'
import { NavLink } from 'react-router-dom'
import {
  LayoutDashboard,
  Zap,
  Coins,
  Target,
  TrendingUp,
  Wallet,
  Shield,
  BarChart3,
  Settings,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react'
import { useUIStore } from '@/stores/uiStore'
import { cn } from '@/utils/helpers'

const navItems = [
  {
    path: '/',
    icon: LayoutDashboard,
    label: 'Dashboard',
  },
  {
    path: '/command-center',
    icon: Zap,
    label: 'Command Center',
    badge: 'NEW',
  },
  {
    path: '/tokens',
    icon: Coins,
    label: 'Tokens',
  },
  {
    path: '/strategies',
    icon: Target,
    label: 'Strategies',
  },
  {
    path: '/trading',
    icon: TrendingUp,
    label: 'Trading',
  },
  {
    path: '/positions',
    icon: Wallet,
    label: 'Positions',
  },
  {
    path: '/risk-control',
    icon: Shield,
    label: 'Risk Control',
  },
  {
    path: '/analytics',
    icon: BarChart3,
    label: 'Analytics',
  },
  {
    path: '/settings',
    icon: Settings,
    label: 'Settings',
  },
]

export const Sidebar: React.FC = () => {
  const { sidebarCollapsed, toggleSidebar } = useUIStore()

  return (
    <aside
      className={cn(
        'bg-dark-800 border-r border-dark-700 transition-all duration-300 flex flex-col',
        sidebarCollapsed ? 'w-16' : 'w-64'
      )}
    >
      {/* Navigation Items */}
      <nav className="flex-1 px-3 py-4 space-y-1">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            end={item.path === '/'}
            className={({ isActive }) =>
              cn(
                'flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all duration-200 relative',
                isActive
                  ? 'bg-primary-600 text-white shadow-glow'
                  : 'text-gray-400 hover:bg-dark-700 hover:text-gray-200'
              )
            }
          >
            <item.icon size={20} className="flex-shrink-0" />
            {!sidebarCollapsed && (
              <>
                <span className="font-medium">{item.label}</span>
                {item.badge && (
                  <span className="ml-auto text-xs px-2 py-0.5 bg-primary-500 text-white rounded-full animate-pulse">
                    {item.badge}
                  </span>
                )}
              </>
            )}
          </NavLink>
        ))}
      </nav>

      {/* Collapse Toggle */}
      <div className="p-3 border-t border-dark-700">
        <button
          onClick={toggleSidebar}
          className="w-full flex items-center justify-center gap-2 px-3 py-2 rounded-lg hover:bg-dark-700 transition-colors text-gray-400 hover:text-gray-200"
        >
          {sidebarCollapsed ? (
            <ChevronRight size={20} />
          ) : (
            <>
              <ChevronLeft size={20} />
              <span className="text-sm font-medium">Collapse</span>
            </>
          )}
        </button>
      </div>
    </aside>
  )
}
