use anyhow::Result;
use sqlx::SqlitePool;

pub struct ApiKeyRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub id: String,
    pub user_id: String,
    pub key_hash: String,
    pub name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ApiKeyRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_id: &str, key_hash: &str, name: Option<&str>) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query!(
            r#"
            INSERT INTO api_keys (id, user_id, key_hash, name, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            id,
            user_id,
            key_hash,
            name,
            now
        )
        .execute(&self.pool)
        .await?;
        
        Ok(id)
    }

    pub async fn get_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, key_hash, name, created_at, last_used_at
            FROM api_keys
            WHERE key_hash = ?1
            "#,
            key_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(ApiKey {
                id: row.id.expect("id should not be null"),
                user_id: row.user_id,
                key_hash: row.key_hash,
                name: row.name,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                last_used_at: row.last_used_at.as_ref().map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .map_err(|e| anyhow::anyhow!("Failed to parse last_used_at: {}", e))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }).transpose()?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_last_used(&self, key_hash: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query!(
            r#"
            UPDATE api_keys
            SET last_used_at = ?1
            WHERE key_hash = ?2
            "#,
            now,
            key_hash
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn list_by_user(&self, user_id: &str) -> Result<Vec<ApiKey>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, key_hash, name, created_at, last_used_at
            FROM api_keys
            WHERE user_id = ?1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|row| {
            ApiKey {
                id: row.id.expect("id should not be null"),
                user_id: row.user_id,
                key_hash: row.key_hash,
                name: row.name,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                last_used_at: row.last_used_at.as_ref().map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s)
                        .unwrap()
                        .with_timezone(&chrono::Utc)
                }),
            }
        }).collect())
    }

    pub async fn get(&self, id: &str) -> Result<Option<ApiKey>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, key_hash, name, created_at, last_used_at
            FROM api_keys
            WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(ApiKey {
                id: row.id.expect("id should not be null"),
                user_id: row.user_id,
                key_hash: row.key_hash,
                name: row.name,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                last_used_at: row.last_used_at.as_ref().map(|s| {
                    chrono::DateTime::parse_from_rfc3339(s.as_str())
                        .map_err(|e| anyhow::anyhow!("Failed to parse last_used_at: {}", e))
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                }).transpose()?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM api_keys WHERE id = ?1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

