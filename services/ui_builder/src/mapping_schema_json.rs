use flowmason_bricks::*;
use flowmason_core::{types::BrickType, Brick};
use serde_json::{json, Value};

pub fn generate_mapping_schema() -> Value {
    json!({
        "bricks": [
            {
                "type": "field_mapping",
                "name": "Field Mapping",
                "description": "Map fields from input to output",
                "schema": FieldMappingBrick.config_schema(),
                "icon": "🔀"
            },
            {
                "type": "combine_text",
                "name": "Combine Text",
                "description": "Combine multiple text fields",
                "schema": CombineTextBrick.config_schema(),
                "icon": "🔗"
            },
            {
                "type": "conditional",
                "name": "Conditional",
                "description": "Apply conditional logic",
                "schema": ConditionalBrick.config_schema(),
                "icon": "❓"
            }
        ],
        "ai_bricks": [
            {
                "type": "openai",
                "name": "OpenAI",
                "description": "GPT models for text generation",
                "schema": OpenAiBrick.config_schema(),
                "icon": "🤖"
            },
            {
                "type": "nvidia",
                "name": "NVIDIA",
                "description": "NVIDIA AI services (ASR, OCR, Text Generation)",
                "schema": NvidiaBrick.config_schema(),
                "icon": "🎮"
            }
        ],
        "connector_bricks": [
            {
                "type": "hubspot",
                "name": "HubSpot",
                "description": "CRM integration",
                "schema": HubSpotBrick.config_schema(),
                "icon": "📊"
            },
            {
                "type": "notion",
                "name": "Notion",
                "description": "Notion workspace integration",
                "schema": NotionBrick.config_schema(),
                "icon": "📝"
            },
            {
                "type": "odoo",
                "name": "Odoo",
                "description": "ERP integration",
                "schema": OdooBrick.config_schema(),
                "icon": "📦"
            },
            {
                "type": "n8n",
                "name": "n8n",
                "description": "Webhook integration",
                "schema": N8nBrick.config_schema(),
                "icon": "🔌"
            }
        ]
    })
}

pub fn get_brick_schema(brick_type: &BrickType) -> Option<Value> {
    match brick_type {
        BrickType::OpenAi => Some(OpenAiBrick.config_schema()),
        BrickType::Nvidia => Some(NvidiaBrick.config_schema()),
        BrickType::HubSpot => Some(HubSpotBrick.config_schema()),
        BrickType::Notion => Some(NotionBrick.config_schema()),
        BrickType::Odoo => Some(OdooBrick.config_schema()),
        BrickType::N8n => Some(N8nBrick.config_schema()),
        BrickType::FieldMapping => Some(FieldMappingBrick.config_schema()),
        BrickType::CombineText => Some(CombineTextBrick.config_schema()),
        BrickType::Conditional => Some(ConditionalBrick.config_schema()),
    }
}

