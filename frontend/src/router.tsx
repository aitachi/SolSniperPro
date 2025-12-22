import { Routes, Route, Navigate } from 'react-router-dom'
import { useAuthStore } from './stores/authStore'
import { Layout } from './components/layout/Layout'
import { Login } from './pages/Login'
import { Dashboard } from './pages/Dashboard'
import { CommandCenter } from './pages/CommandCenter'
import { Tokens } from './pages/Tokens'
import { Strategies } from './pages/Strategies'
import { Trading } from './pages/Trading'
import { Positions } from './pages/Positions'
import { RiskControl } from './pages/RiskControl'
import { Analytics } from './pages/Analytics'
import { Settings } from './pages/Settings'

interface ProtectedRouteProps {
  children: React.ReactNode
}

const ProtectedRoute = ({ children }: ProtectedRouteProps) => {
  const { token } = useAuthStore()

  if (!token) {
    return <Navigate to="/login" replace />
  }

  return <>{children}</>
}

export const AppRouter = () => {
  const { token } = useAuthStore()

  return (
    <Routes>
      {/* Public Routes */}
      <Route
        path="/login"
        element={token ? <Navigate to="/" replace /> : <Login />}
      />

      {/* Protected Routes */}
      <Route
        path="/"
        element={
          <ProtectedRoute>
            <Layout />
          </ProtectedRoute>
        }
      >
        <Route index element={<Dashboard />} />
        <Route path="command-center" element={<CommandCenter />} />
        <Route path="tokens" element={<Tokens />} />
        <Route path="strategies" element={<Strategies />} />
        <Route path="trading" element={<Trading />} />
        <Route path="positions" element={<Positions />} />
        <Route path="risk-control" element={<RiskControl />} />
        <Route path="analytics" element={<Analytics />} />
        <Route path="settings" element={<Settings />} />
      </Route>

      {/* Catch all - redirect to dashboard */}
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  )
}
