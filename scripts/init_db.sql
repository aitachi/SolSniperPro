-- SolSniper Pro 数据库初始化脚本
-- 版本: v2.0.0
-- 日期: 2025-12-21
--
-- Author: Aitachi
-- Email: 44158892@qq.com
-- Wechat: 18116011230

-- 创建扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";  -- 用于文本搜索

-- ============================================================================
-- 代币表
-- ============================================================================
CREATE TABLE IF NOT EXISTS tokens (
    mint VARCHAR(44) PRIMARY KEY,
    symbol VARCHAR(20),
    name VARCHAR(100),
    decimals INTEGER,

    -- 完整的 TokenInfo JSON 数据
    data JSONB NOT NULL,

    -- 常用字段（冗余，便于查询）
    liquidity_sol NUMERIC(20, 9),
    liquidity_usd NUMERIC(20, 2),
    holders_count INTEGER,
    age_hours NUMERIC(10, 2),
    price_usd NUMERIC(30, 18),
    volume_1h NUMERIC(20, 2),
    volume_24h NUMERIC(20, 2),

    -- 风险评分
    risk_score NUMERIC(5, 2),
    risk_breakdown JSONB,

    -- 时间戳
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_tokens_created_at ON tokens(created_at DESC);
CREATE INDEX idx_tokens_liquidity_sol ON tokens(liquidity_sol DESC NULLS LAST);
CREATE INDEX idx_tokens_risk_score ON tokens(risk_score DESC NULLS LAST);
CREATE INDEX idx_tokens_age_hours ON tokens(age_hours ASC NULLS LAST);
CREATE INDEX idx_tokens_symbol ON tokens USING GIN(symbol gin_trgm_ops);

-- JSONB 索引
CREATE INDEX idx_tokens_data_gin ON tokens USING GIN(data);

-- ============================================================================
-- 交易表
-- ============================================================================
CREATE TABLE IF NOT EXISTS trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mint VARCHAR(44) REFERENCES tokens(mint) ON DELETE CASCADE,

    -- 交易信息
    side VARCHAR(4) NOT NULL CHECK (side IN ('buy', 'sell')),
    strategy VARCHAR(50),
    amount_sol NUMERIC(20, 9),
    amount_tokens NUMERIC(30, 0),
    price_usd NUMERIC(30, 18),

    -- 状态
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    -- pending, executed, failed, cancelled

    -- 交易参数
    slippage_bps INTEGER,
    priority_fee BIGINT,

    -- 执行信息
    tx_signature VARCHAR(88),
    slot BIGINT,
    block_time BIGINT,
    fee_lamports BIGINT,

    -- 持仓信息（卖出时记录）
    entry_price NUMERIC(30, 18),
    exit_price NUMERIC(30, 18),
    pnl_sol NUMERIC(20, 9),
    pnl_usd NUMERIC(20, 2),
    pnl_pct NUMERIC(10, 2),
    holding_duration_secs INTEGER,

    -- 时间戳
    created_at TIMESTAMP DEFAULT NOW(),
    executed_at TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_trades_created_at ON trades(created_at DESC);
CREATE INDEX idx_trades_mint ON trades(mint);
CREATE INDEX idx_trades_status ON trades(status);
CREATE INDEX idx_trades_strategy ON trades(strategy);
CREATE INDEX idx_trades_executed_at ON trades(executed_at DESC NULLS LAST);

-- ============================================================================
-- 活跃持仓表
-- ============================================================================
CREATE TABLE IF NOT EXISTS positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mint VARCHAR(44) REFERENCES tokens(mint) ON DELETE CASCADE,
    trade_id UUID REFERENCES trades(id) ON DELETE CASCADE,

    -- 持仓信息
    entry_price NUMERIC(30, 18) NOT NULL,
    amount_sol NUMERIC(20, 9) NOT NULL,
    amount_tokens NUMERIC(30, 0) NOT NULL,

    -- 当前状态
    current_price NUMERIC(30, 18),
    highest_price NUMERIC(30, 18),
    lowest_price NUMERIC(30, 18),
    unrealized_pnl_sol NUMERIC(20, 9),
    unrealized_pnl_pct NUMERIC(10, 2),

    -- 退出策略配置
    exit_strategy_config JSONB,

    -- 时间
    entry_time TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    UNIQUE(mint)
);

-- 索引
CREATE INDEX idx_positions_mint ON positions(mint);
CREATE INDEX idx_positions_entry_time ON positions(entry_time DESC);

-- ============================================================================
-- 策略性能统计表
-- ============================================================================
CREATE TABLE IF NOT EXISTS strategy_stats (
    id SERIAL PRIMARY KEY,
    strategy_name VARCHAR(50) NOT NULL,
    date DATE NOT NULL,

    -- 统计数据
    total_signals INTEGER DEFAULT 0,
    executed_trades INTEGER DEFAULT 0,
    winning_trades INTEGER DEFAULT 0,
    losing_trades INTEGER DEFAULT 0,

    -- 性能指标
    win_rate NUMERIC(5, 4),
    avg_return_pct NUMERIC(10, 2),
    total_pnl_sol NUMERIC(20, 9),
    total_pnl_usd NUMERIC(20, 2),
    sharpe_ratio NUMERIC(10, 4),
    max_drawdown NUMERIC(10, 4),

    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    UNIQUE(strategy_name, date)
);

-- 索引
CREATE INDEX idx_strategy_stats_name_date ON strategy_stats(strategy_name, date DESC);

-- ============================================================================
-- 系统事件日志表
-- ============================================================================
CREATE TABLE IF NOT EXISTS system_events (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL, -- info, warning, error, critical
    message TEXT NOT NULL,
    details JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_system_events_created_at ON system_events(created_at DESC);
CREATE INDEX idx_system_events_type ON system_events(event_type);
CREATE INDEX idx_system_events_severity ON system_events(severity);

-- ============================================================================
-- 黑名单表
-- ============================================================================
CREATE TABLE IF NOT EXISTS blacklist (
    id SERIAL PRIMARY KEY,
    type VARCHAR(20) NOT NULL, -- token, creator
    address VARCHAR(44) NOT NULL,
    reason TEXT,
    added_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),

    UNIQUE(type, address)
);

-- 索引
CREATE INDEX idx_blacklist_type_address ON blacklist(type, address);

-- ============================================================================
-- 触发器：自动更新 updated_at
-- ============================================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tokens_updated_at BEFORE UPDATE ON tokens
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_trades_updated_at BEFORE UPDATE ON trades
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_positions_updated_at BEFORE UPDATE ON positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_strategy_stats_updated_at BEFORE UPDATE ON strategy_stats
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- 初始化数据
-- ============================================================================

-- 插入示例黑名单（可选）
-- INSERT INTO blacklist (type, address, reason, added_by)
-- VALUES
--     ('token', 'scam_token_mint_address', 'Confirmed rug pull', 'system'),
--     ('creator', 'scam_creator_address', 'Multiple rug pulls', 'system');

-- ============================================================================
-- 清理函数：删除旧数据
-- ============================================================================
CREATE OR REPLACE FUNCTION cleanup_old_data()
RETURNS void AS $$
BEGIN
    -- 删除 30 天前的代币数据
    DELETE FROM tokens WHERE created_at < NOW() - INTERVAL '30 days';

    -- 删除 90 天前的交易记录
    DELETE FROM trades WHERE created_at < NOW() - INTERVAL '90 days';

    -- 删除 30 天前的系统事件
    DELETE FROM system_events WHERE created_at < NOW() - INTERVAL '30 days';

    -- 删除 180 天前的策略统计
    DELETE FROM strategy_stats WHERE date < NOW() - INTERVAL '180 days';

    RAISE NOTICE 'Old data cleanup completed';
END;
$$ LANGUAGE plpgsql;

-- 可以通过 cron 定期执行
-- SELECT cleanup_old_data();

-- ============================================================================
-- 视图：活跃代币
-- ============================================================================
CREATE OR REPLACE VIEW active_tokens AS
SELECT
    t.*,
    p.unrealized_pnl_sol,
    p.unrealized_pnl_pct,
    CASE WHEN p.id IS NOT NULL THEN true ELSE false END as has_position
FROM tokens t
LEFT JOIN positions p ON t.mint = p.mint
WHERE t.created_at > NOW() - INTERVAL '24 hours'
ORDER BY t.created_at DESC;

-- ============================================================================
-- 视图：交易概览
-- ============================================================================
CREATE OR REPLACE VIEW trade_summary AS
SELECT
    DATE(executed_at) as trade_date,
    COUNT(*) as total_trades,
    SUM(CASE WHEN pnl_sol > 0 THEN 1 ELSE 0 END) as winning_trades,
    SUM(CASE WHEN pnl_sol < 0 THEN 1 ELSE 0 END) as losing_trades,
    SUM(pnl_sol) as total_pnl_sol,
    SUM(pnl_usd) as total_pnl_usd,
    AVG(CASE WHEN pnl_sol > 0 THEN pnl_pct END) as avg_win_pct,
    AVG(CASE WHEN pnl_sol < 0 THEN pnl_pct END) as avg_loss_pct
FROM trades
WHERE executed_at IS NOT NULL
GROUP BY DATE(executed_at)
ORDER BY trade_date DESC;

-- 完成
\echo 'Database initialization completed successfully!'
