use anyhow::Result;
use sqlx::SqlitePool;
use flowmason_core::types::{Template, Flow};

#[derive(Clone)]
pub struct TemplateRepository {
    pool: SqlitePool,
}

impl TemplateRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, template: &Template) -> Result<()> {
        let flow_config_json = serde_json::to_string(&template.flow_config)?;
        let created_at_str = template.created_at.to_rfc3339();
        let updated_at_str = template.updated_at.to_rfc3339();
        let is_system_i64 = template.is_system as i64;
        
        sqlx::query!(
            r#"
            INSERT INTO templates (id, name, description, category, flow_config, is_system, created_by, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            template.id,
            template.name,
            template.description,
            template.category,
            flow_config_json,
            is_system_i64,
            template.created_by,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get(&self, id: &str) -> Result<Option<Template>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, category, flow_config, is_system, created_by, created_at, updated_at
            FROM templates
            WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Template {
                id: row.id.expect("id should not be null"),
                name: row.name,
                description: row.description,
                category: row.category,
                flow_config: serde_json::from_str(&row.flow_config)?,
                is_system: row.is_system != 0,
                created_by: row.created_by,
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

    pub async fn list(&self, category: Option<&str>, include_system: bool, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Template>> {
        let limit_val = limit.unwrap_or(100).min(1000) as i64;
        let offset_val = offset.unwrap_or(0) as i64;
        
        let rows = if let Some(cat) = category {
            if include_system {
                sqlx::query!(
                    r#"
                    SELECT id, name, description, category, flow_config, is_system, created_by, created_at, updated_at
                    FROM templates
                    WHERE category = ?1
                    ORDER BY is_system DESC, created_at DESC
                    LIMIT ?2 OFFSET ?3
                    "#,
                    cat,
                    limit_val,
                    offset_val
                )
                .fetch_all(&self.pool)
                .await?
            } else {
                sqlx::query!(
                    r#"
                    SELECT id, name, description, category, flow_config, is_system, created_by, created_at, updated_at
                    FROM templates
                    WHERE category = ?1 AND is_system = 0
                    ORDER BY created_at DESC
                    LIMIT ?2 OFFSET ?3
                    "#,
                    cat,
                    limit_val,
                    offset_val
                )
                .fetch_all(&self.pool)
                .await?
            }
        } else {
            if include_system {
                sqlx::query!(
                    r#"
                    SELECT id, name, description, category, flow_config, is_system, created_by, created_at, updated_at
                    FROM templates
                    ORDER BY is_system DESC, created_at DESC
                    LIMIT ?1 OFFSET ?2
                    "#,
                    limit_val,
                    offset_val
                )
                .fetch_all(&self.pool)
                .await?
            } else {
                sqlx::query!(
                    r#"
                    SELECT id, name, description, category, flow_config, is_system, created_by, created_at, updated_at
                    FROM templates
                    WHERE is_system = 0
                    ORDER BY created_at DESC
                    LIMIT ?1 OFFSET ?2
                    "#,
                    limit_val,
                    offset_val
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        let mut templates = Vec::new();
        for row in rows {
            templates.push(Template {
                id: row.id.expect("id should not be null"),
                name: row.name,
                description: row.description,
                category: row.category,
                flow_config: serde_json::from_str(&row.flow_config)?,
                is_system: row.is_system != 0,
                created_by: row.created_by,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse created_at: {}", e))?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                    .map_err(|e| anyhow::anyhow!("Failed to parse updated_at: {}", e))?
                    .with_timezone(&chrono::Utc),
            });
        }

        Ok(templates)
    }

    pub async fn update(&self, template: &Template) -> Result<()> {
        let flow_config_json = serde_json::to_string(&template.flow_config)?;
        let updated_at_str = template.updated_at.to_rfc3339();
        let is_system_i64 = template.is_system as i64;
        
        sqlx::query!(
            r#"
            UPDATE templates
            SET name = ?2, description = ?3, category = ?4, flow_config = ?5, is_system = ?6, updated_at = ?7
            WHERE id = ?1
            "#,
            template.id,
            template.name,
            template.description,
            template.category,
            flow_config_json,
            is_system_i64,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        // Only allow deletion of non-system templates
        let template = self.get(id).await?;
        if let Some(t) = template {
            if t.is_system {
                return Err(anyhow::anyhow!("Cannot delete system template"));
            }
        }
        
        sqlx::query!("DELETE FROM templates WHERE id = ?1", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn list_categories(&self) -> Result<Vec<String>> {
        let rows = sqlx::query!(
            r#"
            SELECT DISTINCT category
            FROM templates
            ORDER BY category
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().filter_map(|r| r.category).collect())
    }
}
