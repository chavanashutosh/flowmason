use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use std::sync::Arc;

use crate::dto::{ScheduleFlowRequest, ScheduleFlowResponse, ScheduledFlowsResponse};
use crate::routes::SchedulerState;
use flowmason_core::{FlowRunner, FlowRunnerContext};
use flowmason_bricks::*;

pub fn routes() -> Router<SchedulerState> {
    Router::new()
        .route("/flows", post(schedule_flow).get(list_scheduled_flows))
        .route("/flows/:flow_id", delete(unschedule_flow))
}

async fn schedule_flow(
    axum::extract::State(state): axum::extract::State<SchedulerState>,
    Json(payload): Json<ScheduleFlowRequest>,
) -> Result<Json<ScheduleFlowResponse>, StatusCode> {
    // Get flow from repository
    let flow = state.flow_repo.get(&payload.flow_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create executor function that will be called by the scheduler
    let flow_repo_clone = state.flow_repo.clone();
    let execution_repo_clone = state.execution_repo.clone();
    let quota_manager_clone = state.quota_manager.clone();
    let usage_logger_clone = state.usage_logger.clone();
    
    let executor = Arc::new(move |flow: flowmason_core::types::Flow, initial_payload: serde_json::Value| {
        let flow_repo = flow_repo_clone.clone();
        let execution_repo = execution_repo_clone.clone();
        let quota_manager = quota_manager_clone.clone();
        let usage_logger = usage_logger_clone.clone();
        
        Box::pin(async move {
            // Create brick instances
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
            
            // Create execution context
            let context = FlowRunnerContext {
                quota_manager: Some(quota_manager),
                usage_logger: Some(usage_logger),
                flow_id: flow.id.clone(),
                execution_id: uuid::Uuid::new_v4().to_string(),
            };
            
            // Execute flow
            let execution = FlowRunner::execute_flow_with_tracking(
                &flow,
                bricks,
                initial_payload,
                Some(context),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Flow execution error: {}", e))?;
            
            // Store execution in history
            execution_repo.create(&execution).await?;
            
            Ok(execution)
        })
    });

    // Schedule the flow
    let job_id = state.cron_executor
        .schedule_flow(flow.clone(), &payload.cron_expression, executor)
        .await
        .map_err(|e| {
            eprintln!("Failed to schedule flow: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ScheduleFlowResponse {
        job_id,
        flow_id: payload.flow_id,
        cron_expression: payload.cron_expression,
        scheduled_at: chrono::Utc::now().to_rfc3339(),
    }))
}

async fn list_scheduled_flows(
    axum::extract::State(state): axum::extract::State<SchedulerState>,
) -> Result<Json<ScheduledFlowsResponse>, StatusCode> {
    // Get scheduled flows with cron expressions from database
    let scheduled_flows_with_cron = state.cron_executor.get_scheduled_flows_with_cron().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let scheduled_flows: Vec<_> = scheduled_flows_with_cron
        .into_iter()
        .map(|(flow_id, cron_expression)| crate::dto::ScheduledFlowResponse {
            flow_id,
            cron_expression,
        })
        .collect();

    Ok(Json(ScheduledFlowsResponse {
        flows: scheduled_flows,
    }))
}

async fn unschedule_flow(
    axum::extract::State(state): axum::extract::State<SchedulerState>,
    Path(flow_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    state.cron_executor
        .unschedule_flow(&flow_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to unschedule flow: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

