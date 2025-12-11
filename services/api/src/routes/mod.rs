pub mod flows;
pub mod bricks;
pub mod executions;
pub mod usage;
pub mod scheduler;
pub mod auth;
pub mod web;

use axum::{Router, middleware};
use std::sync::Arc;
use flowmason_core::quota::{QuotaManager, DatabaseQuotaManager};
use flowmason_core::UsageLogger;
use flowmason_meter::DatabaseUsageLogger;
use flowmason_scheduler::CronExecutor;
use flowmason_db::repositories::{FlowRepository, ExecutionRepository, UsageLogRepository, UserRepository, ApiKeyRepository, ScheduledFlowRepository};
use flowmason_auth::auth_middleware;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct FlowState {
    pub flow_repo: Arc<FlowRepository>,
}

#[derive(Clone)]
pub struct ExecutionState {
    pub flow_repo: Arc<FlowRepository>,
    pub execution_repo: Arc<ExecutionRepository>,
    pub usage_repo: Arc<UsageLogRepository>,
    pub quota_manager: Arc<dyn QuotaManager>,
    pub usage_logger: Arc<dyn UsageLogger>,
}

#[derive(Clone)]
pub struct SchedulerState {
    pub flow_repo: Arc<FlowRepository>,
    pub execution_repo: Arc<ExecutionRepository>,
    pub quota_manager: Arc<dyn QuotaManager>,
    pub usage_logger: Arc<dyn UsageLogger>,
    pub cron_executor: Arc<CronExecutor>,
    #[allow(dead_code)]
    pub scheduled_flow_repo: Arc<ScheduledFlowRepository>,
}

#[derive(Clone)]
pub struct AuthState {
    pub user_repo: Arc<UserRepository>,
    pub api_key_repo: Arc<ApiKeyRepository>,
}

pub async fn create_router(pool: SqlitePool) -> Router {
    let flow_repo = FlowRepository::new(pool.clone());
    let execution_repo = ExecutionRepository::new(pool.clone());
    let usage_repo = UsageLogRepository::new(pool.clone());
    let user_repo = UserRepository::new(pool.clone());
    let api_key_repo = ApiKeyRepository::new(pool.clone());
    let scheduled_flow_repo = ScheduledFlowRepository::new(pool.clone());
    let quota_manager: Arc<dyn QuotaManager> = Arc::new(DatabaseQuotaManager::new(pool.clone()));
    let usage_logger: Arc<dyn UsageLogger> = Arc::new(DatabaseUsageLogger::new(usage_repo.clone()));
    
    // Wrap repositories in Arc for sharing
    let flow_repo = Arc::new(flow_repo);
    let execution_repo = Arc::new(execution_repo);
    let usage_repo = Arc::new(usage_repo);
    let user_repo = Arc::new(user_repo);
    let api_key_repo = Arc::new(api_key_repo);
    let scheduled_flow_repo = Arc::new(scheduled_flow_repo);
    
    // Create cron executor with repositories asynchronously
    let cron_executor = Arc::new(
        CronExecutor::with_repositories(
            scheduled_flow_repo.clone(),
            flow_repo.clone(),
        ).await.expect("Failed to create CronExecutor")
    );
    
    // Start the scheduler and load scheduled flows
    let executor_clone = cron_executor.clone();
    let execution_repo_clone = execution_repo.clone();
    let quota_manager_clone = quota_manager.clone();
    let usage_logger_clone = usage_logger.clone();
    
    tokio::spawn(async move {
        // Start the scheduler
        if let Err(e) = executor_clone.start().await {
            eprintln!("Failed to start scheduler: {}", e);
            return;
        }
        
        // Load scheduled flows from database
        if let Err(e) = executor_clone.load_scheduled_flows(|_flow| {
            let execution_repo = execution_repo_clone.clone();
            let quota_manager = quota_manager_clone.clone();
            let usage_logger = usage_logger_clone.clone();
            
            Arc::new(move |flow: flowmason_core::types::Flow, initial_payload: serde_json::Value| {
                let execution_repo = execution_repo.clone();
                let quota_manager = quota_manager.clone();
                let usage_logger = usage_logger.clone();
                
                Box::pin(async move {
                    use flowmason_bricks::*;
                    use flowmason_core::{FlowRunner, FlowRunnerContext};
                    
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
            })
        }).await {
            eprintln!("Failed to load scheduled flows: {}", e);
        }
    });
    
    let execution_state = ExecutionState {
        flow_repo: flow_repo.clone(),
        execution_repo: execution_repo.clone(),
        usage_repo: usage_repo.clone(),
        quota_manager: quota_manager.clone(),
        usage_logger: usage_logger.clone(),
    };
    
    let scheduler_state = SchedulerState {
        flow_repo: flow_repo.clone(),
        execution_repo: execution_repo.clone(),
        quota_manager,
        usage_logger,
        cron_executor: cron_executor.clone(),
        scheduled_flow_repo: scheduled_flow_repo.clone(),
    };

    let auth_state = AuthState {
        user_repo: user_repo.clone(),
        api_key_repo: api_key_repo.clone(),
    };
    
    Router::new()
        .merge(web::routes().with_state(FlowState { flow_repo: flow_repo.clone() }))
        .nest("/api/v1", Router::new()
            .nest("/auth", auth::routes().with_state(auth_state))
            .nest("/bricks", bricks::routes())
            .nest("/flows", flows::routes()
                .layer(middleware::from_fn(auth_middleware))
                .with_state(FlowState { flow_repo: flow_repo.clone() }))
            .nest("/executions", executions::routes()
                .layer(middleware::from_fn(auth_middleware))
                .with_state(execution_state.clone()))
            .nest("/usage", usage::routes()
                .layer(middleware::from_fn(auth_middleware))
                .with_state(execution_state))
            .nest("/scheduler", scheduler::routes()
                .layer(middleware::from_fn(auth_middleware))
                .with_state(scheduler_state))
        )
}

