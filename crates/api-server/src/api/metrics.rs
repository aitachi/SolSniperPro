use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::time::SystemTime;
use rand::Rng;

use crate::{
    models::{TradingMetrics, SystemMetrics, ApiResponse},
    state::AppState,
};

pub async fn get_summary(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query_as::<_, (f64, i64, i64, i64)>(
        "SELECT
            COALESCE(SUM(pnl_usd), 0) as total_pnl_usd,
            COUNT(*) as total_trades,
            COALESCE(SUM(CASE WHEN pnl_usd > 0 THEN 1 ELSE 0 END), 0) as successful_trades,
            COALESCE(SUM(CASE WHEN pnl_usd <= 0 THEN 1 ELSE 0 END), 0) as failed_trades
         FROM trades
         WHERE status = 'COMPLETED'"
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok((total_pnl_usd, total_trades, successful_trades, failed_trades)) => {
            let win_rate = if total_trades > 0 {
                (successful_trades as f64 / total_trades as f64) * 100.0
            } else {
                0.0
            };

            let metrics = TradingMetrics {
                total_pnl_usd,
                total_pnl_sol: total_pnl_usd / 100.0, // 假设 SOL 价格为 $100
                win_rate,
                total_trades,
                successful_trades,
                failed_trades,
                profit_factor: 2.5, // 模拟值
            };

            let response = serde_json::json!({
                "trading_metrics": metrics
            });

            (StatusCode::OK, Json(ApiResponse::success(response)))
        }
        Err(e) => {
            tracing::error!("Failed to fetch metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch metrics: {}", e),
                )),
            )
        }
    }
}

pub async fn get_system_metrics(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<SystemMetrics>>) {
    let ws_connections = state.ws_clients.read().await.len();
    let mut rng = rand::thread_rng();

    let metrics = SystemMetrics {
        cpu_usage: 25.0 + (rng.gen::<f64>() * 20.0), // 模拟值
        memory_usage: 40.0 + (rng.gen::<f64>() * 30.0), // 模拟值
        uptime: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
            - 3600000, // 假设运行了1小时
        api_latency: 10.0 + (rng.gen::<f64>() * 40.0), // 模拟值
        websocket_connections: ws_connections,
        status: "healthy".to_string(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    };

    (StatusCode::OK, Json(ApiResponse::success(metrics)))
}

pub async fn get_strategy_metrics(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query_as::<_, (String,)>(
        "SELECT name FROM strategies WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some((strategy_name,))) => {
            let stats_result = sqlx::query_as::<_, (i64, f64, f64)>(
                "SELECT
                    COUNT(*) as total_trades,
                    COALESCE(SUM(CASE WHEN pnl_usd > 0 THEN 1 ELSE 0 END)::float / NULLIF(COUNT(*), 0) * 100, 0) as win_rate,
                    COALESCE(SUM(pnl_usd), 0) as total_pnl
                 FROM trades
                 WHERE strategy_name = $1"
            )
            .bind(&strategy_name)
            .fetch_one(&state.db)
            .await;

            match stats_result {
                Ok((total_trades, win_rate, total_pnl)) => {
                    let metrics = serde_json::json!({
                        "total_trades": total_trades,
                        "win_rate": win_rate,
                        "total_pnl": total_pnl,
                        "sharpe_ratio": 1.85,
                    });

                    (StatusCode::OK, Json(ApiResponse::success(metrics)))
                }
                Err(e) => {
                    tracing::error!("Failed to fetch strategy stats: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            "DATABASE_ERROR".to_string(),
                            format!("Failed to fetch strategy stats: {}", e),
                        )),
                    )
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(
                "STRATEGY_NOT_FOUND".to_string(),
                format!("Strategy {} not found", id),
            )),
        ),
        Err(e) => {
            tracing::error!("Failed to fetch strategy: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch strategy: {}", e),
                )),
            )
        }
    }
}
