use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    models::{LoginRequest, LoginResponse, UserInfo, ApiResponse},
    state::AppState,
};

pub async fn login(
    State(_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    // 简单的认证逻辑 (生产环境应该查询数据库并验证密码哈希)
    if payload.username == "admin" && payload.password == "admin123" {
        let response = LoginResponse {
            token: "mock_jwt_token_12345".to_string(),
            user: UserInfo {
                id: "user1".to_string(),
                username: "admin".to_string(),
                role: "ADMIN".to_string(),
            },
        };
        (StatusCode::OK, Json(ApiResponse::success(response)))
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "INVALID_CREDENTIALS".to_string(),
                "Invalid username or password".to_string(),
            )),
        )
    }
}

pub async fn logout(
    State(_state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    (
        StatusCode::OK,
        Json(ApiResponse::success("Logged out successfully".to_string())),
    )
}
