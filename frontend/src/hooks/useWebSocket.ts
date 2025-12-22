import { useEffect, useRef, useState, useCallback } from 'react'
import { useAuthStore } from '@/stores/authStore'
import { useTokenStore } from '@/stores/tokenStore'
import { useTradeStore } from '@/stores/tradeStore'
import { useStrategyStore } from '@/stores/strategyStore'
import type {
  WebSocketMessage,
  WebSocketTopic,
  WebSocketEvent,
} from '@/types/api'
import toast from 'react-hot-toast'

interface UseWebSocketOptions {
  topics: WebSocketTopic[]
  onMessage?: (message: WebSocketMessage) => void
  autoReconnect?: boolean
  reconnectInterval?: number
}

export const useWebSocket = ({
  topics,
  onMessage,
  autoReconnect = true,
  reconnectInterval = 5000,
}: UseWebSocketOptions) => {
  const [isConnected, setIsConnected] = useState(false)
  const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null)
  const ws = useRef<WebSocket | null>(null)
  const reconnectTimeout = useRef<NodeJS.Timeout | null>(null)
  const { token } = useAuthStore()
  const { addToken, updateToken } = useTokenStore()
  const { addTrade, updateTrade, addPosition, updatePosition, removePosition } =
    useTradeStore()
  const { updateStrategy } = useStrategyStore()

  const connect = useCallback(() => {
    if (!token) {
      console.warn('No auth token available for WebSocket connection')
      return
    }

    // Close existing connection
    if (ws.current) {
      ws.current.close()
    }

    try {
      const wsUrl = `ws://localhost:3000/ws?token=${token}`
      ws.current = new WebSocket(wsUrl)

      ws.current.onopen = () => {
        console.log('WebSocket connected')
        setIsConnected(true)

        // Subscribe to topics
        ws.current?.send(
          JSON.stringify({
            type: 'subscribe',
            topics,
          })
        )
      }

      ws.current.onmessage = (event) => {
        try {
          const message: WebSocketMessage = JSON.parse(event.data)
          setLastMessage(message)

          // Call custom handler
          if (onMessage) {
            onMessage(message)
          }

          // Handle built-in events
          handleEvent(message)
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error)
        }
      }

      ws.current.onerror = (error) => {
        console.error('WebSocket error:', error)
      }

      ws.current.onclose = () => {
        console.log('WebSocket disconnected')
        setIsConnected(false)

        // Auto-reconnect
        if (autoReconnect && token) {
          reconnectTimeout.current = setTimeout(() => {
            console.log('Attempting to reconnect...')
            connect()
          }, reconnectInterval)
        }
      }
    } catch (error) {
      console.error('Failed to create WebSocket connection:', error)
    }
  }, [token, topics, onMessage, autoReconnect, reconnectInterval])

  const handleEvent = (message: WebSocketMessage) => {
    const { event, data } = message

    switch (event as WebSocketEvent) {
      case 'new_token':
        addToken(data)
        toast.success(`New token detected: ${data.symbol}`)
        break

      case 'token_update':
        updateToken(data.mint, data)
        break

      case 'trade_created':
      case 'trade_executed':
        addTrade(data)
        if (event === 'trade_executed') {
          toast.success(`Trade executed: ${data.side} ${data.token_symbol}`)
        }
        break

      case 'trade_failed':
        updateTrade(data.id, data)
        toast.error(`Trade failed: ${data.error_message}`)
        break

      case 'position_opened':
        addPosition(data)
        toast.success(`Position opened: ${data.token_symbol}`)
        break

      case 'position_updated':
        updatePosition(data.id, data)
        break

      case 'position_closed':
        removePosition(data.id)
        toast.success(
          `Position closed: ${data.token_symbol} - PnL: ${data.pnl_usd > 0 ? '+' : ''}$${data.pnl_usd.toFixed(2)}`
        )
        break

      case 'strategy_triggered':
        toast(`Strategy triggered: ${data.strategy_name} for ${data.token_symbol}`, { icon: 'ℹ️' })
        break

      case 'strategy_updated':
        updateStrategy(data.id, data)
        break

      case 'risk_alert':
        const severity = data.severity
        if (severity === 'CRITICAL') {
          toast.error(`Risk Alert: ${data.message}`)
        } else if (severity === 'WARNING') {
          toast('Risk Warning: ' + data.message, { icon: '⚠️' })
        }
        break

      case 'risk_violation':
        toast.error(`Risk Violation: ${data.message}`)
        break

      default:
        console.log('Unhandled WebSocket event:', event, data)
    }
  }

  const disconnect = useCallback(() => {
    if (reconnectTimeout.current) {
      clearTimeout(reconnectTimeout.current)
    }
    if (ws.current) {
      ws.current.close()
      ws.current = null
    }
    setIsConnected(false)
  }, [])

  const subscribe = useCallback((newTopics: WebSocketTopic[]) => {
    if (ws.current && ws.current.readyState === WebSocket.OPEN) {
      ws.current.send(
        JSON.stringify({
          type: 'subscribe',
          topics: newTopics,
        })
      )
    }
  }, [])

  const unsubscribe = useCallback((topicsToRemove: WebSocketTopic[]) => {
    if (ws.current && ws.current.readyState === WebSocket.OPEN) {
      ws.current.send(
        JSON.stringify({
          type: 'unsubscribe',
          topics: topicsToRemove,
        })
      )
    }
  }, [])

  useEffect(() => {
    connect()

    return () => {
      disconnect()
    }
  }, [connect, disconnect])

  return {
    isConnected,
    lastMessage,
    subscribe,
    unsubscribe,
    reconnect: connect,
    disconnect,
  }
}
