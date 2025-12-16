use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
    body::Bytes,
};
use serde_json::json;
use crate::routes::ExecutionState;
use flowmason_core::types::BrickType;

pub fn routes() -> Router<ExecutionState> {
    Router::new()
        .route("/flows/:flow_id/trigger", post(trigger_flow_webhook))
}

#[derive(serde::Deserialize)]
struct WebhookTriggerRequest {
    payload: Option<serde_json::Value>,
}

async fn trigger_flow_webhook(
    State(state): State<ExecutionState>,
    Path(flow_id): Path<String>,
    body: Bytes,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Parse payload from body or use empty object
    let input_payload = if body.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&body)
            .unwrap_or_else(|_| json!({}))
    };

    // Get flow
    let flow = state.flow_repo.get(&flow_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if !flow.active {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create brick instances
    use flowmason_bricks::*;
    use flowmason_bricks::RulesEngineBrick;
    use flowmason_core::{FlowRunner, FlowRunnerContext};
    use std::sync::Arc;

    let mut bricks: Vec<Box<dyn flowmason_core::Brick>> = Vec::new();
    
    for brick_config in &flow.bricks {
        let brick: Box<dyn flowmason_core::Brick> = match brick_config.brick_type {
            BrickType::OpenAi => Box::new(OpenAiBrick),
            BrickType::Nvidia => Box::new(NvidiaBrick),
            BrickType::HubSpot => Box::new(HubSpotBrick),
            BrickType::Notion => Box::new(NotionBrick),
            BrickType::Odoo => Box::new(OdooBrick),
            BrickType::N8n => Box::new(N8nBrick),
            BrickType::FieldMapping => Box::new(FieldMappingBrick),
            BrickType::CombineText => Box::new(CombineTextBrick),
            BrickType::Conditional => Box::new(ConditionalBrick),
            BrickType::RulesEngine => Box::new(RulesEngineBrick),
        };
        bricks.push(brick);
    }

    let execution_data_storage: Arc<dyn flowmason_core::ExecutionDataStorage> = state.execution_data_repo.clone();
    let context = FlowRunnerContext {
        quota_manager: Some(state.quota_manager.clone()),
        usage_logger: Some(state.usage_logger.clone()),
        execution_data_storage: Some(execution_data_storage),
        flow_id: flow.id.clone(),
        execution_id: String::new(),
    };

    // Execute flow
    let execution = FlowRunner::execute_flow_with_tracking(
        &flow,
        bricks,
        input_payload,
        Some(context),
    )
    .await
    .map_err(|e| {
        tracing::error!(error = %e, flow_id = %flow_id, "Webhook flow execution error");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Store execution
    state.execution_repo.create(&execution).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": true,
        "execution_id": execution.execution_id,
        "status": format!("{:?}", execution.status),
        "output": execution.output_payload,
    })))
}
