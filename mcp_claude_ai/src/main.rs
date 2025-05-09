// src/main.rs
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::Root,
    Surreal,
};
use tower_http::{cors::CorsLayer, services::ServeDir};

// Model Context Protocol types
#[derive(Debug, Serialize, Deserialize)]
struct ModelRequest {
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    context: Option<Vec<ContextItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContextItem {
    id: String,
    content: String,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelResponse {
    completion: String,
    model: String,
    usage: TokenUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    query: String,
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ContextDocument {
    id: Option<String>,
    content: String,
    metadata: Option<serde_json::Value>,
}

// Shared application state
#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<surrealdb::engine::remote::ws::Client>>,
    anthropic_api_key: String,
    anthropic_model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get server configuration from environment variables
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    // Get SurrealDB configuration from environment variables
    let db_url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let db_user = std::env::var("SURREALDB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let db_pass = std::env::var("SURREALDB_PASSWORD").unwrap_or_else(|_| "root".to_string());
    let db_ns = std::env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| "test".to_string());
    let db_name = std::env::var("SURREALDB_DATABASE").unwrap_or_else(|_| "test".to_string());
    
    // Connect to SurrealDB
    tracing::info!("Connecting to SurrealDB at {}", db_url);
    let db = Surreal::new::<Ws>(format!("ws://{}", db_url)).await?;
    
    // Sign in to the database
    db.signin(Root {
        username: &db_user,
        password: &db_pass,
    })
    .await?;
    
    // Use the namespace and database
    db.use_ns(&db_ns).use_db(&db_name).await?;
    tracing::info!("Connected to SurrealDB: ns={}, db={}", db_ns, db_name);

    // Get the Anthropic API key from env
    let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set in environment variables");
    
    // Get the Claude model from env
    let anthropic_model = std::env::var("ANTHROPIC_MODEL")
        .unwrap_or_else(|_| "claude-3-7-sonnet-20250219".to_string());
    
    tracing::info!("Using Claude model: {}", anthropic_model);

    // Create the shared state
    let state = AppState {
        db: Arc::new(db),
        anthropic_api_key,
        anthropic_model,
    };

    // Build the router with our routes
    let app = Router::new()
        // API routes
        .route("/api/contexts", get(get_contexts))
        .route("/api/contexts/:id", get(get_context_by_id))
        .route("/api/contexts", post(create_context))
        .route("/api/search", get(search_contexts))
        .route("/api/generate", post(generate_with_claude))
        // Static files
        .nest_service("/", ServeDir::new("static"))
        // CORS
        .layer(CorsLayer::permissive())
        // Shared state
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from((
        server_host.parse::<std::net::IpAddr>().unwrap_or_else(|_| ([127, 0, 0, 1]).into()),
        server_port
    ));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Handler to get all contexts
async fn get_contexts(
    State(state): State<AppState>,
) -> Result<Json<Vec<ContextDocument>>, StatusCode> {
    // Execute the query and get all records from the 'context' table
    let contexts: Vec<ContextDocument> = state.db
        .query("SELECT * FROM context")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(contexts))
}

// Handler to get a context by ID
async fn get_context_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ContextDocument>, StatusCode> {
    // Query for a specific context by ID
    let context: Option<ContextDocument> = state.db
        .query("SELECT * FROM context WHERE id = $id")
        .bind(("id", id))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Return the context or 404 if not found
    match context {
        Some(context) => Ok(Json(context)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Handler to create a new context
async fn create_context(
    State(state): State<AppState>,
    Json(context): Json<ContextDocument>,
) -> Result<Json<ContextDocument>, StatusCode> {
    // Create a new record in the 'context' table
    let created: ContextDocument = state.db
        .create("context")
        .content(context)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(created))
}

// Handler to search contexts
async fn search_contexts(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<ContextDocument>>, StatusCode> {
    let limit = params.limit.unwrap_or(10);
    
    // Search for contexts containing the query string
    let results: Vec<ContextDocument> = state.db
        .query("SELECT * FROM context WHERE content CONTAINS $query LIMIT $limit")
        .bind(("query", params.query))
        .bind(("limit", limit))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .take(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(results))
}

// Handler to generate text with Claude 3.7 Sonnet
async fn generate_with_claude(
    State(state): State<AppState>,
    Json(request): Json<ModelRequest>,
) -> Result<Json<ModelResponse>, StatusCode> {
    // Build the Claude API request
    let anthropic_url = "https://api.anthropic.com/v1/messages";
    
    // Create the system prompt
    let system_prompt = "You are Claude 3.7 Sonnet, an AI assistant that provides helpful, accurate, and harmless responses.";
    
    // Prepare the context items if any
    let context_text = if let Some(context_items) = &request.context {
        let mut context_text = String::new();
        for item in context_items {
            context_text.push_str(&format!("--- BEGIN CONTEXT ITEM: {} ---\n{}\n--- END CONTEXT ITEM ---\n\n", 
                item.id, item.content));
        }
        context_text
    } else {
        String::new()
    };
    
    // Build the full prompt with context
    let user_message = if context_text.is_empty() {
        request.prompt.clone()
    } else {
        format!(
            "I'm providing you with some context information that might help with my question. Please use this information to inform your response.\n\n{}\n\nNow, here's my question: {}",
            context_text,
            request.prompt
        )
    };
    
    // Prepare the API request body
    let request_body = serde_json::json!({
        "model": state.anthropic_model,
        "max_tokens": request.max_tokens.unwrap_or(1024),
        "temperature": request.temperature.unwrap_or(0.7),
        "system": system_prompt,
        "messages": [
            {
                "role": "user",
                "content": user_message
            }
        ]
    });
    
    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "x-api-key", 
        state.anthropic_api_key.parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    );
    headers.insert(
        "anthropic-version", 
        "2023-06-01".parse().unwrap()
    );
    
    // Make the API request
    let client = reqwest::Client::new();
    let response = client.post(anthropic_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Handle the API response
    if !response.status().is_success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    let claude_response: serde_json::Value = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Extract the completion from Claude's response
    let completion = claude_response["content"][0]["text"]
        .as_str()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    
    // Get token usage if available, or use estimates
    let prompt_tokens = claude_response["usage"]["input_tokens"]
        .as_u64()
        .unwrap_or(0) as u32;
    
    let completion_tokens = claude_response["usage"]["output_tokens"]
        .as_u64()
        .unwrap_or(0) as u32;
    
    // Create the model response
    let model_response = ModelResponse {
        completion,
        model: state.anthropic_model.clone(),
        usage: TokenUsage {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        },
    };
    
    Ok(Json(model_response))
}
