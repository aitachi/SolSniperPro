import { BrowserRouter } from 'react-router-dom'
import { Toaster } from 'react-hot-toast'
import { QueryClient, QueryClientProvider } from 'react-query'
import { AppRouter } from './router'
import { useAuthStore } from './stores/authStore'
import { useEffect } from 'react'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
      staleTime: 5000,
    },
  },
})

function App() {
  const { token } = useAuthStore()

  useEffect(() => {
    // Initialize theme
    document.documentElement.classList.add('dark')
  }, [])

  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <AppRouter />
        <Toaster
          position="top-right"
          toastOptions={{
            duration: 4000,
            className: 'toast',
            success: {
              className: 'toast-success',
              iconTheme: {
                primary: '#10b981',
                secondary: '#fff',
              },
            },
            error: {
              className: 'toast-error',
              iconTheme: {
                primary: '#ef4444',
                secondary: '#fff',
              },
            },
          }}
        />
      </BrowserRouter>
    </QueryClientProvider>
  )
}

export default App
