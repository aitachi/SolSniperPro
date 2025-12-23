mod api;
mod models;
mod state;
mod config;

use anyhow::Result;
use axum::{
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::{auth, tokens, strategies, positions, trades, metrics, risk, health, ws},
    config::AppConfig,
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 SolSniper Pro API Server v2.0.0 启动中...");

    // 加载配置
    let config = AppConfig::load()?;
    info!("✅ 配置加载成功");

    // 初始化应用状态
    let state = AppState::new(config.clone()).await?;
    info!("✅ 应用状态初始化成功");

    // 构建路由
    let app = Router::new()
        // Health check
        .route("/health", get(health::health_check))

        // Auth routes
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/logout", post(auth::logout))

        // Token routes
        .route("/api/v1/tokens", get(tokens::list_tokens))
        .route("/api/v1/tokens/:mint", get(tokens::get_token))

        // Strategy routes
        .route("/api/v1/strategies", get(strategies::list_strategies))
        .route("/api/v1/strategies/:id", get(strategies::get_strategy))
        .route("/api/v1/strategies/:id/start", post(strategies::start_strategy))
        .route("/api/v1/strategies/:id/pause", post(strategies::pause_strategy))
        .route("/api/v1/strategies", post(strategies::create_strategy))

        // Position routes
        .route("/api/v1/positions", get(positions::list_positions))
        .route("/api/v1/positions/:id", get(positions::get_position))
        .route("/api/v1/positions/:id/close", post(positions::close_position))

        // Trade routes
        .route("/api/v1/trades", get(trades::list_trades))
        .route("/api/v1/trades/:id", get(trades::get_trade))

        // Metrics routes
        .route("/api/v1/metrics/summary", get(metrics::get_summary))
        .route("/api/v1/metrics/system", get(metrics::get_system_metrics))
        .route("/api/v1/metrics/strategy/:id", get(metrics::get_strategy_metrics))

        // Risk control routes
        .route("/api/v1/risk/limits", get(risk::get_limits))
        .route("/api/v1/risk/limits", put(risk::update_limits))
        .route("/api/v1/risk/status", get(risk::get_risk_status))

        // WebSocket
        .route("/ws", get(ws::websocket_handler))

        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("🌐 API Server 监听在 http://{}", addr);
    info!("📊 WebSocket 端点: ws://{}/ws", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .await
        .map_err(|e| {
            error!("服务器错误: {}", e);
            anyhow::anyhow!("服务器启动失败: {}", e)
        })?;

    Ok(())
}
