import axios, { AxiosError } from 'axios'
import { useAuthStore } from '@/stores/authStore'
import { API_BASE_URL, API_TIMEOUT } from '@/utils/constants'
import type { ApiResponse, ApiError } from '@/types/api'

const client = axios.create({
  baseURL: API_BASE_URL,
  timeout: API_TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Request interceptor: Add authorization token
client.interceptors.request.use(
  (config) => {
    const { token } = useAuthStore.getState()
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// Response interceptor: Handle errors and unwrap data
client.interceptors.response.use(
  (response) => {
    // Unwrap ApiResponse if present
    if (response.data && 'success' in response.data) {
      const apiResponse = response.data as ApiResponse<any>
      if (apiResponse.success) {
        return apiResponse.data
      } else {
        return Promise.reject(apiResponse.error)
      }
    }
    return response.data
  },
  (error: AxiosError<ApiResponse<any>>) => {
    // Handle 401 Unauthorized
    if (error.response?.status === 401) {
      const { logout } = useAuthStore.getState()
      logout()
      window.location.href = '/login'
    }

    // Extract error message
    let errorMessage = 'An error occurred'
    let errorCode = 'UNKNOWN_ERROR'

    if (error.response?.data) {
      const apiError = error.response.data.error
      if (apiError) {
        errorMessage = apiError.message || errorMessage
        errorCode = apiError.code || errorCode
      }
    } else if (error.message) {
      errorMessage = error.message
    }

    // Create standardized error
    const apiError: ApiError = {
      code: errorCode,
      message: errorMessage,
      details: error.response?.data?.error?.details,
    }

    return Promise.reject(apiError)
  }
)

export default client
