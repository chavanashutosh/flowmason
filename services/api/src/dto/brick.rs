use serde::{Deserialize, Serialize};
use serde_json::Value;
use flowmason_core::types::BrickType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickSchemaResponse {
    pub brick_type: BrickType,
    pub name: String,
    pub config_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickListResponse {
    pub bricks: Vec<BrickSchemaResponse>,
}

