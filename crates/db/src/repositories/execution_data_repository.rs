use anyhow::Result;
use sqlx::{SqlitePool, Row};
use serde_json::Value;
use uuid::Uuid;
use flowmason_core::{ExecutionDataStorage, BrickType};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct ExecutionData {
    pub id: String,
    pub execution_id: String,
    pub brick_index: i32,
    pub brick_type: String,
    pub data_type: String,
    pub data_key: String,
    pub data_value: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct ExecutionDataRepository {
    pool: SqlitePool,
}

impl ExecutionDataRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: &ExecutionData) -> Result<()> {
        let data_value_json = serde_json::to_string(&data.data_value)?;
        let timestamp_str = data.timestamp.to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO execution_data (id, execution_id, brick_index, brick_type, data_type, data_key, data_value, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
        )
        .bind(&data.id)
        .bind(&data.execution_id)
        .bind(data.brick_index)
        .bind(&data.brick_type)
        .bind(&data.data_type)
        .bind(&data.data_key)
        .bind(&data_value_json)
        .bind(&timestamp_str)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_simple(
        &self,
        execution_id: &str,
        brick_index: usize,
        brick_type: &BrickType,
        data_type: &str,
        data_key: &str,
        data_value: Value,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();
        let data = ExecutionData {
            id,
            execution_id: execution_id.to_string(),
            brick_index: brick_index as i32,
            brick_type: brick_type.as_str().to_string(),
            data_type: data_type.to_string(),
            data_key: data_key.to_string(),
            data_value,
            timestamp,
        };
        self.create(&data).await
    }

    fn row_to_execution_data(&self, row: &sqlx::sqlite::SqliteRow) -> Result<ExecutionData> {
        let data_value_str: String = row.get("data_value");
        let timestamp_str: String = row.get("timestamp");
        Ok(ExecutionData {
            id: row.get("id"),
            execution_id: row.get("execution_id"),
            brick_index: row.get("brick_index"),
            brick_type: row.get("brick_type"),
            data_type: row.get("data_type"),
            data_key: row.get("data_key"),
            data_value: serde_json::from_str(&data_value_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse data_value JSON: {}", e))?,
            timestamp: chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))?
                .with_timezone(&chrono::Utc),
        })
    }

    pub async fn get_by_execution(&self, execution_id: &str) -> Result<Vec<ExecutionData>> {
        let rows = sqlx::query(
            r#"
            SELECT id, execution_id, brick_index, brick_type, data_type, data_key, data_value, timestamp
            FROM execution_data
            WHERE execution_id = ?1
            ORDER BY brick_index, timestamp
            "#,
        )
        .bind(execution_id)
        .fetch_all(&self.pool)
        .await?;

        let mut data_list = Vec::new();
        for row in rows {
            data_list.push(self.row_to_execution_data(&row)?);
        }

        Ok(data_list)
    }

    pub async fn get_by_brick(
        &self,
        execution_id: &str,
        brick_index: i32,
    ) -> Result<Vec<ExecutionData>> {
        let rows = sqlx::query(
            r#"
            SELECT id, execution_id, brick_index, brick_type, data_type, data_key, data_value, timestamp
            FROM execution_data
            WHERE execution_id = ?1 AND brick_index = ?2
            ORDER BY timestamp
            "#,
        )
        .bind(execution_id)
        .bind(brick_index)
        .fetch_all(&self.pool)
        .await?;

        let mut data_list = Vec::new();
        for row in rows {
            data_list.push(self.row_to_execution_data(&row)?);
        }

        Ok(data_list)
    }

    pub async fn get_by_data_type(
        &self,
        execution_id: &str,
        data_type: &str,
    ) -> Result<Vec<ExecutionData>> {
        let rows = sqlx::query(
            r#"
            SELECT id, execution_id, brick_index, brick_type, data_type, data_key, data_value, timestamp
            FROM execution_data
            WHERE execution_id = ?1 AND data_type = ?2
            ORDER BY brick_index, timestamp
            "#,
        )
        .bind(execution_id)
        .bind(data_type)
        .fetch_all(&self.pool)
        .await?;

        let mut data_list = Vec::new();
        for row in rows {
            data_list.push(self.row_to_execution_data(&row)?);
        }

        Ok(data_list)
    }

    pub async fn get_by_key(
        &self,
        execution_id: &str,
        data_key: &str,
    ) -> Result<Option<ExecutionData>> {
        let row = sqlx::query(
            r#"
            SELECT id, execution_id, brick_index, brick_type, data_type, data_key, data_value, timestamp
            FROM execution_data
            WHERE execution_id = ?1 AND data_key = ?2
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
        )
        .bind(execution_id)
        .bind(data_key)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(self.row_to_execution_data(&row)?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_by_execution(&self, execution_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM execution_data
            WHERE execution_id = ?1
            "#,
        )
        .bind(execution_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_summary(&self, execution_id: &str) -> Result<ExecutionDataSummary> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN data_type = 'fetched' THEN 1 ELSE 0 END) as fetched_count,
                SUM(CASE WHEN data_type = 'intermediate' THEN 1 ELSE 0 END) as intermediate_count
            FROM execution_data
            WHERE execution_id = ?1
            "#,
        )
        .bind(execution_id)
        .fetch_one(&self.pool)
        .await?;

        let total: i64 = row.get("total");
        let fetched_count: Option<i64> = row.get("fetched_count");
        let intermediate_count: Option<i64> = row.get("intermediate_count");

        let brick_rows = sqlx::query(
            r#"
            SELECT brick_type, COUNT(*) as count
            FROM execution_data
            WHERE execution_id = ?1
            GROUP BY brick_type
            "#,
        )
        .bind(execution_id)
        .fetch_all(&self.pool)
        .await?;

        let mut brick_data_count = std::collections::HashMap::new();
        for row in brick_rows {
            let brick_type: String = row.get("brick_type");
            let count: i64 = row.get("count");
            brick_data_count.insert(brick_type, count as u32);
        }

        Ok(ExecutionDataSummary {
            total_records: total as u32,
            fetched_count: fetched_count.unwrap_or(0) as u32,
            intermediate_count: intermediate_count.unwrap_or(0) as u32,
            brick_data_count,
        })
    }
}

#[async_trait]
impl ExecutionDataStorage for ExecutionDataRepository {
    async fn store_data(
        &self,
        execution_id: &str,
        brick_index: usize,
        brick_type: &BrickType,
        data_type: &str,
        data_key: &str,
        data_value: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.create_simple(execution_id, brick_index, brick_type, data_type, data_key, data_value)
            .await
            .map_err(|e| {
                let err_msg = format!("Failed to store execution data: {}", e);
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn std::error::Error + Send + Sync>
            })
    }
}

#[derive(Debug)]
pub struct ExecutionDataSummary {
    pub total_records: u32,
    pub fetched_count: u32,
    pub intermediate_count: u32,
    pub brick_data_count: std::collections::HashMap<String, u32>,
}
