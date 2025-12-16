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
            token_usage: log.token_usage.map(|v| v as u64),
            metadata: log.metadata,
        }
    }
}

// BrickIdentifier kept for potential future use in filtering/grouping usage stats
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BrickIdentifier {
    Type(BrickType),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatsResponse {
    pub brick_type: String,
    pub daily_usage: u64,
    pub daily_limit: u64,
    pub monthly_usage: Option<u64>,
    pub monthly_limit: Option<u64>,
}

impl UsageStatsResponse {
    pub fn from_brick_type(brick_type: BrickType, daily_usage: u64, daily_limit: u64, monthly_usage: Option<u64>, monthly_limit: Option<u64>) -> Self {
        // Convert enum to snake_case string
        let brick_type_str = match brick_type {
            BrickType::OpenAi => "openai",
            BrickType::Nvidia => "nvidia",
            BrickType::HubSpot => "hubspot",
            BrickType::Notion => "notion",
            BrickType::Odoo => "odoo",
            BrickType::N8n => "n8n",
            BrickType::FieldMapping => "field_mapping",
            BrickType::CombineText => "combine_text",
            BrickType::Conditional => "conditional",
        }.to_string();
        
        Self {
            brick_type: brick_type_str,
            daily_usage,
            daily_limit,
            monthly_usage,
            monthly_limit,
        }
    }

    pub fn from_custom_brick(brick_name: String, daily_usage: u64, daily_limit: u64, monthly_usage: Option<u64>, monthly_limit: Option<u64>) -> Self {
        Self {
            brick_type: brick_name,
            daily_usage,
            daily_limit,
            monthly_usage,
            monthly_limit,
        }
    }
}

