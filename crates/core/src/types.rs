use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum BrickType {
    OpenAi,
    Nvidia,
    HubSpot,
    Notion,
    Odoo,
    N8n,
    FieldMapping,
    CombineText,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickConfig {
    pub brick_type: BrickType,
    pub config: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bricks: Vec<BrickConfig>,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowExecution {
    pub flow_id: String,
    pub execution_id: String,
    pub status: ExecutionStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub input_payload: Value,
    pub output_payload: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLog {
    pub id: String,
    pub brick_name: String,
    pub flow_id: String,
    pub execution_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cost_unit: f64,
    pub token_usage: Option<u64>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    pub brick_type: BrickType,
    pub daily_limit: u64,
    pub monthly_limit: Option<u64>,
    pub current_daily_usage: u64,
    pub current_monthly_usage: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    pub source_path: String,
    pub target_path: String,
    pub transform: Option<TransformType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformType {
    StringConcat { separator: String },
    NumberAdd,
    NumberMultiply,
    StringToUpper,
    StringToLower,
    Conditional { condition: String, true_value: Value, false_value: Value },
}

