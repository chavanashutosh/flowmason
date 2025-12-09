use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use crate::dto::{ExecuteFlowRequest, FlowExecutionResponse};
use crate::routes::ExecutionState;
use flowmason_core::{FlowRunner, FlowRunnerContext};
use flowmason_bricks::*;

pub fn routes() -> Router<ExecutionState> {
    Router::new()
        .route("/", post(execute_flow).get(list_executions))
        .route("/:execution_id", get(get_execution))
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
        };
        bricks.push(brick);
    }
    
    // Create execution context with quota manager and usage logger
    let context = FlowRunnerContext {
        quota_manager: Some(state.quota_manager.clone()),
        usage_logger: Some(state.usage_logger.clone()),
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
        eprintln!("Flow execution error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // Store execution in history
    state.execution_repo.create(&execution).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(FlowExecutionResponse::from(execution)))
}

async fn list_executions(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
) -> Result<Json<Vec<FlowExecutionResponse>>, StatusCode> {
    let exec_list = state.execution_repo.list_all().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(exec_list.into_iter().map(FlowExecutionResponse::from).collect()))
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
) -> Result<Json<Vec<FlowExecutionResponse>>, StatusCode> {
    let exec_list = state.execution_repo.list_by_flow(&flow_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(exec_list.into_iter().map(FlowExecutionResponse::from).collect()))
}

