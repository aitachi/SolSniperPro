import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { Activity, AlertCircle } from 'lucide-react'
import { useAuth } from '@/hooks/useAuth'
import { Button } from '@/components/common/Button'
import { Input } from '@/components/common/Input'

export const Login: React.FC = () => {
  const navigate = useNavigate()
  const { login, isLoading, error, clearError } = useAuth()
  const [credentials, setCredentials] = useState({
    username: '',
    password: '',
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    clearError()

    try {
      await login(credentials)
      navigate('/')
    } catch (err) {
      // Error is handled by the auth store
    }
  }

  return (
    <div className="min-h-screen bg-dark-950 flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        {/* Logo and Title */}
        <div className="text-center mb-8">
          <div className="inline-flex w-16 h-16 bg-gradient-to-br from-primary-500 to-primary-700 rounded-2xl items-center justify-center mb-4 shadow-glow">
            <Activity size={32} className="text-white" />
          </div>
          <h1 className="text-3xl font-bold text-gradient mb-2">
            SolSniper Pro
          </h1>
          <p className="text-gray-400">
            Professional Solana Token Trading Platform
          </p>
        </div>

        {/* Login Form */}
        <div className="card">
          <h2 className="text-xl font-semibold mb-6">Sign In</h2>

          {error && (
            <div className="mb-4 p-3 bg-danger-500/10 border border-danger-500/30 rounded-lg flex items-start gap-2">
              <AlertCircle size={18} className="text-danger-500 mt-0.5" />
              <div className="text-sm text-danger-500">{error}</div>
            </div>
          )}

          <form onSubmit={handleSubmit} className="space-y-4">
            <Input
              label="Username"
              type="text"
              value={credentials.username}
              onChange={(e) =>
                setCredentials({ ...credentials, username: e.target.value })
              }
              placeholder="Enter your username"
              required
              autoFocus
            />

            <Input
              label="Password"
              type="password"
              value={credentials.password}
              onChange={(e) =>
                setCredentials({ ...credentials, password: e.target.value })
              }
              placeholder="Enter your password"
              required
            />

            <Button
              type="submit"
              variant="primary"
              className="w-full"
              loading={isLoading}
              disabled={!credentials.username || !credentials.password}
            >
              Sign In
            </Button>
          </form>

          <div className="mt-6 pt-6 border-t border-dark-700">
            <p className="text-sm text-gray-400 text-center">
              Default credentials: admin / admin123
            </p>
          </div>
        </div>

        {/* Footer */}
        <div className="mt-8 text-center text-sm text-gray-500">
          <p>v2.0.0 &copy; 2025 SolSniper Pro</p>
        </div>
      </div>
    </div>
  )
}
