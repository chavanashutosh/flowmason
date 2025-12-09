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

/// In-memory quota manager for testing and development
pub struct InMemoryQuotaManager {
    quotas: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<BrickType, Quota>>>,
}

impl InMemoryQuotaManager {
    pub fn new() -> Self {
        let mut quotas = std::collections::HashMap::new();
        
        // Initialize default quotas
        quotas.insert(
            BrickType::OpenAi,
            Quota {
                brick_type: BrickType::OpenAi,
                daily_limit: 200,
                monthly_limit: Some(5000),
                current_daily_usage: 0,
                current_monthly_usage: Some(0),
            },
        );
        
        quotas.insert(
            BrickType::Nvidia,
            Quota {
                brick_type: BrickType::Nvidia,
                daily_limit: 1000,
                monthly_limit: Some(25000),
                current_daily_usage: 0,
                current_monthly_usage: Some(0),
            },
        );
        
        Self {
            quotas: std::sync::Arc::new(tokio::sync::RwLock::new(quotas)),
        }
    }
}

#[async_trait]
impl QuotaManager for InMemoryQuotaManager {
    async fn check_quota(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        let quotas = self.quotas.read().await;
        let quota = quotas
            .get(brick_type)
            .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {:?}", brick_type)))?;

        if quota.current_daily_usage >= quota.daily_limit {
            return Err(QuotaError::Exceeded(format!(
                "Daily limit of {} exceeded for {:?}",
                quota.daily_limit, brick_type
            )));
        }

        if let Some(monthly_limit) = quota.monthly_limit {
            if let Some(current_monthly) = quota.current_monthly_usage {
                if current_monthly >= monthly_limit {
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
        let mut quotas = self.quotas.write().await;
        if let Some(quota) = quotas.get_mut(brick_type) {
            quota.current_daily_usage += 1;
            if let Some(ref mut monthly) = quota.current_monthly_usage {
                *monthly += 1;
            }
        }
        Ok(())
    }

    async fn get_quota(&self, brick_type: &BrickType) -> Result<Quota, QuotaError> {
        let quotas = self.quotas.read().await;
        quotas
            .get(brick_type)
            .cloned()
            .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {:?}", brick_type)))
    }
}

impl Default for InMemoryQuotaManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Database-backed quota manager for production
pub struct DatabaseQuotaManager {
    pool: SqlitePool,
}

impl DatabaseQuotaManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Initialize default quotas if they don't exist
    async fn ensure_quota_exists(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        let brick_type_str = format!("{:?}", brick_type);
        
        // Check if quota exists
        let exists = sqlx::query(
            "SELECT 1 FROM quotas WHERE brick_type = ?1"
        )
        .bind(&brick_type_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;

        if exists.is_none() {
            // Initialize default quota based on brick type
            let (daily_limit, monthly_limit) = match brick_type {
                BrickType::OpenAi => (200, Some(5000)),
                BrickType::Nvidia => (1000, Some(25000)),
                _ => (1000, Some(10000)),
            };

            sqlx::query(
                r#"
                INSERT INTO quotas (brick_type, daily_limit, monthly_limit, current_daily_usage, current_monthly_usage, last_reset_date)
                VALUES (?1, ?2, ?3, 0, ?4, ?5)
                "#
            )
            .bind(&brick_type_str)
            .bind(daily_limit as i64)
            .bind(monthly_limit.map(|v| v as i64))
            .bind(monthly_limit.map(|_| 0i64))
            .bind(Utc::now().date_naive().to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    /// Reset daily usage if it's a new day
    async fn reset_daily_if_needed(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        let brick_type_str = format!("{:?}", brick_type);
        let today = Utc::now().date_naive().to_string();

        sqlx::query(
            r#"
            UPDATE quotas
            SET current_daily_usage = 0, last_reset_date = ?1
            WHERE brick_type = ?2 AND (last_reset_date IS NULL OR last_reset_date != ?1)
            "#
        )
        .bind(&today)
        .bind(&brick_type_str)
        .execute(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Reset monthly usage if it's a new month
    async fn reset_monthly_if_needed(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        let brick_type_str = format!("{:?}", brick_type);
        let current_month = Utc::now().format("%Y-%m").to_string();

        // Check last reset date to see if we need to reset monthly
        let last_reset = sqlx::query(
            "SELECT last_reset_date FROM quotas WHERE brick_type = ?1"
        )
        .bind(&brick_type_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;

        if let Some(row) = last_reset {
            if let Ok(Some(last_date)) = row.try_get::<Option<String>, _>("last_reset_date") {
                if !last_date.starts_with(&current_month) {
                        sqlx::query(
                            r#"
                            UPDATE quotas
                            SET current_monthly_usage = 0
                            WHERE brick_type = ?1
                            "#
                        )
                        .bind(&brick_type_str)
                        .execute(&self.pool)
                        .await
                        .map_err(|e| QuotaError::DatabaseError(e.to_string()))?;
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl QuotaManager for DatabaseQuotaManager {
    async fn check_quota(&self, brick_type: &BrickType) -> Result<(), QuotaError> {
        self.ensure_quota_exists(brick_type).await?;
        self.reset_daily_if_needed(brick_type).await?;
        self.reset_monthly_if_needed(brick_type).await?;

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
        .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {:?}", brick_type)))?;

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
        self.ensure_quota_exists(brick_type).await?;
        self.reset_daily_if_needed(brick_type).await?;
        self.reset_monthly_if_needed(brick_type).await?;

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
        self.ensure_quota_exists(brick_type).await?;
        self.reset_daily_if_needed(brick_type).await?;
        self.reset_monthly_if_needed(brick_type).await?;

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
        .ok_or_else(|| QuotaError::NotFound(format!("Quota not found for {:?}", brick_type)))?;

        Ok(Quota {
            brick_type: brick_type.clone(),
            daily_limit: row.get::<i64, _>("daily_limit") as u64,
            monthly_limit: row.get::<Option<i64>, _>("monthly_limit").map(|v| v as u64),
            current_daily_usage: row.get::<i64, _>("current_daily_usage") as u64,
            current_monthly_usage: row.get::<Option<i64>, _>("current_monthly_usage").map(|v| v as u64),
        })
    }
}

