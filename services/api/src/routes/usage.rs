use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use flowmason_core::types::BrickType;
use std::collections::HashSet;

use crate::dto::{UsageLogResponse, UsageStatsResponse};
use crate::routes::ExecutionState;

/// Converts BrickType to its database name format (matches brick.name())
fn brick_type_to_db_name(brick_type: &BrickType) -> String {
    match brick_type {
        BrickType::OpenAi => "openai",
        BrickType::Nvidia => "nvidia",
        BrickType::HubSpot => "hubspot",
        BrickType::Notion => "notion",
        BrickType::Odoo => "odoo",
        BrickType::N8n => "n8n",
        BrickType::FieldMapping => "field_mapping",
        BrickType::CombineText => "combine_text",
        BrickType::Conditional => "conditional",
        BrickType::RulesEngine => "rules_engine",
    }.to_string()
}

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
    // Get stats for all predefined brick types from quota manager
    let brick_types = vec![
        BrickType::OpenAi,
        BrickType::Nvidia,
        BrickType::HubSpot,
        BrickType::Notion,
        BrickType::Odoo,
        BrickType::N8n,
        BrickType::FieldMapping,
        BrickType::CombineText,
        BrickType::Conditional,
        BrickType::RulesEngine,
    ];
    
    // Map predefined brick types to their string representations (matching database format)
    let predefined_brick_names: HashSet<String> = brick_types.iter()
        .map(|bt| brick_type_to_db_name(bt))
        .collect();
    
    let mut stats = Vec::new();
    
    // Get stats for predefined brick types
    for brick_type in &brick_types {
        let quota = state.quota_manager.get_quota(brick_type)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let daily_usage = state.usage_logger.get_daily_usage_count(brick_type)
            .await
            .unwrap_or(0);
        
        stats.push(UsageStatsResponse::from_brick_type(
            brick_type.clone(),
            daily_usage,
            quota.daily_limit,
            quota.current_monthly_usage,
            quota.monthly_limit,
        ));
    }
    
    // Get custom brick names from database
    let all_brick_names = state.usage_repo.get_unique_brick_names()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Filter out predefined brick types to get only custom bricks
    let custom_brick_names: Vec<String> = all_brick_names
        .into_iter()
        .filter(|name| !predefined_brick_names.contains(name))
        .collect();
    
    // Get stats for custom bricks
    for brick_name in custom_brick_names {
        let daily_usage = state.usage_repo.get_daily_usage_count_by_name(&brick_name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Try to get quota for custom brick (may not exist)
        // Use default values if quota doesn't exist
        let (daily_limit, monthly_limit, monthly_usage) = {
            // Try to parse brick_name as BrickType to check quota
            // If it fails or quota doesn't exist, use defaults
            let default_daily_limit = 1000u64;
            let default_monthly_limit = Some(10000u64);
            
            // For custom bricks, we'll use default quotas
            // In the future, custom bricks could have their own quota entries
            (default_daily_limit, default_monthly_limit, Some(0u64))
        };
        
        stats.push(UsageStatsResponse::from_custom_brick(
            brick_name,
            daily_usage,
            daily_limit,
            monthly_usage,
            monthly_limit,
        ));
    }
    
    Ok(Json(stats))
}

async fn get_brick_stats(
    axum::extract::State(state): axum::extract::State<ExecutionState>,
    Path(brick_type_str): Path<String>,
) -> Result<Json<UsageStatsResponse>, StatusCode> {
    // Try to match as predefined brick type first
    match brick_type_str.as_str() {
        "openai" | "nvidia" | "hubspot" | "notion" | "odoo" | "n8n" | 
        "field_mapping" | "combine_text" | "conditional" => {
            let brick_type = match brick_type_str.as_str() {
                "openai" => BrickType::OpenAi,
                "nvidia" => BrickType::Nvidia,
                "hubspot" => BrickType::HubSpot,
                "notion" => BrickType::Notion,
                "odoo" => BrickType::Odoo,
                "n8n" => BrickType::N8n,
                "field_mapping" => BrickType::FieldMapping,
                "combine_text" => BrickType::CombineText,
                "conditional" => BrickType::Conditional,
                "rules_engine" => BrickType::RulesEngine,
                _ => unreachable!(),
            };
            
            let quota = state.quota_manager.get_quota(&brick_type)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?;
            
            let daily_usage = state.usage_logger.get_daily_usage_count(&brick_type)
                .await
                .unwrap_or(0);
            
            Ok(Json(UsageStatsResponse::from_brick_type(
                brick_type,
                daily_usage,
                quota.daily_limit,
                quota.current_monthly_usage,
                quota.monthly_limit,
            )))
        }
        // Custom brick - check if it exists in usage logs
        _ => {
            let daily_usage = state.usage_repo.get_daily_usage_count_by_name(&brick_type_str)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?;
            
            // Use default quotas for custom bricks
            let default_daily_limit = 1000u64;
            let default_monthly_limit = Some(10000u64);
            let default_monthly_usage = Some(0u64);
            
            Ok(Json(UsageStatsResponse::from_custom_brick(
                brick_type_str,
                daily_usage,
                default_daily_limit,
                default_monthly_usage,
                default_monthly_limit,
            )))
        }
    }
}

