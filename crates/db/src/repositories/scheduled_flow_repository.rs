use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ScheduledFlow {
    pub id: String,
    pub flow_id: String,
    pub cron_expression: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct ScheduledFlowRepository {
    pool: SqlitePool,
}

impl ScheduledFlowRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, flow_id: &str, cron_expression: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            INSERT INTO scheduled_flows (id, flow_id, cron_expression, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            id,
            flow_id,
            cron_expression,
            now,
            now
        )
        .execute(&self.pool)
        .await?;
        
        Ok(id)
    }

    pub async fn get_by_flow_id(&self, flow_id: &str) -> Result<Option<ScheduledFlow>> {
        let row = sqlx::query!(
            r#"
            SELECT id, flow_id, cron_expression, created_at, updated_at
            FROM scheduled_flows
            WHERE flow_id = ?1
            "#,
            flow_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(ScheduledFlow {
                id: row.id,
                flow_id: row.flow_id,
                cron_expression: row.cron_expression,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_all(&self) -> Result<Vec<ScheduledFlow>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, flow_id, cron_expression, created_at, updated_at
            FROM scheduled_flows
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|row| ScheduledFlow {
            id: row.id,
            flow_id: row.flow_id,
            cron_expression: row.cron_expression,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        }).collect())
    }

    pub async fn delete(&self, flow_id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM scheduled_flows
            WHERE flow_id = ?1
            "#,
            flow_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn update(&self, flow_id: &str, cron_expression: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            UPDATE scheduled_flows
            SET cron_expression = ?1, updated_at = ?2
            WHERE flow_id = ?3
            "#,
            cron_expression,
            now,
            flow_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

