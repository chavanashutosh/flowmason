use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

/// Middleware that adds a request ID to each request for tracing
pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Add request ID to request extensions for use in handlers
    request.extensions_mut().insert(request_id.clone());

    // Add request ID to response headers
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap_or_default(),
    );

    response
}

/// Extract request ID from request extensions
pub fn get_request_id(request: &Request) -> Option<String> {
    request.extensions().get::<String>().cloned()
}
