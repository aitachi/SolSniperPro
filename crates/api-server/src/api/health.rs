use axum::{http::StatusCode, Json};

use crate::models::ApiResponse;

pub async fn health_check() -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    use std::time::SystemTime;

    let health = serde_json::json!({
        "status": "healthy",
        "timestamp": SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
        "version": "2.0.0",
    });

    (StatusCode::OK, Json(ApiResponse::success(health)))
}
