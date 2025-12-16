use async_trait::async_trait;
use thiserror::Error;
use sqlx::{SqlitePool, Row};
use chrono::Utc;

use crate::types::{BrickType, Quota};

#[derive(Debug, Error)]
pub enum QuotaError {
    #[error("Quota exceeded: {0}")]
    Exceeded(String),
    
    #[error("Quota not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[async_trait]
pub trait QuotaManager: Send + Sync {
    /// Checks if a quota allows execution
    async fn check_quota(&self, brick_type: &BrickType) -> Result<(), QuotaError>;
    
    /// Records usage after execution
    async fn record_usage(
        &self,
        brick_type: &BrickType,
        cost_unit: f64,
        token_usage: Option<u64>,
    ) -> Result<(), QuotaError>;
    
    /// Gets current quota status
    async fn get_quota(&self, brick_type: &BrickType) -> Result<Quota, QuotaError>;
}

/// Database-backed quota manager for production
pub struct DatabaseQuotaManager {
    pool: SqlitePool,
}

impl DatabaseQuotaManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Initialize default quotas if they don't exist and reset if needed
    /// Optimized to use a single UPSERT and single UPDATE query instead of multiple queries
    async fn ensure_quota_and_reset_if_needed(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        let brick_type_str = brick_type.as_str();
        let today = Utc::now().date_naive().to_string();
        let current_month = Utc::now().format("%Y-%m").to_string();
        
        // Initialize default quota based on brick type
        let (daily_limit, monthly_limit) = match brick_type {
            BrickType::OpenAi => (200, Some(5000)),
            BrickType::Nvidia => (1000, Some(25000)),
            _ => (1000, Some(10000)),
        };

        // Use INSERT OR IGNORE to create quota if it doesn't exist
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO quotas (brick_type, daily_limit, monthly_limit, current_daily_usage, current_monthly_usage, last_reset_date)
            VALUES (?1, ?2, ?3, 0, ?4, ?5)
            "#
        )
        .bind(&brick_type_str)
        .bind(daily_limit as i64)
        .bind(monthly_limit.map(|v| v as i64))
        .bind(monthly_limit.map(|_| 0i64))
        .bind(&today)
        .execute(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;

        // Combined reset query: reset daily and monthly usage in a single UPDATE
        // Uses CASE statements to conditionally reset based on date checks
        sqlx::query(
            r#"
            UPDATE quotas
            SET 
                current_daily_usage = CASE 
                    WHEN last_reset_date IS NULL OR last_reset_date != ?1 THEN 0 
                    ELSE current_daily_usage 
                END,
                current_monthly_usage = CASE 
                    WHEN last_reset_date IS NULL OR last_reset_date NOT LIKE ?2 || '%' THEN 0 
                    ELSE current_monthly_usage 
                END,
                last_reset_date = CASE 
                    WHEN last_reset_date IS NULL OR last_reset_date != ?1 THEN ?1 
                    ELSE last_reset_date 
                END
            WHERE brick_type = ?3
            "#
        )
        .bind(&today)
        .bind(&current_month)
        .bind(&brick_type_str)
        .execute(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl QuotaManager for DatabaseQuotaManager {
    async fn check_quota(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        // Ensure quota exists and reset if needed (single optimized operation)
        self.ensure_quota_and_reset_if_needed(brick_type).await?;

        let brick_type_str = format!("{:?}", brick_type);
        let row = sqlx::query(
            r#"
            SELECT daily_limit, monthly_limit, current_daily_usage, current_monthly_usage
            FROM quotas
            WHERE brick_type = ?1
            "#
        )
        .bind(&brick_type_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?
        .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {}", brick_type.as_str())))?;

        let daily_limit: i64 = row.get("daily_limit");
        let monthly_limit: Option<i64> = row.get("monthly_limit");
        let current_daily_usage: i64 = row.get("current_daily_usage");
        let current_monthly_usage: Option<i64> = row.get("current_monthly_usage");

        if current_daily_usage as u64 >= daily_limit as u64 {
            return Err(QuotaError::Exceeded(format!(
                "Daily limit of {} exceeded for {:?}",
                daily_limit, brick_type
            )));
        }

        if let Some(monthly_limit) = monthly_limit {
            if let Some(current_monthly) = current_monthly_usage {
                if current_monthly as u64 >= monthly_limit as u64 {
                    return Err(QuotaError::Exceeded(format!(
                        "Monthly limit of {} exceeded for {:?}",
                        monthly_limit, brick_type
                    )));
                }
            }
        }

        Ok(())
    }

    async fn record_usage(
        &self,
        brick_type: &BrickType,
        _cost_unit: f64,
        _token_usage: Option<u64>,
    ) -> Result<(), QuotaError> {
        // Ensure quota exists and reset if needed (single optimized operation)
        self.ensure_quota_and_reset_if_needed(brick_type).await?;

        let brick_type_str = format!("{:?}", brick_type);
        
        sqlx::query(
            r#"
            UPDATE quotas
            SET current_daily_usage = current_daily_usage + 1,
                current_monthly_usage = COALESCE(current_monthly_usage, 0) + 1
            WHERE brick_type = ?1
            "#
        )
        .bind(&brick_type_str)
        .execute(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_quota(&self, brick_type: &BrickType) -> Result<Quota, QuotaError> {
        // Ensure quota exists and reset if needed (single optimized operation)
        self.ensure_quota_and_reset_if_needed(brick_type).await?;

        let brick_type_str = format!("{:?}", brick_type);
        let row = sqlx::query(
            r#"
            SELECT daily_limit, monthly_limit, current_daily_usage, current_monthly_usage
            FROM quotas
            WHERE brick_type = ?1
            "#
        )
        .bind(&brick_type_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?
        .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {}", brick_type.as_str())))?;

        Ok(Quota {
            brick_type: brick_type.clone(),
            daily_limit: row.get::<i64, _>("daily_limit") as u64,
            monthly_limit: row.get::<Option<i64>, _>("monthly_limit").map(|v| v as u64),
            current_daily_usage: row.get::<i64, _>("current_daily_usage") as u64,
            current_monthly_usage: row.get::<Option<i64>, _>("current_monthly_usage").map(|v| v as u64),
        })
    }
}

/// In-memory quota manager for testing
#[cfg(test)]
pub struct InMemoryQuotaManager {
    quotas: std::collections::HashMap<BrickType, Quota>,
}

#[cfg(test)]
impl InMemoryQuotaManager {
    pub fn new() -> Self {
        Self {
            quotas: std::collections::HashMap::new(),
        }
    }

    fn ensure_quota(&mut self, brick_type: &BrickType) {
        if !self.quotas.contains_key(brick_type) {
            let (daily_limit, monthly_limit) = match brick_type {
                BrickType::OpenAi => (200, Some(5000)),
                BrickType::Nvidia => (1000, Some(25000)),
                _ => (1000, Some(10000)),
            };
            self.quotas.insert(
                brick_type.clone(),
                Quota {
                    brick_type: brick_type.clone(),
                    daily_limit,
                    monthly_limit,
                    current_daily_usage: 0,
                    current_monthly_usage: Some(0),
                },
            );
        }
    }
}

#[cfg(test)]
#[async_trait::async_trait]
impl QuotaManager for InMemoryQuotaManager {
    async fn check_quota(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        let quota = self
            .quotas
            .get(brick_type)
            .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {}", brick_type.as_str())))?;

        if quota.current_daily_usage >= quota.daily_limit {
            return Err(QuotaError::Exceeded(format!(
                "Daily limit of {} exceeded",
                quota.daily_limit
            )));
        }

        if let Some(monthly_limit) = quota.monthly_limit {
            if let Some(current_monthly) = quota.current_monthly_usage {
                if current_monthly >= monthly_limit {
                    return Err(QuotaError::Exceeded(format!(
                        "Monthly limit of {} exceeded",
                        monthly_limit
                    )));
                }
            }
        }

        Ok(())
    }

    async fn record_usage(
        &self,
        brick_type: &BrickType,
        _cost_unit: f64,
        _token_usage: Option<u64>,
    ) -> Result<(), QuotaError> {
        // In-memory implementation would need mutability, but for testing we'll use a different approach
        // This is a limitation of the trait design - in real tests, we'd use Arc<Mutex<>> or similar
        Ok(())
    }

    async fn get_quota(&self, brick_type: &BrickType) -> Result<Quota, QuotaError> {
        self.quotas
            .get(brick_type)
            .cloned()
            .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {}", brick_type.as_str())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_quota_check() {
        let mut manager = InMemoryQuotaManager::new();
        manager.ensure_quota(&BrickType::OpenAi);

        // Should pass when under limit
        let result = manager.check_quota(&BrickType::OpenAi).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_in_memory_quota_not_found() {
        let manager = InMemoryQuotaManager::new();
        let result = manager.check_quota(&BrickType::OpenAi).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QuotaError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_get_quota() {
        let mut manager = InMemoryQuotaManager::new();
        manager.ensure_quota(&BrickType::OpenAi);

        let quota = manager.get_quota(&BrickType::OpenAi).await.unwrap();
        assert_eq!(quota.brick_type, BrickType::OpenAi);
        assert_eq!(quota.daily_limit, 200);
        assert_eq!(quota.monthly_limit, Some(5000));
    }
}

