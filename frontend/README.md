# SolSniper Pro - Frontend
---
---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

Professional React-based frontend for the SolSniper Pro Solana token trading platform.

## Tech Stack

- **React 18.2** - UI framework
- **TypeScript 5.2** - Type safety
- **Vite 5.0** - Build tool and dev server
- **TailwindCSS 3.4** - Styling framework
- **React Router DOM 6.20** - Client-side routing
- **Zustand 4.4** - State management
- **React Query 3.39** - Data fetching and caching
- **Axios 1.6** - HTTP client
- **Lucide React** - Icon library
- **React Hot Toast** - Notifications

## Project Structure

```
frontend/
├── src/
│   ├── api/              # API client modules
│   │   ├── client.ts      # Axios instance with interceptors
│   │   ├── auth.ts        # Authentication API
│   │   ├── tokens.ts      # Token API
│   │   ├── strategies.ts  # Strategy API
│   │   ├── trades.ts      # Trading API
│   │   ├── risk.ts        # Risk management API
│   │   └── metrics.ts     # Analytics API
│   │
│   ├── components/       # React components
│   │   ├── common/        # Reusable components
│   │   │   ├── Button.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Table.tsx
│   │   │   ├── Modal.tsx
│   │   │   ├── Loading.tsx
│   │   │   ├── Badge.tsx
│   │   │   └── Input.tsx
│   │   │
│   │   └── layout/        # Layout components
│   │       ├── Header.tsx
│   │       ├── Sidebar.tsx
│   │       ├── Footer.tsx
│   │       └── Layout.tsx
│   │
│   ├── hooks/            # Custom React hooks
│   │   ├── useAuth.ts
│   │   ├── useTokens.ts
│   │   ├── useStrategies.ts
│   │   ├── useTrades.ts
│   │   ├── usePositions.ts
│   │   ├── useMetrics.ts
│   │   └── useWebSocket.ts
│   │
│   ├── pages/            # Page components
│   │   ├── Login.tsx
│   │   ├── Dashboard.tsx
│   │   ├── Tokens.tsx
│   │   ├── Strategies.tsx
│   │   ├── Trading.tsx
│   │   ├── Positions.tsx
│   │   ├── RiskControl.tsx
│   │   ├── Analytics.tsx
│   │   └── Settings.tsx
│   │
│   ├── stores/           # Zustand state stores
│   │   ├── authStore.ts
│   │   ├── tokenStore.ts
│   │   ├── strategyStore.ts
│   │   ├── tradeStore.ts
│   │   └── uiStore.ts
│   │
│   ├── types/            # TypeScript type definitions
│   │   ├── api.ts
│   │   ├── token.ts
│   │   ├── strategy.ts
│   │   ├── trade.ts
│   │   ├── risk.ts
│   │   └── metrics.ts
│   │
│   ├── utils/            # Utility functions
│   │   ├── format.ts      # Formatting utilities
│   │   ├── validation.ts  # Validation functions
│   │   ├── constants.ts   # App constants
│   │   └── helpers.ts     # Helper functions
│   │
│   ├── styles/           # Global styles
│   │   └── index.css      # TailwindCSS + custom styles
│   │
│   ├── App.tsx           # Main app component
│   ├── main.tsx          # Entry point
│   └── router.tsx        # Route configuration
│
├── public/               # Static assets
├── index.html            # HTML template
├── package.json          # Dependencies
├── vite.config.ts        # Vite configuration
├── tailwind.config.js    # Tailwind configuration
├── tsconfig.json         # TypeScript configuration
└── README.md             # This file
```

## Features

### 1. Dashboard
- Real-time trading metrics overview
- Active positions summary
- Recent trades list
- Performance charts
- System health monitoring

### 2. Token Monitor
- Real-time token discovery
- Advanced filtering (liquidity, risk score, age)
- Risk analysis visualization
- Strategy matching indicators
- One-click trading

### 3. Strategy Management
- View all trading strategies
- Enable/disable strategies
- Configure strategy parameters
- Monitor strategy performance
- Priority management

### 4. Trading Interface
- Trade history with detailed information
- Manual trade execution
- Trade simulation
- Real-time status updates
- PnL tracking

### 5. Position Management
- Active positions monitoring
- Real-time PnL updates
- Position closing
- Exit signal tracking
- Performance metrics

### 6. Risk Control
- Configure position limits
- Set loss limits
- Manage blacklist
- Risk alert system
- Cooldown management

### 7. Analytics
- Performance analysis over time
- Strategy comparison
- Trading heatmaps
- Profit/loss charts
- Statistical metrics

### 8. Settings
- Account information
- Notification preferences
- Display settings
- Currency preferences
- Auto-refresh configuration

## Development

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
cd frontend
npm install
```

### Development Server

```bash
npm run dev
```

Visit: http://localhost:5173

### Build for Production

```bash
npm run build
```

Build output will be in `dist/` directory.

### Type Checking

```bash
npm run type-check
```

### Linting

```bash
npm run lint
```

## API Integration

The frontend connects to the backend API at `/api/v1` (proxied through Vite in development).

### WebSocket Connection

Real-time updates are received through WebSocket at `/ws`. The connection is established automatically when logged in.

**Subscribed Topics:**
- `tokens` - New token discoveries and updates
- `trades` - Trade creation, execution, and completion
- `positions` - Position opens, updates, and closes
- `strategies` - Strategy triggers and updates
- `risk` - Risk alerts and violations
- `metrics` - System metrics updates

## State Management

### Zustand Stores

1. **authStore** - Authentication state
   - User information
   - JWT token
   - Login/logout actions

2. **tokenStore** - Token data
   - Token list
   - Selected token
   - Filters
   - Loading states

3. **strategyStore** - Strategy data
   - Strategy list
   - Strategy configuration
   - Performance metrics

4. **tradeStore** - Trading data
   - Trade history
   - Active positions
   - Filters

5. **uiStore** - UI state
   - Sidebar collapsed state
   - Modals
   - Theme
   - Notifications
   - Auto-refresh settings

## Styling

### TailwindCSS

Custom theme configuration in `tailwind.config.js`:

- **Color Palette**: primary, success, danger, warning, dark
- **Custom Animations**: pulse-slow, bounce-slow
- **Shadow Effects**: glow, glow-lg
- **Custom Classes**: btn, card, badge, input, table

### Custom CSS

Additional styles in `src/styles/index.css`:

- Component styles
- Utility classes
- Scrollbar customization
- Status indicators

## Key Components

### Common Components

- **Button** - Customizable button with variants and sizes
- **Card** - Container component with header support
- **Table** - Data table with sorting and pagination
- **Modal** - Overlay modal dialog
- **Loading** - Loading spinner and skeleton
- **Badge** - Status and label badges
- **Input** - Form input components

### Layout Components

- **Header** - Top navigation bar with system status
- **Sidebar** - Collapsible navigation sidebar
- **Footer** - Footer with links
- **Layout** - Main layout wrapper

## Custom Hooks

- **useAuth** - Authentication management
- **useTokens** - Token data fetching
- **useStrategies** - Strategy management
- **useTrades** - Trade history
- **usePositions** - Position monitoring
- **useMetrics** - Analytics data
- **useWebSocket** - Real-time updates

## Environment Variables

Create `.env.local` for custom configuration:

```env
VITE_API_URL=http://localhost:3000
VITE_WS_URL=ws://localhost:3000
```

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## Performance Optimization

- Code splitting by route
- Manual chunks for vendor libraries
- Lazy loading for heavy components
- React Query caching
- WebSocket connection pooling
- Optimistic updates

## Default Credentials

For development/testing:

- **Username**: admin
- **Password**: admin123

## Troubleshooting

### API Connection Issues

1. Ensure backend is running on port 3000
2. Check CORS configuration
3. Verify JWT token validity

### WebSocket Disconnection

- Auto-reconnect is enabled
- Check network connectivity
- Verify authentication token

### Build Errors

```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install

# Clear Vite cache
rm -rf node_modules/.vite
```

## Contributing

1. Follow TypeScript strict mode
2. Use functional components with hooks
3. Maintain component modularity
4. Add proper error handling
5. Include loading states
6. Write type-safe code

## License

Proprietary - SolSniper Pro © 2025
