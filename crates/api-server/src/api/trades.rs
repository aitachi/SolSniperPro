use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    models::{Trade, PaginatedResponse, PaginationMeta, ApiResponse},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct TradesQuery {
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

pub async fn list_trades(
    State(state): State<AppState>,
    Query(params): Query<TradesQuery>,
) -> (StatusCode, Json<PaginatedResponse<Trade>>) {
    let limit = params.limit.unwrap_or(10).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let total_result = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM trades"
    )
    .fetch_one(&state.db)
    .await;

    let trades_result = sqlx::query_as::<_, Trade>(
        "SELECT * FROM trades ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await;

    match (total_result, trades_result) {
        (Ok(total), Ok(trades)) => {
            let total_pages = ((total as f64) / (limit as f64)).ceil() as i32;
            let pagination = PaginationMeta {
                total,
                page,
                limit,
                total_pages,
            };

            (
                StatusCode::OK,
                Json(PaginatedResponse {
                    success: true,
                    data: trades,
                    pagination,
                }),
            )
        }
        (Err(e), _) | (_, Err(e)) => {
            tracing::error!("Failed to fetch trades: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(PaginatedResponse {
                    success: false,
                    data: vec![],
                    pagination: PaginationMeta {
                        total: 0,
                        page: 1,
                        limit,
                        total_pages: 0,
                    },
                }),
            )
        }
    }
}

pub async fn get_trade(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Trade>>) {
    let result = sqlx::query_as::<_, Trade>(
        "SELECT * FROM trades WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(trade)) => (StatusCode::OK, Json(ApiResponse::success(trade))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(
                "TRADE_NOT_FOUND".to_string(),
                format!("Trade {} not found", id),
            )),
        ),
        Err(e) => {
            tracing::error!("Failed to fetch trade: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch trade: {}", e),
                )),
            )
        }
    }
}
