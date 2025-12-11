use anyhow::Result;
use sqlx::SqlitePool;
use flowmason_core::types::{UsageLog, BrickType};
use serde_json::Value;
use chrono::Utc;

#[derive(Clone)]
pub struct UsageLogRepository {
    pool: SqlitePool,
}

impl UsageLogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, log: &UsageLog) -> Result<()> {
        let timestamp_str = log.timestamp.to_rfc3339();
        let token_usage_i64 = log.token_usage.map(|v| v as i64);
        let metadata_json = log.metadata.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default());
        
        sqlx::query!(
            r#"
            INSERT INTO usage_logs (id, brick_name, flow_id, execution_id, timestamp, cost_unit, token_usage, metadata)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            log.id,
            log.brick_name,
            log.flow_id,
            log.execution_id,
            timestamp_str,
            log.cost_unit,
            token_usage_i64,
            metadata_json
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn list_by_flow(&self, flow_id: &str) -> Result<Vec<UsageLog>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, brick_name, flow_id, execution_id, timestamp, cost_unit, token_usage, metadata
            FROM usage_logs
            WHERE flow_id = ?1
            ORDER BY timestamp DESC
            "#,
            flow_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(UsageLog {
                id: row.id.expect("id should not be null"),
                brick_name: row.brick_name,
                flow_id: row.flow_id,
                execution_id: row.execution_id,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.timestamp)
                    .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))?
                    .with_timezone(&chrono::Utc),
                cost_unit: row.cost_unit,
                token_usage: row.token_usage,
                metadata: row.metadata.as_ref().map(|s| serde_json::from_str(s.as_str()).unwrap_or(Value::Null)),
            });
        }

        Ok(logs)
    }

    pub async fn list_all(&self) -> Result<Vec<UsageLog>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, brick_name, flow_id, execution_id, timestamp, cost_unit, token_usage, metadata
            FROM usage_logs
            ORDER BY timestamp DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(UsageLog {
                id: row.id.expect("id should not be null"),
                brick_name: row.brick_name,
                flow_id: row.flow_id,
                execution_id: row.execution_id,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.timestamp)
                    .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))?
                    .with_timezone(&chrono::Utc),
                cost_unit: row.cost_unit,
                token_usage: row.token_usage,
                metadata: row.metadata.as_ref().map(|s| serde_json::from_str(s.as_str()).unwrap_or(Value::Null)),
            });
        }

        Ok(logs)
    }

    pub async fn list_by_brick_type(&self, brick_type: &BrickType) -> Result<Vec<UsageLog>> {
        let brick_name = format!("{:?}", brick_type).to_lowercase();
        let rows = sqlx::query!(
            r#"
            SELECT id, brick_name, flow_id, execution_id, timestamp, cost_unit, token_usage, metadata
            FROM usage_logs
            WHERE brick_name = ?1
            ORDER BY timestamp DESC
            "#,
            brick_name
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(UsageLog {
                id: row.id.expect("id should not be null"),
                brick_name: row.brick_name,
                flow_id: row.flow_id,
                execution_id: row.execution_id,
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.timestamp)
                    .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))?
                    .with_timezone(&chrono::Utc),
                cost_unit: row.cost_unit,
                token_usage: row.token_usage,
                metadata: row.metadata.as_ref().map(|s| serde_json::from_str(s.as_str()).unwrap_or(Value::Null)),
            });
        }

        Ok(logs)
    }

    pub async fn get_daily_usage_count(&self, brick_type: &BrickType) -> Result<u64> {
        let brick_name = format!("{:?}", brick_type).to_lowercase();
        let today = Utc::now().date_naive();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap().and_utc();
        let today_end = today.and_hms_opt(23, 59, 59).unwrap().and_utc();
        let today_start_str = today_start.to_rfc3339();
        let today_end_str = today_end.to_rfc3339();

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM usage_logs
            WHERE brick_name = ?1
            AND timestamp >= ?2
            AND timestamp <= ?3
            "#,
            brick_name,
            today_start_str,
            today_end_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count as u64)
    }

    /// Get daily usage count for a brick by name (for custom bricks)
    pub async fn get_daily_usage_count_by_name(&self, brick_name: &str) -> Result<u64> {
        let today = Utc::now().date_naive();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap().and_utc();
        let today_end = today.and_hms_opt(23, 59, 59).unwrap().and_utc();
        let today_start_str = today_start.to_rfc3339();
        let today_end_str = today_end.to_rfc3339();

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM usage_logs
            WHERE brick_name = ?1
            AND timestamp >= ?2
            AND timestamp <= ?3
            "#,
            brick_name,
            today_start_str,
            today_end_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count as u64)
    }

    /// Get all unique brick names from usage_logs table
    pub async fn get_unique_brick_names(&self) -> Result<Vec<String>> {
        let rows = sqlx::query!(
            r#"
            SELECT DISTINCT brick_name
            FROM usage_logs
            ORDER BY brick_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|row| row.brick_name).collect())
    }
}

