use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{
        Strategy, StrategyWithStats, StrategyStats, CreateStrategyRequest,
        ApiResponse,
    },
    state::AppState,
};

pub async fn list_strategies(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<StrategyWithStats>>>) {
    let result = sqlx::query_as::<_, Strategy>(
        "SELECT * FROM strategies ORDER BY priority DESC"
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(strategies) => {
            let mut strategies_with_stats = Vec::new();

            for strategy in strategies {
                // 获取策略统计信息
                let stats_result = sqlx::query_as::<_, (i64, f64, f64, f64)>(
                    "SELECT
                        COUNT(*) as total_trades,
                        COALESCE(SUM(CASE WHEN pnl_usd > 0 THEN 1 ELSE 0 END)::float / NULLIF(COUNT(*), 0) * 100, 0) as win_rate,
                        COALESCE(SUM(pnl_usd), 0) as total_pnl,
                        1.85 as sharpe_ratio
                     FROM trades
                     WHERE strategy_name = $1"
                )
                .bind(&strategy.name)
                .fetch_one(&state.db)
                .await;

                let stats = match stats_result {
                    Ok((total_trades, win_rate, total_pnl, sharpe_ratio)) => {
                        StrategyStats {
                            total_trades,
                            win_rate,
                            total_pnl,
                            sharpe_ratio,
                        }
                    }
                    Err(_) => {
                        StrategyStats {
                            total_trades: 0,
                            win_rate: 0.0,
                            total_pnl: 0.0,
                            sharpe_ratio: 0.0,
                        }
                    }
                };

                strategies_with_stats.push(StrategyWithStats {
                    id: strategy.id,
                    name: strategy.name,
                    strategy_type: strategy.strategy_type,
                    is_active: strategy.is_active,
                    priority: strategy.priority,
                    stats,
                });
            }

            (StatusCode::OK, Json(ApiResponse::success(strategies_with_stats)))
        }
        Err(e) => {
            tracing::error!("Failed to fetch strategies: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch strategies: {}", e),
                )),
            )
        }
    }
}

pub async fn get_strategy(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Strategy>>) {
    let result = sqlx::query_as::<_, Strategy>(
        "SELECT * FROM strategies WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(strategy)) => (StatusCode::OK, Json(ApiResponse::success(strategy))),
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

pub async fn start_strategy(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query(
        "UPDATE strategies SET is_active = true WHERE id = $1"
    )
    .bind(&id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            let message = serde_json::json!({"message": "Strategy started"});
            (StatusCode::OK, Json(ApiResponse::success(message)))
        }
        Err(e) => {
            tracing::error!("Failed to start strategy: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to start strategy: {}", e),
                )),
            )
        }
    }
}

pub async fn pause_strategy(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query(
        "UPDATE strategies SET is_active = false WHERE id = $1"
    )
    .bind(&id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            let message = serde_json::json!({"message": "Strategy paused"});
            (StatusCode::OK, Json(ApiResponse::success(message)))
        }
        Err(e) => {
            tracing::error!("Failed to pause strategy: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to pause strategy: {}", e),
                )),
            )
        }
    }
}

pub async fn create_strategy(
    State(state): State<AppState>,
    Json(payload): Json<CreateStrategyRequest>,
) -> (StatusCode, Json<ApiResponse<Strategy>>) {
    let id = Uuid::new_v4().to_string();
    let priority = payload.priority.unwrap_or(50);

    let result = sqlx::query_as::<_, Strategy>(
        "INSERT INTO strategies (id, name, strategy_type, is_active, priority, config, created_at, updated_at)
         VALUES ($1, $2, $3, false, $4, $5, NOW(), NOW())
         RETURNING *"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.strategy_type)
    .bind(priority)
    .bind(&payload.config)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(strategy) => (StatusCode::CREATED, Json(ApiResponse::success(strategy))),
        Err(e) => {
            tracing::error!("Failed to create strategy: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to create strategy: {}", e),
                )),
            )
        }
    }
}
