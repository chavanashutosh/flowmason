use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::dto::{CreateFlowRequest, FlowResponse, UpdateFlowRequest};
use crate::routes::FlowState;
use flowmason_core::types::{BrickConfig, Flow};

pub fn routes() -> Router<FlowState> {
    Router::new()
        .route("/", post(create_flow).get(list_flows))
        .route("/:id", get(get_flow).put(update_flow).delete(delete_flow))
}

async fn create_flow(
    axum::extract::State(state): axum::extract::State<FlowState>,
    Json(payload): Json<CreateFlowRequest>,
) -> Result<Json<FlowResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let flow = Flow {
        id: id.clone(),
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
) -> Result<Json<Vec<FlowResponse>>, StatusCode> {
    let flows = state.flow_repo.list().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(flows.into_iter().map(FlowResponse::from).collect()))
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

