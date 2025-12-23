use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{Token, ApiResponse},
    state::AppState,
};

pub async fn list_tokens(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<Token>>>) {
    let result = sqlx::query_as::<_, Token>(
        "SELECT * FROM tokens ORDER BY created_at DESC LIMIT 100"
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(tokens) => (StatusCode::OK, Json(ApiResponse::success(tokens))),
        Err(e) => {
            tracing::error!("Failed to fetch tokens: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch tokens: {}", e),
                )),
            )
        }
    }
}

pub async fn get_token(
    State(state): State<AppState>,
    Path(mint): Path<String>,
) -> (StatusCode, Json<ApiResponse<Token>>) {
    let result = sqlx::query_as::<_, Token>(
        "SELECT * FROM tokens WHERE mint = $1"
    )
    .bind(&mint)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(token)) => (StatusCode::OK, Json(ApiResponse::success(token))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(
                "TOKEN_NOT_FOUND".to_string(),
                format!("Token {} not found", mint),
            )),
        ),
        Err(e) => {
            tracing::error!("Failed to fetch token: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to fetch token: {}", e),
                )),
            )
        }
    }
}
