use axum::Router;
use tower_http::cors::CorsLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer, key_extractor::SmartIpKeyExtractor};
use tower::timeout::TimeoutLayer;
use std::{net::SocketAddr, time::Duration};

use crate::routes::create_router;
use flowmason_db::connection::create_pool;

pub async fn start_server() -> anyhow::Result<()> {
    // Database pool
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://flowmason.db".to_string());
    let pool = create_pool(&database_url).await?;

    // Configure rate limiting: 100 requests per second per IP
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(100)
            .burst_size(200)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let governor_layer = GovernorLayer::new(governor_conf);
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(30));

    let app = Router::new()
        .merge(create_router(pool))
        .layer(CorsLayer::permissive())
        .layer(timeout_layer)
        .layer(governor_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 FlowMason API server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

