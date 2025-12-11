use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{post, delete},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::routes::AuthState;
use flowmason_auth::{JwtService, ApiKeyService, AuthContext};
use axum::extract::Extension;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub key: String,
    pub name: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ApiKeyListResponse {
    pub keys: Vec<ApiKeyInfo>,
}

#[derive(Serialize)]
pub struct ApiKeyInfo {
    pub id: String,
    pub name: Option<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

pub fn routes() -> Router<AuthState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .nest("/api-keys", Router::new()
            .route("/", post(create_api_key).get(list_api_keys))
            .route("/:id", delete(delete_api_key))
            .layer(axum::middleware::from_fn(flowmason_auth::auth_middleware)))
}

fn hash_password(password: &str) -> Result<String, StatusCode> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn register(
    axum::extract::State(state): axum::extract::State<AuthState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Check if user already exists
    if state.user_repo.get_by_email(&payload.email).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let password_hash = hash_password(&payload.password)?;
    let user = flowmason_auth::User::new(payload.email.clone(), password_hash);
    
    state.user_repo.create(&user).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let jwt_service = JwtService::from_env();
    let token = jwt_service.generate_token(&user.id, &user.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    }))
}

async fn login(
    axum::extract::State(state): axum::extract::State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = state.user_repo.get_by_email(&payload.email).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_service = JwtService::from_env();
    let token = jwt_service.generate_token(&user.id, &user.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    }))
}

async fn create_api_key(
    axum::extract::State(state): axum::extract::State<AuthState>,
    Extension(auth_context): Extension<AuthContext>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>, StatusCode> {
    let user_id = auth_context.user_id;
    
    let api_key = ApiKeyService::generate();
    let key_hash = ApiKeyService::hash(&api_key);
    
    let id = state.api_key_repo.create(
        &user_id,
        &key_hash,
        payload.name.as_deref()
    ).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiKeyResponse {
        id,
        key: api_key,
        name: payload.name,
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}

async fn list_api_keys(
    axum::extract::State(state): axum::extract::State<AuthState>,
    Extension(auth_context): Extension<AuthContext>,
) -> Result<Json<ApiKeyListResponse>, StatusCode> {
    let user_id = auth_context.user_id;
    
    let keys = state.api_key_repo.list_by_user(&user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiKeyListResponse {
        keys: keys.into_iter().map(|k| ApiKeyInfo {
            id: k.id,
            name: k.name,
            created_at: k.created_at.to_rfc3339(),
            last_used_at: k.last_used_at.map(|d| d.to_rfc3339()),
        }).collect(),
    }))
}

async fn delete_api_key(
    axum::extract::State(state): axum::extract::State<AuthState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Verify ownership before deletion
    let api_key = state.api_key_repo.get(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(key) = api_key {
        if key.user_id != auth_context.user_id {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
    
    state.api_key_repo.delete(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct CreateApiKeyRequest {
    pub name: Option<String>,
}

