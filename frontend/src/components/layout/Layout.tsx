import React, { useEffect } from 'react'
import { Outlet } from 'react-router-dom'
import { Header } from './Header'
import { Sidebar } from './Sidebar'
import { Footer } from './Footer'
import { useWebSocket } from '@/hooks/useWebSocket'
import { useUIStore } from '@/stores/uiStore'

export const Layout: React.FC = () => {
  const { notificationsEnabled } = useUIStore()

  // Connect to WebSocket for real-time updates
  const { isConnected } = useWebSocket({
    topics: ['tokens', 'trades', 'positions', 'strategies', 'risk', 'metrics'],
    autoReconnect: true,
  })

  useEffect(() => {
    // Update connection status indicator
    console.log('WebSocket connection status:', isConnected)
  }, [isConnected])

  return (
    <div className="min-h-screen bg-dark-950 flex flex-col">
      {/* Header */}
      <Header />

      {/* Main Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        <Sidebar />

        {/* Page Content */}
        <main className="flex-1 overflow-y-auto custom-scrollbar">
          <div className="container mx-auto p-6">
            <Outlet />
          </div>
        </main>
      </div>

      {/* Footer */}
      <Footer />

      {/* WebSocket Status Indicator (for debugging) */}
      {process.env.NODE_ENV === 'development' && (
        <div className="fixed bottom-4 left-4 z-50">
          <div className="flex items-center gap-2 px-3 py-2 bg-dark-800 border border-dark-700 rounded-lg shadow-lg">
            <div
              className={`w-2 h-2 rounded-full ${
                isConnected ? 'status-online' : 'status-offline'
              }`}
            />
            <span className="text-xs text-gray-400">
              {isConnected ? 'Connected' : 'Disconnected'}
            </span>
          </div>
        </div>
      )}
    </div>
  )
}
