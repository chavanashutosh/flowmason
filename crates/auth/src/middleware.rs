use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::jwt::JwtService;

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub email: String,
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
        
        let claims = jwt_service.verify_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        
        AuthContext {
            user_id: claims.sub,
            email: claims.email,
        }
    } else if auth_header.starts_with("ApiKey ") {
        // For API keys, we'll need to look up user_id in routes that need it
        // For now, return unauthorized - API key routes should handle this separately
        return Err(StatusCode::UNAUTHORIZED);
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
