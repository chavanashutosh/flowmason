use anyhow::Result;
use sqlx::SqlitePool;
use flowmason_core::types::FlowExecution;
use serde_json::Value;

#[derive(Clone)]
pub struct ExecutionRepository {
    pool: SqlitePool,
}

impl ExecutionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, execution: &FlowExecution) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO executions (execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            execution.execution_id,
            execution.flow_id,
            serde_json::to_string(&execution.status)?,
            execution.started_at,
            execution.completed_at,
            serde_json::to_string(&execution.input_payload)?,
            execution.output_payload.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default()),
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
                execution_id: row.execution_id,
                status: serde_json::from_str(&row.status)?,
                started_at: row.started_at,
                completed_at: row.completed_at,
                input_payload: serde_json::from_str(&row.input_payload)?,
                output_payload: row.output_payload.map(|s| serde_json::from_str(&s).unwrap_or(Value::Null)),
                error: row.error,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_by_flow(&self, flow_id: &str) -> Result<Vec<FlowExecution>> {
        let rows = sqlx::query!(
            r#"
            SELECT execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error
            FROM executions
            WHERE flow_id = ?1
            ORDER BY started_at DESC
            "#,
            flow_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(FlowExecution {
                flow_id: row.flow_id,
                execution_id: row.execution_id,
                status: serde_json::from_str(&row.status)?,
                started_at: row.started_at,
                completed_at: row.completed_at,
                input_payload: serde_json::from_str(&row.input_payload)?,
                output_payload: row.output_payload.map(|s| serde_json::from_str(&s).unwrap_or(Value::Null)),
                error: row.error,
            });
        }

        Ok(executions)
    }

    pub async fn list_all(&self) -> Result<Vec<FlowExecution>> {
        let rows = sqlx::query!(
            r#"
            SELECT execution_id, flow_id, status, started_at, completed_at, input_payload, output_payload, error
            FROM executions
            ORDER BY started_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut executions = Vec::new();
        for row in rows {
            executions.push(FlowExecution {
                flow_id: row.flow_id,
                execution_id: row.execution_id,
                status: serde_json::from_str(&row.status)?,
                started_at: row.started_at,
                completed_at: row.completed_at,
                input_payload: serde_json::from_str(&row.input_payload)?,
                output_payload: row.output_payload.map(|s| serde_json::from_str(&s).unwrap_or(Value::Null)),
                error: row.error,
            });
        }

        Ok(executions)
    }
}

