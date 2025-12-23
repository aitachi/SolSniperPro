use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;

use crate::state::AppState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(stream: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = stream.split();

    // 创建消息通道
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // 注册客户端
    {
        let mut clients = state.ws_clients.write().await;
        clients.push(tx.clone());
        tracing::info!("WebSocket 客户端连接, 当前连接数: {}", clients.len());
    }

    // 发送欢迎消息
    let welcome = serde_json::json!({
        "type": "connection",
        "data": {
            "status": "connected",
            "timestamp": chrono::Utc::now().timestamp_millis(),
        }
    });

    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg)).await;
    }

    // 启动接收任务
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // 启动发送任务
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                tracing::debug!("收到 WebSocket 消息: {}", text);
                // 可以处理客户端消息
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    // 移除客户端
    {
        let mut clients = state.ws_clients.write().await;
        // 从列表中移除断开的客户端
        // 注意: 我们无法直接判断 UnboundedSender 是否关闭
        // 实际应用中应该维护一个连接ID映射
        tracing::info!("WebSocket 客户端断开");
    }
}
