use anyhow::Result;
use sqlx::SqlitePool;
use flowmason_core::types::FlowExecution;
use serde_json::Value;

/// Parses JSON string with error logging on failure
fn parse_json_with_logging(json_str: &str, field_name: &str) -> Value {
    match serde_json::from_str(json_str) {
        Ok(value) => value,
        Err(e) => {
            tracing::warn!(field = %field_name, error = %e, "Failed to parse JSON, using Null fallback");
            Value::Null
        }
    }
}

#[derive(Clone)]
pub struct ExecutionRepository {
    pool: SqlitePool,
}

impl ExecutionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, execution: &FlowExecution) -> Result<()> {
        let status_json = serde_json::to_string(&execution.status)?;
        let started_at_str = execution.started_at.to_rfc3339();
        let completed_at_str = execution.completed_at.as_ref().map(|dt| dt.to_rfc3339());
        let input_payload_json = serde_json::to_string(&execution.input_payload)?;
        let output_payload_json = execution.output_payload.as_ref()
            .map(|v| serde_json::to_string(v))
            .transpose()
            .map_err(|e| anyhow::anyhow!("Failed to serialize output_payload: {}", e))?;
        
        let output_payload_str = output_payload_json.as_deref();
        
        sqlx::query!(
            r#"
            INSERT INTO executions (execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            execution.execution_id,
            execution.flow_id,
            status_json,
            started_at_str,
            completed_at_str,
            input_payload_json,
            output_payload_str,
            execution.error
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get(&self, execution_id: &str) -> Result<Option<FlowExecution>> {
        let row = sqlx::query!(
            r#"
            SELECT execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error
            FROM executions
            WHERE execution_id = ?1
            "#,
            execution_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(FlowExecution {
                flow_id: row.flow_id,
                execution_id: row.execution_id.expect("execution_id should not be null"),
                status: serde_json::from_str(&row.status)?,
                started_at: chrono::DateTime::parse_from_rfc3339(&row.started_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse started_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                completed_at: row.completed_at.as_ref().map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .map_err(|e| anyhow::anyhow!("Failed to parse completed_at: {}", e))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }).transpose()?,
                input_payload: serde_json::from_str(&row.input_payload)?,
                output_payload: row.output_payload.as_ref().map(|s| parse_json_with_logging(s, "output_payload")),
                error: row.error,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_by_flow(&self, flow_id: &str, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<FlowExecution>> {
        let limit_val = limit.unwrap_or(100).min(1000) as i64; // Max 1000 items
        let offset_val = offset.unwrap_or(0) as i64;
        
        let rows = sqlx::query!(
            r#"
            SELECT execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error
            FROM executions
            WHERE flow_id = ?1
            ORDER BY started_at DESC
            LIMIT ?2 OFFSET ?3
            "#,
            flow_id,
            limit_val,
            offset_val
        )
        .fetch_all(&self.pool)
        .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(FlowExecution {
                flow_id: row.flow_id,
                execution_id: row.execution_id.expect("execution_id should not be null"),
                status: serde_json::from_str(&row.status)?,
                started_at: chrono::DateTime::parse_from_rfc3339(&row.started_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse started_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                completed_at: row.completed_at.as_ref().map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .map_err(|e| anyhow::anyhow!("Failed to parse completed_at: {}", e))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }).transpose()?,
                input_payload: serde_json::from_str(&row.input_payload)?,
                output_payload: row.output_payload.as_ref().map(|s| parse_json_with_logging(s, "output_payload")),
                error: row.error,
            });
        }

        Ok(executions)
    }

    pub async fn list_all(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<FlowExecution>> {
        let limit_val = limit.unwrap_or(100).min(1000) as i64; // Max 1000 items
        let offset_val = offset.unwrap_or(0) as i64;
        
        let rows = sqlx::query!(
            r#"
            SELECT execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error
            FROM executions
            ORDER BY started_at DESC
            LIMIT ?1 OFFSET ?2
            "#,
            limit_val,
            offset_val
        )
        .fetch_all(&self.pool)
        .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(FlowExecution {
                flow_id: row.flow_id,
                execution_id: row.execution_id.expect("execution_id should not be null"),
                status: serde_json::from_str(&row.status)?,
                started_at: chrono::DateTime::parse_from_rfc3339(&row.started_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse started_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                completed_at: row.completed_at.as_ref().map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .map_err(|e| anyhow::anyhow!("Failed to parse completed_at: {}", e))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }).transpose()?,
                input_payload: serde_json::from_str(&row.input_payload)?,
                output_payload: row.output_payload.as_ref().map(|s| parse_json_with_logging(s, "output_payload")),
                error: row.error,
            });
        }

        Ok(executions)
    }
}

