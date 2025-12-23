use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    models::{RiskLimits, UpdateRiskLimitsRequest, ApiResponse},
    state::AppState,
};

pub async fn get_limits(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<RiskLimits>>) {
    let result = sqlx::query_as::<_, RiskLimits>(
        "SELECT * FROM risk_limits ORDER BY id DESC LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(limits)) => (StatusCode::OK, Json(ApiResponse::success(limits))),
        Ok(None) => {
            // 返回默认值
            let default_limits = RiskLimits {
                max_position_size_sol: 10.0,
                max_position_size_percent: 20.0,
                max_total_exposure_sol: 100.0,
                max_positions: 10,
                max_loss_per_trade_sol: 2.0,
                max_daily_loss_sol: 10.0,
                max_drawdown_percent: 20.0,
                min_risk_score: 70,
                max_risk_score: 95,
                block_extreme_risk: true,
            };
            (StatusCode::OK, Json(ApiResponse::success(default_limits)))
        }
        Err(e) => {
            tracing::error!("Failed to fetch risk limits: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch risk limits: {}", e),
                )),
            )
        }
    }
}

pub async fn update_limits(
    State(state): State<AppState>,
    Json(payload): Json<UpdateRiskLimitsRequest>,
) -> (StatusCode, Json<ApiResponse<RiskLimits>>) {
    // 获取当前限制
    let current_result = sqlx::query_as::<_, RiskLimits>(
        "SELECT * FROM risk_limits ORDER BY id DESC LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await;

    let current = current_result.ok().flatten().unwrap_or(RiskLimits {
        max_position_size_sol: 10.0,
        max_position_size_percent: 20.0,
        max_total_exposure_sol: 100.0,
        max_positions: 10,
        max_loss_per_trade_sol: 2.0,
        max_daily_loss_sol: 10.0,
        max_drawdown_percent: 20.0,
        min_risk_score: 70,
        max_risk_score: 95,
        block_extreme_risk: true,
    });

    // 更新值
    let updated = RiskLimits {
        max_position_size_sol: payload.max_position_size_sol.unwrap_or(current.max_position_size_sol),
        max_position_size_percent: payload.max_position_size_percent.unwrap_or(current.max_position_size_percent),
        max_total_exposure_sol: payload.max_total_exposure_sol.unwrap_or(current.max_total_exposure_sol),
        max_positions: payload.max_positions.unwrap_or(current.max_positions),
        max_loss_per_trade_sol: payload.max_loss_per_trade_sol.unwrap_or(current.max_loss_per_trade_sol),
        max_daily_loss_sol: payload.max_daily_loss_sol.unwrap_or(current.max_daily_loss_sol),
        max_drawdown_percent: payload.max_drawdown_percent.unwrap_or(current.max_drawdown_percent),
        min_risk_score: payload.min_risk_score.unwrap_or(current.min_risk_score),
        max_risk_score: payload.max_risk_score.unwrap_or(current.max_risk_score),
        block_extreme_risk: payload.block_extreme_risk.unwrap_or(current.block_extreme_risk),
    };

    // 保存到数据库
    let result = sqlx::query(
        "INSERT INTO risk_limits
         (max_position_size_sol, max_position_size_percent, max_total_exposure_sol,
          max_positions, max_loss_per_trade_sol, max_daily_loss_sol, max_drawdown_percent,
          min_risk_score, max_risk_score, block_extreme_risk, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())"
    )
    .bind(updated.max_position_size_sol)
    .bind(updated.max_position_size_percent)
    .bind(updated.max_total_exposure_sol)
    .bind(updated.max_positions)
    .bind(updated.max_loss_per_trade_sol)
    .bind(updated.max_daily_loss_sol)
    .bind(updated.max_drawdown_percent)
    .bind(updated.min_risk_score)
    .bind(updated.max_risk_score)
    .bind(updated.block_extreme_risk)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(ApiResponse::success(updated))),
        Err(e) => {
            tracing::error!("Failed to update risk limits: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to update risk limits: {}", e),
                )),
            )
        }
    }
}

pub async fn get_risk_status(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query_as::<_, (i64, f64, f64)>(
        "SELECT
            COUNT(*) as active_positions,
            COALESCE(SUM(invested_usd), 0) as total_exposure,
            COALESCE(SUM(pnl_usd), 0) as current_pnl
         FROM positions
         WHERE status = 'ACTIVE'"
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok((active_positions, total_exposure, current_pnl)) => {
            let status = serde_json::json!({
                "active_positions": active_positions,
                "total_exposure_usd": total_exposure,
                "current_pnl_usd": current_pnl,
                "risk_level": if total_exposure > 80.0 { "HIGH" } else { "NORMAL" },
            });

            (StatusCode::OK, Json(ApiResponse::success(status)))
        }
        Err(e) => {
            tracing::error!("Failed to fetch risk status: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch risk status: {}", e),
                )),
            )
        }
    }
}
