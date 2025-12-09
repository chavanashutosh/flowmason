pub mod flows;
pub mod bricks;
pub mod executions;
pub mod usage;
pub mod scheduler;
pub mod auth;

use axum::{Router, middleware};
use std::sync::Arc;
use flowmason_core::quota::{QuotaManager, DatabaseQuotaManager};
use flowmason_core::UsageLogger;
use flowmason_meter::DatabaseUsageLogger;
use flowmason_scheduler::CronExecutor;
use flowmason_db::repositories::{FlowRepository, ExecutionRepository, UsageLogRepository, UserRepository, ApiKeyRepository};
use flowmason_auth::auth_middleware;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct FlowState {
    pub flow_repo: FlowRepository,
}

#[derive(Clone)]
pub struct ExecutionState {
    pub flow_repo: FlowRepository,
    pub execution_repo: ExecutionRepository,
    pub usage_repo: UsageLogRepository,
    pub quota_manager: Arc<dyn QuotaManager>,
    pub usage_logger: Arc<dyn UsageLogger>,
}

#[derive(Clone)]
pub struct SchedulerState {
    pub flow_repo: FlowRepository,
    pub execution_repo: ExecutionRepository,
    pub quota_manager: Arc<dyn QuotaManager>,
    pub usage_logger: Arc<dyn UsageLogger>,
    pub cron_executor: Arc<CronExecutor>,
}

#[derive(Clone)]
pub struct AuthState {
    pub user_repo: UserRepository,
    pub api_key_repo: ApiKeyRepository,
}

pub fn create_router(pool: SqlitePool) -> Router {
    let flow_repo = FlowRepository::new(pool.clone());
    let execution_repo = ExecutionRepository::new(pool.clone());
    let usage_repo = UsageLogRepository::new(pool.clone());
    let user_repo = UserRepository::new(pool.clone());
    let api_key_repo = ApiKeyRepository::new(pool.clone());
    let quota_manager: Arc<dyn QuotaManager> = Arc::new(DatabaseQuotaManager::new(pool.clone()));
    let usage_logger: Arc<dyn UsageLogger> = Arc::new(DatabaseUsageLogger::new(usage_repo.clone()));
    let cron_executor = Arc::new(
        CronExecutor::new().expect("Failed to create CronExecutor")
    );
    
    // Start the scheduler
    let executor_clone = cron_executor.clone();
    tokio::spawn(async move {
        if let Err(e) = executor_clone.start().await {
            eprintln!("Failed to start scheduler: {}", e);
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
        cron_executor,
    };

    let auth_state = AuthState {
        user_repo: user_repo.clone(),
        api_key_repo: api_key_repo.clone(),
    };
    
    Router::new()
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

