mod context;
mod agent;
mod orchestrator;
mod db;
mod model;

use axum::{routing::post, Router, Json, WebSocket, extract::ws::{Message, WebSocketUpgrade}};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use agent::{SupervisorAgent, PlannerAgent, ResearchAgent};
use orchestrator::Orchestrator;
use db::init_db;

#[derive(Deserialize)]
struct Query {
    user_id: String,
    message: String,
}

#[tokio::main]
async fn main() {
    init_db();

    let agents = vec![
        Box::new(SupervisorAgent::new()) as Box<dyn agent::Agent + Send + Sync>,
        Box::new(PlannerAgent::new()),
        Box::new(ResearchAgent::new()),
    ];

    let orchestrator = Arc::new(Mutex::new(Orchestrator::new(agents)));

    let app = Router::new()
        .route("/query", post(handle_query))
        .route("/healthz", axum::routing::get(health_check))
        .route("/ws", axum::routing::get(ws_handler))
        .with_state(orchestrator.clone());

    println!("MCP Rust API server running at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_query(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<Orchestrator>>>,
    Json(payload): Json<Query>,
) -> Json<serde_json::Value> {
    let mut orchestrator = state.lock().await;
    let result = orchestrator.handle_request(payload.message).await;
    Json(serde_json::json!({ "response": result }))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<Arc<Mutex<Orchestrator>>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        while let Some(Ok(Message::Text(msg))) = socket.recv().await {
            let mut orchestrator = state.lock().await;
            let result = orchestrator.handle_request(msg).await;
            let _ = socket.send(Message::Text(result)).await;
        }
    })
}