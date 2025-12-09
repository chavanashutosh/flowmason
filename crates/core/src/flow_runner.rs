use serde_json::Value;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use crate::brick_traits::{Brick, BrickError};
use crate::quota::{QuotaError, QuotaManager};
use crate::types::{Flow, FlowExecution, ExecutionStatus, BrickType};
use async_trait::async_trait;

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
    
    #[error("Execution interrupted: {0}")]
    Interrupted(String),
}

/// Context for flow execution with optional quota and usage tracking
pub struct FlowRunnerContext {
    pub quota_manager: Option<Arc<dyn QuotaManager>>,
    pub usage_logger: Option<Arc<dyn UsageLogger>>,
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

        for (index, brick) in bricks.iter().enumerate() {
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

            // Execute brick with current payload
            let result = brick.execute(current_payload.clone(), config.clone())
                .await
                .map_err(FlowError::BrickError)?;

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
            }

            // Update payload for next brick
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

        match Self::execute_flow(
            bricks,
            flow.bricks.iter().map(|b| b.config.clone()).collect(),
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

