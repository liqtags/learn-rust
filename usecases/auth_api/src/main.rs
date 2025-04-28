use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32, // user id
    exp: usize, // expiration time
}

// User model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
}

// Request/Response types
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
    user: User,
}

// Application state
struct AppState {
    db: PgPool,
    jwt_secret: String,
}

// Custom error type
#[derive(Debug)]
enum AppError {
    DatabaseError(sqlx::Error),
    AuthError(String),
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
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
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Create database connection pool
    let pool = PgPool::connect(&database_url).await?;

    // Create application state
    let state = Arc::new(AppState {
        db: pool,
        jwt_secret,
    });

    // Build our application with routes
    let app = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(get_current_user))
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
async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Hash the password
    let password_hash = bcrypt::hash(payload.password, 10)
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Insert the user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::DatabaseError)?;

    // Generate JWT token
    let token = generate_token(&state.jwt_secret, user.id)?;

    Ok(Json(AuthResponse { token, user }))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find the user
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE username = $1
        "#,
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::AuthError("Invalid credentials".to_string()))?;

    // Verify password
    if !bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|e| AppError::ValidationError(e.to_string()))?
    {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let token = generate_token(&state.jwt_secret, user.id)?;

    Ok(Json(AuthResponse { token, user }))
}

async fn get_current_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<User>, AppError> {
    // Extract token from Authorization header
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AppError::AuthError("Missing token".to_string()))?;

    // Verify token
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::AuthError("Invalid token".to_string()))?
    .claims;

    // Get user from database
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(Json(user))
}

// Helper functions
fn generate_token(secret: &str, user_id: i32) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::ValidationError(e.to_string()))
} 