use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use uuid::Uuid;

use crate::dto::{CreateFlowRequest, FlowResponse, UpdateFlowRequest, PaginationParams, PaginatedResponse};
use crate::routes::FlowState;
use crate::validation::validate_webhook_url;
use flowmason_core::types::{BrickConfig, Flow, BrickType};
use serde_json::{Value, json};

pub fn routes() -> Router<FlowState> {
    Router::new()
        .route("/", post(create_flow).get(list_flows))
        .route("/:id", get(get_flow).put(update_flow).delete(delete_flow))
        .route("/:id/duplicate", post(duplicate_flow))
        .route("/:id/export", get(export_flow))
        .route("/import", post(import_flow))
}

async fn create_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Json(payload): Json<CreateFlowRequest>,
) -> Result<Json<FlowResponse>, StatusCode> {
    // Validate webhook URLs in brick configs
    for brick in &payload.bricks {
        if brick.brick_type == BrickType::N8n {
            if let Some(webhook_url) = brick.config.get("webhook_url").and_then(|v| v.as_str()) {
                if let Err(e) = validate_webhook_url(webhook_url, None) {
                    tracing::warn!(webhook_url = %webhook_url, error = %e, "Invalid webhook URL in flow creation");
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let flow = Flow {
        id,
        name: payload.name,
        description: payload.description,
        bricks: payload.bricks.into_iter().map(|b| BrickConfig {
            brick_type: b.brick_type,
            config: b.config,
        }).collect(),
        active: true,
        created_at: now,
        updated_at: now,
    };
    
    state.flow_repo.create(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(FlowResponse::from(flow)))
}

async fn list_flows(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<FlowResponse>>, StatusCode> {
    let flows = state.flow_repo.list(Some(params.limit), Some(params.offset))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let items = flows.into_iter().map(FlowResponse::from).collect();
    Ok(Json(PaginatedResponse::new(items, params.limit, params.offset)))
}

async fn get_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Path(id): Path<String>,
) -> Result<Json<FlowResponse>, StatusCode> {
    let flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(FlowResponse::from(flow)))
}

async fn update_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateFlowRequest>,
) -> Result<Json<FlowResponse>, StatusCode> {
    let mut flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = payload.name {
        flow.name = name;
    }
    if let Some(description) = payload.description {
        flow.description = Some(description);
    }
    if let Some(bricks) = payload.bricks {
        // Validate webhook URLs in brick configs
        for brick in &bricks {
            if brick.brick_type == BrickType::N8n {
                if let Some(webhook_url) = brick.config.get("webhook_url").and_then(|v| v.as_str()) {
                    if let Err(e) = validate_webhook_url(webhook_url, None) {
                        tracing::warn!(webhook_url = %webhook_url, error = %e, "Invalid webhook URL in flow update");
                        return Err(StatusCode::BAD_REQUEST);
                    }
                }
            }
        }
        flow.bricks = bricks.into_iter().map(|b| BrickConfig {
            brick_type: b.brick_type,
            config: b.config,
        }).collect();
    }
    if let Some(active) = payload.active {
        flow.active = active;
    }
    flow.updated_at = chrono::Utc::now();

    state.flow_repo.update(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(FlowResponse::from(flow)))
}

async fn delete_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let existing = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    state.flow_repo.delete(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn duplicate_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Path(id): Path<String>,
) -> Result<Json<FlowResponse>, StatusCode> {
    let original_flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let new_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let duplicated_flow = Flow {
        id: new_id,
        name: format!("{} (Copy)", original_flow.name),
        description: original_flow.description.clone(), // Clone needed for Option<String>
        bricks: original_flow.bricks.clone(), // Clone needed for Vec<BrickConfig>
        active: false, // Duplicated flows start as inactive
        created_at: now,
        updated_at: now,
    };
    
    state.flow_repo.create(&duplicated_flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(FlowResponse::from(duplicated_flow)))
}

async fn export_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let export_data = json!({
        "version": "1.0",
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "flow": {
            "name": flow.name,
            "description": flow.description,
            "bricks": flow.bricks,
            "active": flow.active,
        }
    });
    
    Ok(Json(export_data))
}

#[derive(serde::Deserialize)]
struct ImportFlowRequest {
    flow: serde_json::Value,
}

async fn import_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Json(payload): Json<ImportFlowRequest>,
) -> Result<Json<FlowResponse>, StatusCode> {
    let flow_data = payload.flow;
    
    let name = flow_data.get("name")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let description = flow_data.get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let bricks_json = flow_data.get("bricks")
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let bricks: Vec<BrickConfig> = serde_json::from_value(bricks_json.clone())
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Validate webhook URLs in imported bricks
    for brick in &bricks {
        if brick.brick_type == BrickType::N8n {
            if let Some(webhook_url) = brick.config.get("webhook_url").and_then(|v| v.as_str()) {
                if let Err(_) = validate_webhook_url(webhook_url, None) {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }
    
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let flow = Flow {
        id,
        name: name.to_string(),
        description,
        bricks,
        active: false, // Imported flows start as inactive
        created_at: now,
        updated_at: now,
    };
    
    state.flow_repo.create(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(FlowResponse::from(flow)))
}

