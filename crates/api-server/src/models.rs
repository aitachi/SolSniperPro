use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== Request/Response Models ====================

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError { code, message }),
        }
    }
}

// ==================== Database Models ====================

#[derive(Debug, Serialize, FromRow)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub mint: String,
    pub liquidity: f64,
    pub holders: i32,
    pub price: f64,
    pub price_change_1h: f64,
    pub age: i32,
    pub risk_score: i32,
    pub is_renounced: bool,
    pub is_immutable: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Strategy {
    pub id: String,
    pub name: String,
    pub strategy_type: String,
    pub is_active: bool,
    pub priority: i32,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StrategyWithStats {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub strategy_type: String,
    pub is_active: bool,
    pub priority: i32,
    pub stats: StrategyStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyStats {
    #[serde(rename = "totalTrades")]
    pub total_trades: i64,
    #[serde(rename = "winRate")]
    pub win_rate: f64,
    #[serde(rename = "totalPnl")]
    pub total_pnl: f64,
    #[serde(rename = "sharpeRatio")]
    pub sharpe_ratio: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateStrategyRequest {
    pub name: String,
    pub strategy_type: String,
    pub priority: Option<i32>,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Position {
    pub id: String,
    pub token_symbol: String,
    pub token_mint: String,
    pub strategy_name: String,
    pub entry_price_usd: f64,
    pub current_price_usd: f64,
    pub amount_sol: f64,
    pub invested_usd: f64,
    pub current_value_usd: f64,
    pub pnl_usd: f64,
    pub pnl_percentage: f64,
    pub holding_time: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Trade {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub side: String, // BUY / SELL
    pub token_symbol: String,
    pub token_mint: String,
    pub strategy_name: String,
    pub amount_usd: f64,
    pub amount_sol: f64,
    pub price_usd: f64,
    pub status: String, // PENDING, COMPLETED, FAILED
    pub pnl_usd: Option<f64>,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TradingMetrics {
    pub total_pnl_usd: f64,
    pub total_pnl_sol: f64,
    pub win_rate: f64,
    pub total_trades: i64,
    pub successful_trades: i64,
    pub failed_trades: i64,
    pub profit_factor: f64,
}

#[derive(Debug, Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub uptime: i64,
    pub api_latency: f64,
    pub websocket_connections: usize,
    pub status: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RiskLimits {
    pub max_position_size_sol: f64,
    pub max_position_size_percent: f64,
    pub max_total_exposure_sol: f64,
    pub max_positions: i32,
    pub max_loss_per_trade_sol: f64,
    pub max_daily_loss_sol: f64,
    pub max_drawdown_percent: f64,
    pub min_risk_score: i32,
    pub max_risk_score: i32,
    pub block_extreme_risk: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRiskLimitsRequest {
    pub max_position_size_sol: Option<f64>,
    pub max_position_size_percent: Option<f64>,
    pub max_total_exposure_sol: Option<f64>,
    pub max_positions: Option<i32>,
    pub max_loss_per_trade_sol: Option<f64>,
    pub max_daily_loss_sol: Option<f64>,
    pub max_drawdown_percent: Option<f64>,
    pub min_risk_score: Option<i32>,
    pub max_risk_score: Option<i32>,
    pub block_extreme_risk: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i32,
    pub limit: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}
