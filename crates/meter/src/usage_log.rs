use anyhow::Result;
use chrono::Utc;
use flowmason_core::types::{BrickType, UsageLog};
use flowmason_core::UsageLogger as CoreUsageLogger;
use flowmason_db::repositories::UsageLogRepository;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use async_trait::async_trait;

/// In-memory usage logger for development
/// In production, this would use a database
pub struct UsageLogger {
    logs: Arc<RwLock<Vec<UsageLog>>>,
}

impl UsageLogger {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Records usage for a brick execution (internal implementation)
    async fn record_usage_internal(
        &self,
        brick_name: &str,
        _brick_type: &BrickType,
        flow_id: &str,
        execution_id: &str,
        cost_unit: f64,
        token_usage: Option<u64>,
        metadata: Option<Value>,
    ) -> Result<String> {
        let log = UsageLog {
            id: Uuid::new_v4().to_string(),
            brick_name: brick_name.to_string(),
            flow_id: flow_id.to_string(),
            execution_id: execution_id.to_string(),
            timestamp: Utc::now(),
            cost_unit,
            token_usage: token_usage.map(|v| v as i64),
            metadata,
        };

        let log_id = log.id.clone();
        self.logs.write().await.push(log);

        Ok(log_id)
    }

    /// Gets usage logs for a brick type
    pub async fn get_usage_by_brick_type(
        &self,
        brick_type: &BrickType,
    ) -> Result<Vec<UsageLog>> {
        let logs = self.logs.read().await;
        Ok(logs
            .iter()
            .filter(|log| {
                // Match brick type by name (simplified - in production use enum)
                log.brick_name == format!("{:?}", brick_type).to_lowercase()
            })
            .cloned()
            .collect())
    }

    /// Gets usage logs for a flow
    pub async fn get_usage_by_flow(&self, flow_id: &str) -> Result<Vec<UsageLog>> {
        let logs = self.logs.read().await;
        Ok(logs
            .iter()
            .filter(|log| log.flow_id == flow_id)
            .cloned()
            .collect())
    }

    /// Gets total usage count for a brick type today
    pub async fn get_daily_usage_count(&self, brick_type: &BrickType) -> Result<u64> {
        let logs = self.logs.read().await;
        let today = Utc::now().date_naive();
        
        Ok(logs
            .iter()
            .filter(|log| {
                log.timestamp.date_naive() == today
                    && log.brick_name == format!("{:?}", brick_type).to_lowercase()
            })
            .count() as u64)
    }

    /// Gets all usage logs
    pub async fn get_all_logs(&self) -> Result<Vec<UsageLog>> {
        let logs = self.logs.read().await;
        Ok(logs.clone())
    }
}

impl Default for UsageLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Database-backed usage logger for production
pub struct DatabaseUsageLogger {
    repo: UsageLogRepository,
}

impl DatabaseUsageLogger {
    pub fn new(repo: UsageLogRepository) -> Self {
        Self { repo }
    }

    /// Gets usage logs for a brick type
    pub async fn get_usage_by_brick_type(
        &self,
        brick_type: &BrickType,
    ) -> Result<Vec<UsageLog>> {
        self.repo.list_by_brick_type(brick_type).await
    }

    /// Gets usage logs for a flow
    pub async fn get_usage_by_flow(&self, flow_id: &str) -> Result<Vec<UsageLog>> {
        self.repo.list_by_flow(flow_id).await
    }

    /// Gets total usage count for a brick type today
    pub async fn get_daily_usage_count(&self, brick_type: &BrickType) -> Result<u64> {
        self.repo.get_daily_usage_count(brick_type).await
    }

    /// Gets all usage logs
    pub async fn get_all_logs(&self) -> Result<Vec<UsageLog>> {
        self.repo.list_all().await
    }
}

#[async_trait]
impl CoreUsageLogger for DatabaseUsageLogger {
    async fn record_usage(
        &self,
        brick_name: &str,
        _brick_type: &BrickType,
        flow_id: &str,
        execution_id: &str,
        cost_unit: f64,
        token_usage: Option<u64>,
        metadata: Option<Value>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let log = UsageLog {
            id: Uuid::new_v4().to_string(),
            brick_name: brick_name.to_string(),
            flow_id: flow_id.to_string(),
            execution_id: execution_id.to_string(),
            timestamp: Utc::now(),
            cost_unit,
            token_usage: token_usage.map(|v| v as i64),
            metadata,
        };

        let log_id = log.id.clone();
        self.repo.create(&log).await.map_err(|e| {
            let err_msg = e.to_string();
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn std::error::Error + Send + Sync>
        })?;

        Ok(log_id)
    }

    async fn get_all_logs(
        &self,
    ) -> Result<Vec<UsageLog>, Box<dyn std::error::Error + Send + Sync>> {
        self.repo.list_all().await.map_err(|e| {
            let err_msg = e.to_string();
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn std::error::Error + Send + Sync>
        })
    }

    async fn get_daily_usage_count(
        &self,
        brick_type: &BrickType,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.repo.get_daily_usage_count(brick_type).await.map_err(|e| {
            let err_msg = e.to_string();
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn std::error::Error + Send + Sync>
        })
    }
}

#[async_trait]
impl CoreUsageLogger for UsageLogger {
    async fn record_usage(
        &self,
        brick_name: &str,
        brick_type: &BrickType,
        flow_id: &str,
        execution_id: &str,
        cost_unit: f64,
        token_usage: Option<u64>,
        metadata: Option<Value>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.record_usage_internal(brick_name, brick_type, flow_id, execution_id, cost_unit, token_usage, metadata)
            .await
            .map_err(|e| {
                let err_msg = e.to_string();
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)) as Box<dyn std::error::Error + Send + Sync>
            })
    }

    async fn get_all_logs(
        &self,
    ) -> Result<Vec<UsageLog>, Box<dyn std::error::Error + Send + Sync>> {
        let logs = self.logs.read().await;
        Ok(logs.clone())
    }

    async fn get_daily_usage_count(
        &self,
        brick_type: &BrickType,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let logs = self.logs.read().await;
        let today = Utc::now().date_naive();
        Ok(logs
            .iter()
            .filter(|log| {
                log.timestamp.date_naive() == today
                    && log.brick_name == format!("{:?}", brick_type).to_lowercase()
            })
            .count() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_usage() {
        let logger = UsageLogger::new();
        let result = logger
            .record_usage_internal(
                "openai",
                &BrickType::OpenAi,
                "flow_1",
                "exec_1",
                0.01,
                Some(100),
                None,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_daily_usage() {
        let logger = UsageLogger::new();
        logger
            .record_usage_internal(
                "openai",
                &BrickType::OpenAi,
                "flow_1",
                "exec_1",
                0.01,
                None,
                None,
            )
            .await
            .unwrap();
        
        let count = logger.get_daily_usage_count(&BrickType::OpenAi).await.unwrap();
        assert_eq!(count, 1);
    }
}

