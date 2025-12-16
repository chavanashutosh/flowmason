use axum::{http::StatusCode, error_handling::HandleErrorLayer};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer, key_extractor::SmartIpKeyExtractor};
use tower::{timeout::TimeoutLayer, ServiceBuilder, BoxError};
use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use flowmason_api::routes::create_router;
use flowmason_api::middleware::request_id_middleware;
use flowmason_db::connection::create_pool;

pub async fn start_server() -> anyhow::Result<()> {
    // Initialize tracing subscriber with environment variable control
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Initializing FlowMason API server");

    // Database pool
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://flowmason.db".to_string());
    let pool = create_pool(&database_url).await?;
    tracing::info!(database_url = %database_url, "Database connection established");

    // Configure rate limiting: 100 requests per second per IP
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(100)
        .burst_size(200)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    // tower-governor 0.2 expects a reference with 'static lifetime
    let governor_conf: &'static _ = Box::leak(Box::new(governor_conf));
    let governor_layer = GovernorLayer { config: governor_conf };

    let middleware_stack = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        request_id = %request_id
                    )
                })
                .on_request(|_request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::debug!("request started");
                })
                .on_response(|_response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!(latency_ms = latency.as_millis(), "request completed");
                })
        )
        .layer(axum::middleware::from_fn(request_id_middleware))
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            tracing::error!(error = %err, "Request error");
            if err.is::<tower::timeout::error::Elapsed>() {
                StatusCode::REQUEST_TIMEOUT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(governor_layer)
        .layer(CorsLayer::permissive());

    let app = create_router(pool).await
        .layer(middleware_stack);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!(address = %addr, "🚀 FlowMason API server starting");

    // Configure HTTP server with increased header size limits to prevent HTTP 431 errors
    // Note: hyper 0.14 (used by Axum 0.7) doesn't expose max_header_size in its public API.
    // The header size limit is hardcoded at ~8KB. However, we use axum::serve which
    // handles the connection setup internally. While we can't directly configure header
    // size limits, increasing buffer sizes may help with some edge cases.
    //
    // For production environments experiencing HTTP 431 errors:
    // 1. Clear browser cookies (most common cause of large headers)
    // 2. Reduce authentication token sizes
    // 3. Use a reverse proxy (nginx/caddy) configured to handle larger headers
    // 4. Consider upgrading to Axum 0.8+ which uses hyper 1.x with configurable limits
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

