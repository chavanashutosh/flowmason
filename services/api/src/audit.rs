use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct AuditLogger {
    pool: SqlitePool,
}

impl AuditLogger {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn log(
        &self,
        user_id: &str,
        action: &str,
        resource_type: &str,
        resource_id: &str,
        details: Option<serde_json::Value>,
        ip_address: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let details_json = details.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default());

        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, details, ip_address, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#
        )
        .bind(&id)
        .bind(user_id)
        .bind(action)
        .bind(resource_type)
        .bind(resource_id)
        .bind(details_json.as_deref())
        .bind(ip_address)
        .bind(timestamp.to_rfc3339())
        .execute(&self.pool)
        .await?;

        tracing::info!(
            user_id = %user_id,
            action = %action,
            resource_type = %resource_type,
            resource_id = %resource_id,
            "Audit log recorded"
        );

        Ok(())
    }
}
