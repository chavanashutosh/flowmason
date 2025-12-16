use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::Serialize;
use crate::dto::{ExecuteFlowRequest, FlowExecutionResponse, PaginationParams, PaginatedResponse};
use crate::routes::ExecutionState;
use flowmason_core::{FlowRunner, FlowRunnerContext};
use flowmason_bricks::*;
use flowmason_bricks::RulesEngineBrick;
use std::sync::Arc;

pub fn routes() -> Router<ExecutionState> {
    Router::new()
        .route("/", post(execute_flow).get(list_executions))
        .route("/:execution_id", get(get_execution))
        .route("/:execution_id/data", get(get_execution_data).delete(delete_execution_data))
        .route("/:execution_id/data/brick/:brick_index", get(get_brick_data_by_path))
        .route("/:execution_id/data/fetched", get(get_fetched_data))
        .route("/:execution_id/data/intermediate", get(get_intermediate_data))
        .route("/flow/:flow_id", get(list_flow_executions))
}

async fn execute_flow(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Json(payload): Json<ExecuteFlowRequest>,
) -> Result<Json<FlowExecutionResponse>, StatusCode> {
    // Get flow from store
    let flow = state.flow_repo.get(&payload.flow_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Create brick instances based on flow configuration
    // This is simplified - in production, you'd have a registry
    let mut bricks: Vec<Box<dyn flowmason_core::Brick>> = Vec::new();
    
    for brick_config in &flow.bricks {
        let brick: Box<dyn flowmason_core::Brick> = match brick_config.brick_type {
            flowmason_core::types::BrickType::OpenAi => Box::new(OpenAiBrick),
            flowmason_core::types::BrickType::Nvidia => Box::new(NvidiaBrick),
            flowmason_core::types::BrickType::HubSpot => Box::new(HubSpotBrick),
            flowmason_core::types::BrickType::Notion => Box::new(NotionBrick),
            flowmason_core::types::BrickType::Odoo => Box::new(OdooBrick),
            flowmason_core::types::BrickType::N8n => Box::new(N8nBrick),
            flowmason_core::types::BrickType::FieldMapping => Box::new(FieldMappingBrick),
            flowmason_core::types::BrickType::CombineText => Box::new(CombineTextBrick),
            flowmason_core::types::BrickType::Conditional => Box::new(ConditionalBrick),
            flowmason_core::types::BrickType::RulesEngine => Box::new(RulesEngineBrick),
        };
        bricks.push(brick);
    }
    
    // Create execution context with quota manager and usage logger
    // Wrap ExecutionDataRepository in Arc<dyn ExecutionDataStorage>
    let execution_data_storage: Arc<dyn flowmason_core::ExecutionDataStorage> = state.execution_data_repo.clone();
    let context = FlowRunnerContext {
        quota_manager: Some(state.quota_manager.clone()),
        usage_logger: Some(state.usage_logger.clone()),
        execution_data_storage: Some(execution_data_storage),
        flow_id: flow.id.clone(),
        execution_id: String::new(), // Will be set in execute_flow_with_tracking
    };
    
    // Execute flow
    let execution = FlowRunner::execute_flow_with_tracking(
        &flow,
        bricks,
        payload.input_payload,
        Some(context),
    )
    .await
    .map_err(|e| {
        tracing::error!(error = %e, flow_id = %payload.flow_id, "Flow execution error");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // Store execution in history
    state.execution_repo.create(&execution).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(FlowExecutionResponse::from(execution)))
}

async fn list_executions(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<FlowExecutionResponse>>, StatusCode> {
    let exec_list = state.execution_repo.list_all(Some(params.limit), Some(params.offset))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let items = exec_list.into_iter().map(FlowExecutionResponse::from).collect();
    Ok(Json(PaginatedResponse::new(items, params.limit, params.offset)))
}

async fn get_execution(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(execution_id): Path<String>,
) -> Result<Json<FlowExecutionResponse>, StatusCode> {
    let execution = state.execution_repo.get(&execution_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(FlowExecutionResponse::from(execution)))
}

async fn list_flow_executions(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(flow_id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<FlowExecutionResponse>>, StatusCode> {
    let exec_list = state.execution_repo.list_by_flow(&flow_id, Some(params.limit), Some(params.offset))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let items = exec_list.into_iter().map(FlowExecutionResponse::from).collect();
    Ok(Json(PaginatedResponse::new(items, params.limit, params.offset)))
}

#[derive(Serialize)]
struct ExecutionDataResponse {
    id: String,
    execution_id: String,
    brick_index: i32,
    brick_type: String,
    data_type: String,
    data_key: String,
    data_value: serde_json::Value,
    timestamp: String,
}

async fn get_execution_data(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<ExecutionDataResponse>>, StatusCode> {
    let data = state.execution_data_repo.get_by_execution(&execution_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(data.into_iter().map(|d| ExecutionDataResponse {
        id: d.id,
        execution_id: d.execution_id,
        brick_index: d.brick_index,
        brick_type: d.brick_type,
        data_type: d.data_type,
        data_key: d.data_key,
        data_value: d.data_value,
        timestamp: d.timestamp.to_rfc3339(),
    }).collect()))
}

#[derive(serde::Deserialize)]
struct BrickDataPath {
    execution_id: String,
    brick_index: i32,
}

async fn get_brick_data_by_path(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(params): Path<BrickDataPath>,
) -> Result<Json<Vec<ExecutionDataResponse>>, StatusCode> {
    let data = state.execution_data_repo.get_by_brick(&params.execution_id, params.brick_index)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(data.into_iter().map(|d| ExecutionDataResponse {
        id: d.id,
        execution_id: d.execution_id,
        brick_index: d.brick_index,
        brick_type: d.brick_type,
        data_type: d.data_type,
        data_key: d.data_key,
        data_value: d.data_value,
        timestamp: d.timestamp.to_rfc3339(),
    }).collect()))
}

async fn get_fetched_data(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<ExecutionDataResponse>>, StatusCode> {
    let data = state.execution_data_repo.get_by_data_type(&execution_id, "fetched")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(data.into_iter().map(|d| ExecutionDataResponse {
        id: d.id,
        execution_id: d.execution_id,
        brick_index: d.brick_index,
        brick_type: d.brick_type,
        data_type: d.data_type,
        data_key: d.data_key,
        data_value: d.data_value,
        timestamp: d.timestamp.to_rfc3339(),
    }).collect()))
}

async fn get_intermediate_data(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(execution_id): Path<String>,
) -> Result<Json<Vec<ExecutionDataResponse>>, StatusCode> {
    let data = state.execution_data_repo.get_by_data_type(&execution_id, "intermediate")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(data.into_iter().map(|d| ExecutionDataResponse {
        id: d.id,
        execution_id: d.execution_id,
        brick_index: d.brick_index,
        brick_type: d.brick_type,
        data_type: d.data_type,
        data_key: d.data_key,
        data_value: d.data_value,
        timestamp: d.timestamp.to_rfc3339(),
    }).collect()))
}

async fn delete_execution_data(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(execution_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    state.execution_data_repo.delete_by_execution(&execution_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

