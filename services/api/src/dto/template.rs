use serde::{Deserialize, Serialize};
use flowmason_core::types::{Template, Flow};
use crate::dto::FlowResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub flow_config: Flow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub flow_config: Option<Flow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub flow_config: FlowResponse,
    pub is_system: bool,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiateTemplateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<Template> for TemplateResponse {
    fn from(template: Template) -> Self {
        Self {
            id: template.id,
            name: template.name,
            description: template.description,
            category: template.category,
            flow_config: FlowResponse::from(template.flow_config),
            is_system: template.is_system,
            created_by: template.created_by,
            created_at: template.created_at.to_rfc3339(),
            updated_at: template.updated_at.to_rfc3339(),
        }
    }
}
