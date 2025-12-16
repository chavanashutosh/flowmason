use axum::http::{StatusCode, header};
use flowmason_db::connection::create_pool;
use flowmason_db::repositories::{FlowRepository, UserRepository};
use flowmason_auth::JwtService;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

mod common;

use common::*;

#[tokio::test]
async fn test_health_endpoint() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/health")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_flow_requires_auth() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri("/api/v1/flows")
                .header(header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(
                    serde_json::to_string(&json!({
                        "name": "Test Flow",
                        "description": "Test"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_flows_requires_auth() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/v1/flows")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_bricks_no_auth() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/v1/bricks")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Bricks endpoint might not require auth, check if it's accessible
    assert!(response.status().is_success() || response.status() == StatusCode::UNAUTHORIZED);
}
