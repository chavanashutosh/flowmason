use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::jwt::JwtService;
use crate::api_key::ApiKeyService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub email: String,
}

/// Callback function type for API key validation
/// This allows the routes module to provide repository access without circular dependencies
pub type ApiKeyValidator = Arc<dyn Fn(String) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AuthContext, StatusCode>> + Send>> + Send + Sync>;

/// AuthState that can be stored in request extensions for API key validation
#[derive(Clone)]
pub struct AuthStateForMiddleware {
    pub validate_api_key: ApiKeyValidator,
}

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check for Authorization header
    let auth_header = headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_context = if auth_header.starts_with("Bearer ") {
        let token = auth_header.strip_prefix("Bearer ").unwrap();
        let jwt_service = JwtService::from_env();
        
        // Try JWT verification first
        match jwt_service.verify_token(token) {
            Ok(claims) => {
                AuthContext {
                    user_id: claims.sub,
                    email: claims.email,
                }
            }
            Err(_) => {
                // JWT verification failed, try as API key
                let auth_state = request.extensions()
                    .get::<AuthStateForMiddleware>()
                    .ok_or(StatusCode::UNAUTHORIZED)?;
                
                // Validate API key format
                if !ApiKeyService::validate_format(token) {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                
                // Use the validation function from auth_state
                (auth_state.validate_api_key)(token.to_string()).await?
            }
        }
    } else if auth_header.starts_with("ApiKey ") {
        // Support ApiKey prefix for backward compatibility
        let api_key = auth_header.strip_prefix("ApiKey ").unwrap();
        
        let auth_state = request.extensions()
            .get::<AuthStateForMiddleware>()
            .ok_or(StatusCode::UNAUTHORIZED)?;
        
        // Validate API key format
        if !ApiKeyService::validate_format(api_key) {
            return Err(StatusCode::UNAUTHORIZED);
        }
        
        // Use the validation function from auth_state
        (auth_state.validate_api_key)(api_key.to_string()).await?
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Store auth context in request extensions
    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}

/// Extract user_id from request extensions
pub fn extract_user_id(request: &Request) -> Option<String> {
    request.extensions().get::<AuthContext>()
        .map(|ctx| ctx.user_id.clone())
}

// Optional auth middleware - allows requests with or without auth
pub async fn optional_auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    // If auth header is present, validate it
    if let Some(auth_header) = headers.get("authorization")
        .and_then(|h| h.to_str().ok()) {
        
        if auth_header.starts_with("Bearer ") {
            let token = auth_header.strip_prefix("Bearer ").unwrap();
            let jwt_service = JwtService::from_env();
            let _ = jwt_service.verify_token(token);
        } else if auth_header.starts_with("ApiKey ") {
            let api_key = auth_header.strip_prefix("ApiKey ").unwrap();
            let _ = crate::api_key::ApiKeyService::validate_format(api_key);
        }
    }
    
    next.run(request).await
}
