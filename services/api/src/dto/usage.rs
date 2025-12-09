use serde::{Deserialize, Serialize};
use flowmason_core::types::{BrickType, UsageLog as CoreUsageLog};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLogResponse {
    pub id: String,
    pub brick_name: String,
    pub flow_id: String,
    pub execution_id: String,
    pub timestamp: String,
    pub cost_unit: f64,
    pub token_usage: Option<u64>,
    pub metadata: Option<serde_json::Value>,
}

impl From<CoreUsageLog> for UsageLogResponse {
    fn from(log: CoreUsageLog) -> Self {
        Self {
            id: log.id,
            brick_name: log.brick_name,
            flow_id: log.flow_id,
            execution_id: log.execution_id,
            timestamp: log.timestamp.to_rfc3339(),
            cost_unit: log.cost_unit,
            token_usage: log.token_usage,
            metadata: log.metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatsResponse {
    pub brick_type: BrickType,
    pub daily_usage: u64,
    pub daily_limit: u64,
    pub monthly_usage: Option<u64>,
    pub monthly_limit: Option<u64>,
}

