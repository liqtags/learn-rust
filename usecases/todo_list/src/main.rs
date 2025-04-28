use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Define our Todo model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
    user_id: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

// Define our request/response types
#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
    user_id: i32,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    completed: Option<bool>,
}

// Define our application state
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = PgPool::connect(&database_url).await?;

    // Create tables if they don't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT false,
            user_id INTEGER NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create application state
    let state = Arc::new(AppState { db: pool });

    // Build our application with a route
    let app = Router::new()
        .route("/todos", get(list_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(state);

    // Run it
    let addr = "127.0.0.1:3000";
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Handler functions
async fn list_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todos))
}

async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, user_id)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(payload.user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todo))
}

async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(todo))
}

async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET 
            title = COALESCE($1, title),
            completed = COALESCE($2, completed)
        WHERE id = $3
        RETURNING *
        "#,
    )
    .bind(payload.title)
    .bind(payload.completed)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(todo))
}

async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Todo not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
} 