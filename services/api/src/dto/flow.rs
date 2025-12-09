use serde::{Deserialize, Serialize};
use serde_json::Value;
use flowmason_core::types::{BrickType, Flow as CoreFlow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFlowRequest {
    pub name: String,
    pub description: Option<String>,
    pub bricks: Vec<BrickConfigDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFlowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub bricks: Option<Vec<BrickConfigDto>>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickConfigDto {
    pub brick_type: BrickType,
    pub config: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bricks: Vec<BrickConfigDto>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<CoreFlow> for FlowResponse {
    fn from(flow: CoreFlow) -> Self {
        Self {
            id: flow.id,
            name: flow.name,
            description: flow.description,
            bricks: flow.bricks.into_iter().map(|b| BrickConfigDto {
                brick_type: b.brick_type,
                config: b.config,
            }).collect(),
            active: flow.active,
            created_at: flow.created_at.to_rfc3339(),
            updated_at: flow.updated_at.to_rfc3339(),
        }
    }
}

