use serde_json::Value;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
use tokio::sync::Semaphore;
use std::sync::OnceLock;

use crate::brick_traits::{Brick, BrickError};
use crate::quota::{QuotaError, QuotaManager};
use crate::types::{Flow, FlowExecution, ExecutionStatus, BrickType, UsageLog};
use async_trait::async_trait;

/// Maximum number of concurrent background tasks for execution data storage
const MAX_CONCURRENT_STORAGE_TASKS: usize = 100;

/// Semaphore to limit concurrent background storage tasks
static STORAGE_SEMAPHORE: OnceLock<Arc<Semaphore>> = OnceLock::new();

fn get_storage_semaphore() -> Arc<Semaphore> {
    STORAGE_SEMAPHORE.get_or_init(|| {
        Arc::new(Semaphore::new(MAX_CONCURRENT_STORAGE_TASKS))
    }).clone()
}

#[derive(Debug, Error)]
pub enum FlowError {
    #[error("Brick execution error: {0}")]
    BrickError(#[from] BrickError),
    
    #[error("Quota error: {0}")]
    QuotaError(#[from] QuotaError),
    
    #[error("Flow not found: {0}")]
    FlowNotFound(String),
    
    #[error("Invalid flow configuration: {0}")]
    InvalidFlow(String),
}

/// Context for flow execution with optional quota and usage tracking
pub struct FlowRunnerContext {
    pub quota_manager: Option<Arc<dyn QuotaManager>>,
    pub usage_logger: Option<Arc<dyn UsageLogger>>,
    pub execution_data_storage: Option<Arc<dyn ExecutionDataStorage>>,
    pub flow_id: String,
    pub execution_id: String,
}

/// Trait for usage logging (to avoid circular dependencies)
#[async_trait]
pub trait UsageLogger: Send + Sync {
    async fn record_usage(
        &self,
        brick_name: &str,
        brick_type: &BrickType,
        flow_id: &str,
        execution_id: &str,
        cost_unit: f64,
        token_usage: Option<u64>,
        metadata: Option<Value>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_all_logs(
        &self,
    ) -> Result<Vec<UsageLog>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_daily_usage_count(
        &self,
        brick_type: &BrickType,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for execution data storage (to avoid circular dependencies)
#[async_trait]
pub trait ExecutionDataStorage: Send + Sync {
    async fn store_data(
        &self,
        execution_id: &str,
        brick_index: usize,
        brick_type: &BrickType,
        data_type: &str,
        data_key: &str,
        data_value: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub struct FlowRunner;

impl FlowRunner {
    /// Executes a flow with the given bricks and initial payload
    pub async fn execute_flow(
        bricks: Vec<Box<dyn Brick>>,
        configs: Vec<Value>,
        initial_payload: Value,
        context: Option<FlowRunnerContext>,
    ) -> Result<Value, FlowError> {
        if bricks.len() != configs.len() {
            return Err(FlowError::InvalidFlow(
                "Number of bricks must match number of configs".to_string(),
            ));
        }

        let mut current_payload = initial_payload;
        let mut skip_count = 0;

        for (index, brick) in bricks.iter().enumerate() {
            // Skip bricks if skip_count > 0
            if skip_count > 0 {
                skip_count -= 1;
                continue;
            }

            let config = &configs[index];
            let brick_type = brick.brick_type();
            
            // Validate config before execution
            brick.validate_config(config)
                .map_err(FlowError::BrickError)?;

            // Check quota before execution
            if let Some(ref ctx) = context {
                if let Some(ref quota_manager) = ctx.quota_manager {
                    quota_manager.check_quota(&brick_type).await?;
                }
            }

            // Execute brick with current payload (move ownership to avoid clone)
            // Note: config.clone() is necessary because execute() takes ownership
            let mut result = brick.execute(current_payload, config.clone())
                .await
                .map_err(FlowError::BrickError)?;

            // Check for branching metadata in result
            if let Some(obj) = result.as_object_mut() {
                // Handle SkipBricks action
                if let Some(skip_value) = obj.remove("_skip_bricks") {
                    if let Some(skip_num) = skip_value.as_u64() {
                        skip_count = skip_num as usize;
                    }
                }
                // Note: Branch action is handled at API level, not in FlowRunner
                // as it requires access to flow repository
            }

            // Extract cost and token usage from result
            let (cost_unit, token_usage) = Self::extract_execution_metadata(&result, &brick_type);

            // Record usage after execution
            if let Some(ref ctx) = context {
                if let Some(ref usage_logger) = ctx.usage_logger {
                    let _ = usage_logger.record_usage(
                        brick.name(),
                        &brick_type,
                        &ctx.flow_id,
                        &ctx.execution_id,
                        cost_unit,
                        token_usage,
                        None,
                    ).await;
                }
                
                // Record usage in quota manager
                if let Some(ref quota_manager) = ctx.quota_manager {
                    let _ = quota_manager.record_usage(&brick_type, cost_unit, token_usage).await;
                }

                // Store execution data asynchronously (non-blocking)
                // Clone result once for storage operations to avoid multiple clones
                if let Some(ref data_storage) = ctx.execution_data_storage {
                    let execution_id = ctx.execution_id.clone();
                    let brick_type_clone = brick_type.clone();
                    // Clone result once before spawning task
                    let result_for_storage = result.clone();
                    let brick_index = index;
                    let is_api_fetch = Self::is_api_fetch_brick(&brick_type);
                    let data_storage_clone = data_storage.clone();
                    let semaphore = get_storage_semaphore();
                    
                    // Spawn background task to store data without blocking flow execution
                    // Use semaphore to limit concurrent tasks and prevent resource exhaustion
                    tokio::spawn(async move {
                        // Acquire permit from semaphore (waits if limit reached)
                        let _permit = match semaphore.acquire().await {
                            Ok(p) => p,
                            Err(_) => {
                                // Semaphore closed, skip storage
                                return;
                            }
                        };
                        
                        // Store intermediate output
                        let _ = data_storage_clone.store_data(
                            &execution_id,
                            brick_index,
                            &brick_type_clone,
                            "intermediate",
                            &format!("brick_{}", brick_index),
                            result_for_storage.clone(),
                        ).await;

                        // If brick fetches data from external API, store as "fetched" with same data
                        // Reuse the already-cloned result_for_storage instead of cloning again
                        if is_api_fetch {
                            let _ = data_storage_clone.store_data(
                                &execution_id,
                                brick_index,
                                &brick_type_clone,
                                "fetched",
                                &format!("fetched_{}", brick_index),
                                result_for_storage, // Move ownership instead of cloning again
                            ).await;
                        }
                        // Permit is automatically released when dropped
                    });
                }
            }

            // Update payload for next brick (move ownership)
            current_payload = result;
        }

        Ok(current_payload)
    }

    /// Executes a flow with detailed execution tracking
    pub async fn execute_flow_with_tracking(
        flow: &Flow,
        bricks: Vec<Box<dyn Brick>>,
        initial_payload: Value,
        context: Option<FlowRunnerContext>,
    ) -> Result<FlowExecution, FlowError> {
        let execution_id = Uuid::new_v4().to_string();
        let started_at = chrono::Utc::now();

        // Create context with execution tracking info
        let mut exec_context = context.unwrap_or_else(|| FlowRunnerContext {
            quota_manager: None,
            usage_logger: None,
            execution_data_storage: None,
            flow_id: flow.id.clone(),
            execution_id: execution_id.clone(),
        });
        exec_context.flow_id = flow.id.clone();
        exec_context.execution_id = execution_id.clone();

        let mut execution = FlowExecution {
            flow_id: flow.id.clone(),
            execution_id: execution_id.clone(),
            status: ExecutionStatus::Running,
            started_at,
            completed_at: None,
            input_payload: initial_payload.clone(),
            output_payload: None,
            error: None,
        };

        // Collect configs once to avoid repeated cloning
        let configs: Vec<Value> = flow.bricks.iter().map(|b| b.config.clone()).collect();
        match Self::execute_flow(
            bricks,
            configs,
            initial_payload,
            Some(exec_context),
        ).await {
            Ok(output) => {
                execution.status = ExecutionStatus::Completed;
                execution.completed_at = Some(chrono::Utc::now());
                execution.output_payload = Some(output);
                Ok(execution)
            }
            Err(e) => {
                execution.status = ExecutionStatus::Failed;
                execution.completed_at = Some(chrono::Utc::now());
                execution.error = Some(e.to_string());
                Err(e)
            }
        }
    }

    /// Extracts cost and token usage metadata from brick execution result
    fn extract_execution_metadata(result: &Value, brick_type: &BrickType) -> (f64, Option<u64>) {
        match brick_type {
            BrickType::OpenAi => {
                // Extract token usage from OpenAI response
                let token_usage = result
                    .get("token_usage")
                    .and_then(|v| v.as_u64());
                
                // Calculate cost (simplified: ~$0.002 per 1K tokens for gpt-3.5-turbo)
                let cost = if let Some(tokens) = token_usage {
                    (tokens as f64 / 1000.0) * 0.002
                } else {
                    0.0
                };
                
                (cost, token_usage)
            }
            BrickType::Nvidia => {
                // Extract from NVIDIA response if available
                let token_usage = result
                    .get("token_usage")
                    .and_then(|v| v.as_u64());
                let cost = 0.0; // Default cost for NVIDIA
                (cost, token_usage)
            }
            _ => {
                // Default for other brick types
                (0.0, None)
            }
        }
    }

    /// Determines if a brick type fetches data from external APIs
    fn is_api_fetch_brick(brick_type: &BrickType) -> bool {
        matches!(
            brick_type,
            BrickType::HubSpot | BrickType::Notion | BrickType::Odoo | BrickType::N8n
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brick_traits::Brick;
    use crate::types::BrickType;
    use async_trait::async_trait;
    use serde_json::json;

    struct MockBrick {
        name: &'static str,
        output: Value,
    }

    #[async_trait]
    impl Brick for MockBrick {
        fn name(&self) -> &'static str {
            self.name
        }

        fn brick_type(&self) -> BrickType {
            BrickType::FieldMapping
        }

        fn config_schema(&self) -> Value {
            json!({})
        }

        async fn execute(&self, _input: Value, _config: Value) -> Result<Value, BrickError> {
            Ok(self.output.clone())
        }
    }

    #[tokio::test]
    async fn test_execute_flow() {
        let brick1 = Box::new(MockBrick {
            name: "brick1",
            output: json!({"step": 1}),
        });
        let brick2 = Box::new(MockBrick {
            name: "brick2",
            output: json!({"step": 2}),
        });

        let bricks: Vec<Box<dyn Brick>> = vec![brick1, brick2];
        let configs = vec![json!({}), json!({})];
        let initial = json!({"start": true});

        let result = FlowRunner::execute_flow(bricks, configs, initial, None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!({"step": 2}));
    }
}

