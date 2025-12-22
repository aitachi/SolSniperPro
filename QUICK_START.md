# SolSniper Pro - Quick Start Guide
---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

**Complete System Startup Guide**

---

**Author**: Aitachi
**Email**: 44158892@qq.com
**Wechat**: 18116011230

---

## Prerequisites

### Required Software
- Rust 1.75+ (`rustc --version`)
- Node.js 18+ (`node --version`)
- Docker & Docker Compose (`docker --version`)
- PostgreSQL 16 (via Docker or local)
- Redis 7 (via Docker or local)
- Kafka (via Docker or local)

---

## 🚀 Quick Start (Automated)

### Option 1: Using the Startup Script

```bash
# Navigate to project root
cd SolSniperPro-main

# Make script executable
chmod +x scripts/start.sh

# Start everything
./scripts/start.sh

# Or force clean start
./scripts/start.sh --force

# Or production mode
./scripts/start.sh --production
```

The script will automatically:
1. ✅ Check all dependencies
2. ✅ Create required directories
3. ✅ Validate configuration files
4. ✅ Start Docker containers (Kafka, PostgreSQL, Redis)
5. ✅ Initialize database
6. ✅ Build backend (Rust)
7. ✅ Start API server
8. ✅ Build and start frontend

### Check Status

After startup, you'll see:

```
=====================================
   SolSniper Pro - System Status
=====================================

✅ Docker Containers: Running
✅ PostgreSQL: Ready (port 5432)
✅ Redis: Ready (port 6379)
✅ Kafka: Ready (port 9092)
✅ Backend API: Running (port 3000)
✅ Frontend: Running (port 5173)

=====================================
   Access Points
=====================================

Frontend:  http://localhost:5173
API:       http://localhost:3000
API Docs:  http://localhost:3000/api/v1/docs

Default Login:
  Username: admin
  Password: admin123

=====================================
```

---

## 🔧 Manual Setup (Step by Step)

### Step 1: Start Infrastructure

```bash
# Start Docker containers
docker-compose up -d

# Verify containers are running
docker ps

# Expected output:
# - solsniper-postgres (port 5432)
# - solsniper-redis (port 6379)
# - solsniper-kafka (port 9092)
# - solsniper-zookeeper (port 2181)
```

### Step 2: Initialize Database

```bash
# Connect to PostgreSQL
psql -h localhost -U solsniper -d solsniper_db

# Run initialization script
\i scripts/init_db.sql

# Verify tables
\dt

# Exit
\q
```

### Step 3: Configure Environment

Create `.env` file in project root:

```env
# Database
DATABASE_URL=postgresql://solsniper:solsniper123@localhost:5432/solsniper_db

# Redis
REDIS_URL=redis://localhost:6379

# Kafka
KAFKA_BROKERS=localhost:9092

# Solana RPC
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_WS_URL=wss://api.mainnet-beta.solana.com

# Server
API_PORT=3000
LOG_LEVEL=info

# Security
JWT_SECRET=your-secret-key-change-in-production
```

### Step 4: Build and Run Backend

```bash
# Build in release mode
cargo build --release

# Run API server
cargo run --bin api-server --release

# Or run separately
./target/release/api-server
```

Expected output:
```
[INFO] Starting SolSniper Pro API Server
[INFO] Database connected: PostgreSQL
[INFO] Redis connected: localhost:6379
[INFO] Kafka connected: localhost:9092
[INFO] API server listening on 0.0.0.0:3000
[INFO] WebSocket server ready
```

### Step 5: Build and Run Frontend

```bash
# Navigate to frontend
cd frontend

# Install dependencies
npm install

# Development mode
npm run dev

# Or production build
npm run build
npm run preview
```

Expected output:
```
VITE v5.0.8  ready in 324 ms

➜  Local:   http://localhost:5173/
➜  Network: http://192.168.1.100:5173/
```

---

## 📱 Accessing the Application

### 1. Open Browser

Visit: **http://localhost:5173**

### 2. Login

```
Username: admin
Password: admin123
```

### 3. Explore Features

- **Dashboard** - Trading metrics overview
- **Tokens** - Monitor new tokens
- **Strategies** - Manage trading strategies
- **Trading** - View trade history
- **Positions** - Monitor active positions
- **Risk Control** - Configure risk settings
- **Analytics** - Performance analysis
- **Settings** - User preferences

---

## 🛠️ Troubleshooting

### Backend Won't Start

**Problem**: Database connection error

```bash
# Check PostgreSQL is running
docker ps | grep postgres

# Check connection
psql -h localhost -U solsniper -d solsniper_db -c "SELECT 1;"

# Restart container
docker restart solsniper-postgres
```

**Problem**: Kafka connection error

```bash
# Check Kafka is running
docker ps | grep kafka

# View Kafka logs
docker logs solsniper-kafka

# Restart
docker restart solsniper-kafka solsniper-zookeeper
```

### Frontend Won't Connect

**Problem**: API not reachable

```bash
# Check API is running
curl http://localhost:3000/api/v1/health

# Check proxy configuration
cat frontend/vite.config.ts | grep proxy
```

**Problem**: WebSocket disconnected

```bash
# Check WebSocket endpoint
curl -i -N \
  -H "Connection: Upgrade" \
  -H "Upgrade: websocket" \
  http://localhost:3000/ws

# Should return: 101 Switching Protocols
```

### Database Issues

```bash
# Reset database
docker stop solsniper-postgres
docker rm solsniper-postgres
docker volume rm solsniper_postgres_data

# Restart
docker-compose up -d postgres

# Re-initialize
psql -h localhost -U solsniper -d solsniper_db -f scripts/init_db.sql
```

---

## 🛑 Stopping the System

### Stop Everything

```bash
# Use stop script
./scripts/stop.sh
```

Or manually:

```bash
# Stop frontend (Ctrl+C in terminal)

# Stop backend (Ctrl+C in terminal)

# Stop Docker containers
docker-compose down

# Or stop and remove volumes
docker-compose down -v
```

---

## 📊 System Monitoring

### Check Logs

**Backend Logs**:
```bash
# If running with systemd
journalctl -u solsniper-api -f

# If running in terminal
# Logs appear in console
```

**Docker Logs**:
```bash
# PostgreSQL
docker logs -f solsniper-postgres

# Redis
docker logs -f solsniper-redis

# Kafka
docker logs -f solsniper-kafka
```

### Monitor Resources

```bash
# Check Docker resources
docker stats

# Check API health
curl http://localhost:3000/api/v1/health

# Check system metrics
curl http://localhost:3000/api/v1/metrics/summary
```

---

## 🔐 Security Checklist

Before deploying to production:

- [ ] Change `JWT_SECRET` in `.env`
- [ ] Change database password
- [ ] Change Redis password (if exposed)
- [ ] Enable HTTPS/TLS
- [ ] Configure firewall rules
- [ ] Set up monitoring and alerts
- [ ] Enable rate limiting
- [ ] Review CORS configuration
- [ ] Implement backup strategy
- [ ] Update default admin password

---

## 📚 Additional Resources

- **Architecture Documentation**: `docs/01_SYSTEM_ARCHITECTURE.md`
- **Strategy Guide**: `docs/02_STRATEGY_GUIDE.md`
- **API Reference**: `docs/03_API_REFERENCE.md`
- **Deployment Guide**: `docs/04_DEPLOYMENT_GUIDE.md`
- **Frontend Architecture**: `docs/05_FRONTEND_ARCHITECTURE.md`
- **Implementation Summary**: `docs/06_FRONTEND_IMPLEMENTATION_SUMMARY.md`

---

## 🆘 Getting Help

### Check Documentation

1. Read relevant doc files in `docs/` directory
2. Check component READMEs in respective folders
3. Review code comments for specific implementations

### Common Issues

| Issue | Solution |
|-------|----------|
| Port already in use | Change port in config or kill process |
| Docker won't start | Check Docker Desktop is running |
| Database error | Verify credentials and connection |
| Build fails | Clear cache: `cargo clean && npm clean-install` |
| WebSocket error | Check backend is running and accessible |

---

## ✅ Success Indicators

Your system is working correctly when:

1. ✅ All Docker containers show as "Up" status
2. ✅ Backend logs show "API server listening"
3. ✅ Frontend loads without console errors
4. ✅ Login succeeds and redirects to dashboard
5. ✅ WebSocket shows "Connected" (check dev tools)
6. ✅ Dashboard displays metrics (even if zero)
7. ✅ Navigation between pages works
8. ✅ Real-time updates appear (check console)

---

## 🚀 Next Steps

After successful startup:

1. **Configure Strategies**:
   - Go to Strategies page
   - Review default strategies
   - Enable strategies you want to use
   - Adjust parameters as needed

2. **Set Risk Limits**:
   - Go to Risk Control page
   - Set position size limits
   - Configure loss limits
   - Set risk score thresholds

3. **Monitor Tokens**:
   - Go to Tokens page
   - Adjust filters to your preference
   - Watch for new token discoveries

4. **Review Dashboard**:
   - Monitor active positions
   - Track PnL in real-time
   - Check recent trades

---

**Happy Trading! 🚀📈**

For issues, check logs and documentation.
For feature requests, update strategy configurations.

---

**Document Version**: v2.0.0
**Last Updated**: 2025-12-21
