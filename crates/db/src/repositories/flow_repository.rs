use anyhow::Result;
use sqlx::SqlitePool;
use flowmason_core::types::Flow;

#[derive(Clone)]
pub struct FlowRepository {
    pool: SqlitePool,
}

impl FlowRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, flow: &Flow) -> Result<()> {
        let bricks_json = serde_json::to_string(&flow.bricks)?;
        let created_at_str = flow.created_at.to_rfc3339();
        let updated_at_str = flow.updated_at.to_rfc3339();
        let active_i64 = flow.active as i64;
        
        sqlx::query!(
            r#"
            INSERT INTO flows (id, name, description, bricks, active, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            flow.id,
            flow.name,
            flow.description,
            bricks_json,
            active_i64,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get(&self, id: &str) -> Result<Option<Flow>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, bricks, active, created_at, updated_at
            FROM flows
            WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            // Parse directly to Vec<BrickConfig> to avoid redundant parsing
            Ok(Some(Flow {
                id: row.id.expect("id should not be null"),
                name: row.name,
                description: row.description,
                bricks: serde_json::from_str(&row.bricks)?,
                active: row.active != 0,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse updated_at: {}", e))?
                    .with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Flow>> {
        let limit_val = limit.unwrap_or(100).min(1000) as i64; // Max 1000 items
        let offset_val = offset.unwrap_or(0) as i64;
        
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, bricks, active, created_at, updated_at
            FROM flows
            ORDER BY created_at DESC
            LIMIT ?1 OFFSET ?2
            "#,
            limit_val,
            offset_val
        )
        .fetch_all(&self.pool)
        .await?;

        let mut flows = Vec::new();
        for row in rows {
            // Parse directly to Vec<BrickConfig> to avoid redundant parsing
            flows.push(Flow {
                id: row.id.expect("id should not be null"),
                name: row.name,
                description: row.description,
                bricks: serde_json::from_str(&row.bricks)?,
                active: row.active != 0,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse updated_at: {}", e))?
                    .with_timezone(&chrono::Utc),
            });
        }

        Ok(flows)
    }

    pub async fn update(&self, flow: &Flow) -> Result<()> {
        let bricks_json = serde_json::to_string(&flow.bricks)?;
        let updated_at_str = flow.updated_at.to_rfc3339();
        let active_i64 = flow.active as i64;
        
        sqlx::query!(
            r#"
            UPDATE flows
            SET name = ?2, description = ?3, bricks = ?4, active = ?5, updated_at = ?6
            WHERE id = ?1
            "#,
            flow.id,
            flow.name,
            flow.description,
            bricks_json,
            active_i64,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM flows WHERE id = ?1", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flowmason_core::types::{BrickConfig, BrickType};
    use chrono::Utc;
    use serde_json::json;

    async fn create_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        crate::connection::init_schema(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_and_get_flow() {
        let pool = create_test_pool().await;
        let repo = FlowRepository::new(pool);

        let flow = Flow {
            id: "test-flow-1".to_string(),
            name: "Test Flow".to_string(),
            description: Some("Test Description".to_string()),
            bricks: vec![BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({}),
            }],
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repo.create(&flow).await.unwrap();
        let retrieved = repo.get("test-flow-1").await.unwrap();
        
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "test-flow-1");
        assert_eq!(retrieved.name, "Test Flow");
    }

    #[tokio::test]
    async fn test_list_flows() {
        let pool = create_test_pool().await;
        let repo = FlowRepository::new(pool);

        let flow1 = Flow {
            id: "test-flow-1".to_string(),
            name: "Test Flow 1".to_string(),
            description: None,
            bricks: vec![],
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let flow2 = Flow {
            id: "test-flow-2".to_string(),
            name: "Test Flow 2".to_string(),
            description: None,
            bricks: vec![],
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repo.create(&flow1).await.unwrap();
        repo.create(&flow2).await.unwrap();
        
        let flows = repo.list().await.unwrap();
        assert!(flows.len() >= 2);
    }

    #[tokio::test]
    async fn test_update_flow() {
        let pool = create_test_pool().await;
        let repo = FlowRepository::new(pool);

        let mut flow = Flow {
            id: "test-flow-1".to_string(),
            name: "Test Flow".to_string(),
            description: None,
            bricks: vec![],
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repo.create(&flow).await.unwrap();
        flow.name = "Updated Flow".to_string();
        flow.updated_at = Utc::now();
        
        repo.update(&flow).await.unwrap();
        let retrieved = repo.get("test-flow-1").await.unwrap().unwrap();
        assert_eq!(retrieved.name, "Updated Flow");
    }

    #[tokio::test]
    async fn test_delete_flow() {
        let pool = create_test_pool().await;
        let repo = FlowRepository::new(pool);

        let flow = Flow {
            id: "test-flow-1".to_string(),
            name: "Test Flow".to_string(),
            description: None,
            bricks: vec![],
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repo.create(&flow).await.unwrap();
        repo.delete("test-flow-1").await.unwrap();
        
        let retrieved = repo.get("test-flow-1").await.unwrap();
        assert!(retrieved.is_none());
    }
}

