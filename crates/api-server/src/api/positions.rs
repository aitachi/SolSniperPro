use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{Position, ApiResponse},
    state::AppState,
};

pub async fn list_positions(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<Position>>>) {
    let result = sqlx::query_as::<_, Position>(
        "SELECT * FROM positions WHERE status = 'ACTIVE' ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(positions) => (StatusCode::OK, Json(ApiResponse::success(positions))),
        Err(e) => {
            tracing::error!("Failed to fetch positions: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch positions: {}", e),
                )),
            )
        }
    }
}

pub async fn get_position(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<Position>>) {
    let result = sqlx::query_as::<_, Position>(
        "SELECT * FROM positions WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(position)) => (StatusCode::OK, Json(ApiResponse::success(position))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(
                "POSITION_NOT_FOUND".to_string(),
                format!("Position {} not found", id),
            )),
        ),
        Err(e) => {
            tracing::error!("Failed to fetch position: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch position: {}", e),
                )),
            )
        }
    }
}

pub async fn close_position(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let result = sqlx::query(
        "UPDATE positions SET status = 'CLOSED', updated_at = NOW() WHERE id = $1"
    )
    .bind(&id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            let message = serde_json::json!({"message": "Position closed"});
            (StatusCode::OK, Json(ApiResponse::success(message)))
        }
        Err(e) => {
            tracing::error!("Failed to close position: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to close position: {}", e),
                )),
            )
        }
    }
}
