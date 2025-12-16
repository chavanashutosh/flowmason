pub mod flows;
pub mod bricks;
pub mod executions;
pub mod usage;
pub mod scheduler;
pub mod auth;
pub mod web;
pub mod templates;
pub mod webhooks;

use axum::{Router, middleware, extract::Request, middleware::Next, response::Response, http::StatusCode, Json};
use tower_http::services::ServeDir;
use std::sync::Arc;
use serde_json::json;
use flowmason_core::quota::{QuotaManager, DatabaseQuotaManager};
use flowmason_core::UsageLogger;
use flowmason_meter::DatabaseUsageLogger;
use flowmason_scheduler::CronExecutor;
use flowmason_db::repositories::{FlowRepository, ExecutionRepository, UsageLogRepository, UserRepository, ApiKeyRepository, ScheduledFlowRepository, ExecutionDataRepository, TemplateRepository};
use flowmason_auth::{auth_middleware, AuthStateForMiddleware, AuthContext, ApiKeyService};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct FlowState {
    pub flow_repo: Arc<FlowRepository>,
    pub template_repo: Arc<TemplateRepository>,
}

#[derive(Clone)]
pub struct ExecutionState {
    pub flow_repo: Arc<FlowRepository>,
    pub execution_repo: Arc<ExecutionRepository>,
    pub usage_repo: Arc<UsageLogRepository>,
    pub execution_data_repo: Arc<ExecutionDataRepository>,
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
    pub scheduled_flow_repo: Arc<ScheduledFlowRepository>,
}

#[derive(Clone)]
pub struct AuthState {
    pub user_repo: Arc<UserRepository>,
    pub api_key_repo: Arc<ApiKeyRepository>,
}

#[derive(Clone)]
pub struct TemplateState {
    pub template_repo: Arc<TemplateRepository>,
    pub flow_repo: Arc<FlowRepository>,
}

pub async fn create_router(pool: SqlitePool) -> Router {
    // Create repositories directly wrapped in Arc to avoid intermediate clones
    let flow_repo = Arc::new(FlowRepository::new(pool.clone()));
    let execution_repo = Arc::new(ExecutionRepository::new(pool.clone()));
    let usage_repo = Arc::new(UsageLogRepository::new(pool.clone()));
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let api_key_repo = Arc::new(ApiKeyRepository::new(pool.clone()));
    let scheduled_flow_repo = Arc::new(ScheduledFlowRepository::new(pool.clone()));
    let execution_data_repo = Arc::new(ExecutionDataRepository::new(pool.clone()));
    let quota_manager: Arc<dyn QuotaManager> = Arc::new(DatabaseQuotaManager::new(pool.clone()));
    let usage_logger: Arc<dyn UsageLogger> = Arc::new(DatabaseUsageLogger::new(usage_repo.clone()));
    
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
            tracing::error!(error = %e, "Failed to start scheduler");
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
                    use flowmason_bricks::RulesEngineBrick;
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
                            flowmason_core::types::BrickType::RulesEngine => Box::new(RulesEngineBrick),
                        };
                        bricks.push(brick);
                    }
                    
                    // Create execution context
                    let context = FlowRunnerContext {
                        quota_manager: Some(quota_manager),
                        usage_logger: Some(usage_logger),
                        execution_data_storage: None, // Scheduler doesn't store execution data
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
            tracing::error!(error = %e, "Failed to load scheduled flows");
        }
    });
    
    let execution_state = ExecutionState {
        flow_repo: flow_repo.clone(),
        execution_repo: execution_repo.clone(),
        usage_repo: usage_repo.clone(),
        execution_data_repo: execution_data_repo.clone(),
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
    
    // Create API key validator closure that has access to repositories
    let user_repo_for_middleware = user_repo.clone();
    let api_key_repo_for_middleware = api_key_repo.clone();
    let validate_api_key: flowmason_auth::ApiKeyValidator = Arc::new(move |api_key: String| {
        let user_repo = user_repo_for_middleware.clone();
        let api_key_repo = api_key_repo_for_middleware.clone();
        Box::pin(async move {
            // Hash the API key and look it up
            let key_hash = ApiKeyService::hash(&api_key);
            let api_key_data = api_key_repo.get_by_hash(&key_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            
            // Update last_used_at
            let _ = api_key_repo.update_last_used(&key_hash).await;
            
            // Get user information
            let user = user_repo.get_by_id(&api_key_data.user_id).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            
            Ok(AuthContext {
                user_id: user.id,
                email: user.email,
            })
        })
    });
    
    let auth_state_for_middleware = AuthStateForMiddleware {
        validate_api_key: validate_api_key.clone(),
    };
    
    // Create a middleware wrapper that injects AuthStateForMiddleware and calls auth_middleware
    let template_repo = Arc::new(TemplateRepository::new(pool.clone()));
    
    // Seed predefined templates
    let template_repo_for_seed = template_repo.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::templates::predefined::seed_predefined_templates(&template_repo_for_seed).await {
            tracing::warn!(error = %e, "Failed to seed predefined templates");
        }
    });
    
    let auth_state_clone_1 = auth_state_for_middleware.clone();
    let auth_state_clone_2 = auth_state_for_middleware.clone();
    let auth_state_clone_3 = auth_state_for_middleware.clone();
    let auth_state_clone_4 = auth_state_for_middleware.clone();
    let auth_state_clone_5 = auth_state_for_middleware.clone();
    
    // Also need to inject auth state for /auth/me route
    let auth_state_for_auth_routes = auth_state_for_middleware.clone();
    
    let mut app = Router::new()
        .route("/health", axum::routing::get(|| async { axum::Json(json!({"status": "ok"})) }))
        .merge(web::routes().with_state(FlowState { 
            flow_repo: flow_repo.clone(),
            template_repo: template_repo.clone(),
        }))
        .nest("/api/v1", Router::new()
            .nest("/auth", auth::routes()
                .layer(middleware::from_fn(move |mut request: Request, next: Next| {
                    let state = auth_state_for_auth_routes.clone();
                    async move {
                        request.extensions_mut().insert(state);
                        next.run(request).await
                    }
                }))
                .with_state(auth_state))
            .nest("/bricks", bricks::routes())
            .nest("/flows", flows::routes()
                .layer(middleware::from_fn(move |mut request: Request, next: Next| {
                    let state = auth_state_clone_1.clone();
                    async move {
                        request.extensions_mut().insert(state);
                        auth_middleware(request, next).await
                    }
                }))
                .with_state(FlowState { flow_repo: flow_repo.clone() }))
            .nest("/executions", executions::routes()
                .layer(middleware::from_fn(move |mut request: Request, next: Next| {
                    let state = auth_state_clone_2.clone();
                    async move {
                        request.extensions_mut().insert(state);
                        auth_middleware(request, next).await
                    }
                }))
                .with_state(execution_state.clone()))
            .nest("/usage", usage::routes()
                .layer(middleware::from_fn(move |mut request: Request, next: Next| {
                    let state = auth_state_clone_3.clone();
                    async move {
                        request.extensions_mut().insert(state);
                        auth_middleware(request, next).await
                    }
                }))
                .with_state(execution_state))
            .nest("/scheduler", scheduler::routes()
                .layer(middleware::from_fn(move |mut request: Request, next: Next| {
                    let state = auth_state_clone_4.clone();
                    async move {
                        request.extensions_mut().insert(state);
                        auth_middleware(request, next).await
                    }
                }))
                .with_state(scheduler_state))
            .nest("/templates", templates::routes()
                .layer(middleware::from_fn(move |mut request: Request, next: Next| {
                    let state = auth_state_clone_5.clone();
                    async move {
                        request.extensions_mut().insert(state);
                        auth_middleware(request, next).await
                    }
                }))
                .with_state(TemplateState {
                    template_repo: template_repo.clone(),
                    flow_repo: flow_repo.clone(),
                }))
            .nest("/webhooks", webhooks::routes()
                .with_state(execution_state.clone()))
        );

    // Serve static files from React/Vite build directory if it exists
    let static_dir = std::path::Path::new("services/web-ui-vite/dist");
    if static_dir.exists() {
        app = app.fallback_service(ServeDir::new(static_dir));
    }

    app
}
