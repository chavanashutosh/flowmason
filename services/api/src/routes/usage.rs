use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use flowmason_core::types::BrickType;
use std::sync::Arc;

use crate::dto::{UsageLogResponse, UsageStatsResponse};
use crate::routes::ExecutionState;

pub fn routes() -> Router<ExecutionState> {
    Router::new()
        .route("/", get(list_usage_logs))
        .route("/stats", get(get_usage_stats))
        .route("/stats/:brick_type", get(get_brick_stats))
}

async fn list_usage_logs(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
) -> Result<Json<Vec<UsageLogResponse>>, StatusCode> {
    let logs = state.usage_logger.get_all_logs()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(logs.into_iter().map(UsageLogResponse::from).collect()))
}

async fn get_usage_stats(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
) -> Result<Json<Vec<UsageStatsResponse>>, StatusCode> {
    // Get stats for all brick types from quota manager
    let brick_types = vec![BrickType::OpenAi, BrickType::Nvidia];
    let mut stats = Vec::new();
    
    for brick_type in brick_types {
        let quota = state.quota_manager.get_quota(&brick_type)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let daily_usage = state.usage_logger.get_daily_usage_count(&brick_type)
            .await
            .unwrap_or(0);
        
        stats.push(UsageStatsResponse {
            brick_type: brick_type.clone(),
            daily_usage,
            daily_limit: quota.daily_limit,
            monthly_usage: quota.current_monthly_usage,
            monthly_limit: quota.monthly_limit,
        });
    }
    
    Ok(Json(stats))
}

async fn get_brick_stats(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(brick_type_str): Path<String>,
) -> Result<Json<UsageStatsResponse>, StatusCode> {
    let brick_type = match brick_type_str.as_str() {
        "openai" => BrickType::OpenAi,
        "nvidia" => BrickType::Nvidia,
        "hubspot" => BrickType::HubSpot,
        "notion" => BrickType::Notion,
        "odoo" => BrickType::Odoo,
        "n8n" => BrickType::N8n,
        _ => return Err(StatusCode::NOT_FOUND),
    };
    
    let quota = state.quota_manager.get_quota(&brick_type)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let daily_usage = state.usage_logger.get_daily_usage_count(&brick_type)
        .await
        .unwrap_or(0);
    
    Ok(Json(UsageStatsResponse {
        brick_type,
        daily_usage,
        daily_limit: quota.daily_limit,
        monthly_usage: quota.current_monthly_usage,
        monthly_limit: quota.monthly_limit,
    }))
}

