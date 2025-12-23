-- SolSniper Pro - 简化数据库初始化脚本
-- 兼容 Rust API Server models.rs
-- Author: Aitachi
-- Date: 2025-12-23

-- 创建扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ==================== 用户表 ====================
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4()::text,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'USER',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 插入默认管理员 (密码: admin123)
INSERT INTO users (id, username, password_hash, role)
VALUES ('user1', 'admin', 'admin123_plain_text_for_demo', 'ADMIN')
ON CONFLICT (username) DO NOTHING;

-- ==================== 代币表 ====================
CREATE TABLE IF NOT EXISTS tokens (
    id VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4()::text,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    mint VARCHAR(100) UNIQUE NOT NULL,
    liquidity DOUBLE PRECISION DEFAULT 0,
    holders INT DEFAULT 0,
    price DOUBLE PRECISION DEFAULT 0,
    price_change_1h DOUBLE PRECISION DEFAULT 0,
    age INT DEFAULT 0,
    risk_score INT DEFAULT 0,
    is_renounced BOOLEAN DEFAULT FALSE,
    is_immutable BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- 插入示例代币
INSERT INTO tokens (id, symbol, name, mint, liquidity, holders, price, price_change_1h, age, risk_score, is_renounced, is_immutable)
VALUES
    ('token1', 'BONK', 'Bonk Token', '5t1dC...abc123', 150000, 1500, 0.000025, 12.5, 5, 85, TRUE, FALSE),
    ('token2', 'WIF', 'dogwifhat', '8x2yN...def456', 250000, 2500, 0.000045, -5.3, 12, 78, TRUE, TRUE)
ON CONFLICT (mint) DO NOTHING;

-- ==================== 策略表 ====================
CREATE TABLE IF NOT EXISTS strategies (
    id VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4()::text,
    name VARCHAR(100) NOT NULL,
    strategy_type VARCHAR(50) NOT NULL,
    is_active BOOLEAN DEFAULT FALSE,
    priority INT DEFAULT 50,
    config JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 插入示例策略
INSERT INTO strategies (id, name, strategy_type, is_active, priority, config)
VALUES
    ('strategy1', 'Early Bird', 'EarlyBird', TRUE, 90, '{"min_liquidity_sol": 50, "max_age_hours": 1}'::jsonb),
    ('strategy2', 'Liquidity Hunter', 'LiquidityHunter', TRUE, 80, '{"min_liquidity_sol": 100}'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- ==================== 持仓表 ====================
CREATE TABLE IF NOT EXISTS positions (
    id VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4()::text,
    token_symbol VARCHAR(20) NOT NULL,
    token_mint VARCHAR(100) NOT NULL,
    strategy_name VARCHAR(100) NOT NULL,
    entry_price_usd DOUBLE PRECISION NOT NULL,
    current_price_usd DOUBLE PRECISION NOT NULL,
    amount_sol DOUBLE PRECISION NOT NULL,
    invested_usd DOUBLE PRECISION NOT NULL,
    current_value_usd DOUBLE PRECISION NOT NULL,
    pnl_usd DOUBLE PRECISION NOT NULL,
    pnl_percentage DOUBLE PRECISION NOT NULL,
    holding_time BIGINT NOT NULL,
    status VARCHAR(20) DEFAULT 'ACTIVE',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 插入示例持仓
INSERT INTO positions (id, token_symbol, token_mint, strategy_name, entry_price_usd, current_price_usd,
                      amount_sol, invested_usd, current_value_usd, pnl_usd, pnl_percentage, holding_time, status)
VALUES
    ('pos1', 'BONK', '5t1dC...abc123', 'Early Bird', 0.00002, 0.000025, 2.5, 50, 62.5, 12.5, 25.0, 7200000, 'ACTIVE'),
    ('pos2', 'WIF', '8x2yN...def456', 'Liquidity Hunter', 0.000048, 0.000045, 1.5, 75, 70.7, -4.3, -5.7, 3600000, 'ACTIVE')
ON CONFLICT (id) DO NOTHING;

-- ==================== 交易表 ====================
CREATE TABLE IF NOT EXISTS trades (
    id VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4()::text,
    created_at TIMESTAMP DEFAULT NOW(),
    side VARCHAR(10) NOT NULL, -- BUY / SELL
    token_symbol VARCHAR(20) NOT NULL,
    token_mint VARCHAR(100) NOT NULL,
    strategy_name VARCHAR(100) NOT NULL,
    amount_usd DOUBLE PRECISION NOT NULL,
    amount_sol DOUBLE PRECISION NOT NULL,
    price_usd DOUBLE PRECISION NOT NULL,
    status VARCHAR(20) DEFAULT 'PENDING', -- PENDING, COMPLETED, FAILED
    pnl_usd DOUBLE PRECISION,
    tx_hash VARCHAR(100)
);

-- 插入示例交易
INSERT INTO trades (id, side, token_symbol, token_mint, strategy_name, amount_usd, amount_sol, price_usd, status, pnl_usd, tx_hash)
VALUES
    ('trade1', 'BUY', 'BONK', '5t1dC...abc123', 'Early Bird', 50, 2.5, 0.00002, 'COMPLETED', NULL, '4kX...abc'),
    ('trade2', 'SELL', 'WIF', '8x2yN...def456', 'Liquidity Hunter', 75, 1.5, 0.000048, 'COMPLETED', 15.5, '7yZ...def'),
    ('trade3', 'BUY', 'SAMO', '9aB...ghi', 'Volume Explosion', 100, 5.0, 0.00012, 'COMPLETED', NULL, '2mN...ghi')
ON CONFLICT (id) DO NOTHING;

-- ==================== 风险限制表 ====================
CREATE TABLE IF NOT EXISTS risk_limits (
    id SERIAL PRIMARY KEY,
    max_position_size_sol DOUBLE PRECISION DEFAULT 10.0,
    max_position_size_percent DOUBLE PRECISION DEFAULT 20.0,
    max_total_exposure_sol DOUBLE PRECISION DEFAULT 100.0,
    max_positions INT DEFAULT 10,
    max_loss_per_trade_sol DOUBLE PRECISION DEFAULT 2.0,
    max_daily_loss_sol DOUBLE PRECISION DEFAULT 10.0,
    max_drawdown_percent DOUBLE PRECISION DEFAULT 20.0,
    min_risk_score INT DEFAULT 70,
    max_risk_score INT DEFAULT 95,
    block_extreme_risk BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- 插入默认风险限制
INSERT INTO risk_limits (max_position_size_sol, max_position_size_percent, max_total_exposure_sol,
                        max_positions, max_loss_per_trade_sol, max_daily_loss_sol, max_drawdown_percent,
                        min_risk_score, max_risk_score, block_extreme_risk)
VALUES (10.0, 20.0, 100.0, 10, 2.0, 10.0, 20.0, 70, 95, TRUE);

-- ==================== 索引 ====================
CREATE INDEX IF NOT EXISTS idx_tokens_symbol ON tokens(symbol);
CREATE INDEX IF NOT EXISTS idx_tokens_mint ON tokens(mint);
CREATE INDEX IF NOT EXISTS idx_tokens_created_at ON tokens(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_strategies_active ON strategies(is_active);
CREATE INDEX IF NOT EXISTS idx_strategies_priority ON strategies(priority DESC);

CREATE INDEX IF NOT EXISTS idx_positions_status ON positions(status);
CREATE INDEX IF NOT EXISTS idx_positions_created_at ON positions(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_trades_created_at ON trades(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_trades_strategy ON trades(strategy_name);
CREATE INDEX IF NOT EXISTS idx_trades_status ON trades(status);

-- ==================== 触发器：自动更新 updated_at ====================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_strategies_updated_at BEFORE UPDATE ON strategies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_positions_updated_at BEFORE UPDATE ON positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 完成
SELECT 'Simplified database initialization completed!' as status;
