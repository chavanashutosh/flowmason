use axum::{
    response::Json,
    routing::get,
    Router,
};

use crate::dto::{BrickListResponse, BrickSchemaResponse};
use flowmason_bricks::*;
use flowmason_core::types::BrickType;
use flowmason_core::Brick;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_bricks))
        .route("/:brick_type/schema", get(get_brick_schema))
}

async fn list_bricks() -> Json<BrickListResponse> {
    let openai = OpenAiBrick;
    let nvidia = NvidiaBrick;
    let hubspot = HubSpotBrick;
    let notion = NotionBrick;
    let odoo = OdooBrick;
    let n8n = N8nBrick;
    let field_mapping = FieldMappingBrick;
    let combine_text = CombineTextBrick;
    let conditional = ConditionalBrick;
    let rules_engine = RulesEngineBrick;
    
    let bricks = vec![
        BrickSchemaResponse {
            brick_type: BrickType::OpenAi,
            name: "openai".to_string(),
            config_schema: openai.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::Nvidia,
            name: "nvidia".to_string(),
            config_schema: nvidia.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::HubSpot,
            name: "hubspot".to_string(),
            config_schema: hubspot.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::Notion,
            name: "notion".to_string(),
            config_schema: notion.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::Odoo,
            name: "odoo".to_string(),
            config_schema: odoo.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::N8n,
            name: "n8n".to_string(),
            config_schema: n8n.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::FieldMapping,
            name: "field_mapping".to_string(),
            config_schema: field_mapping.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::CombineText,
            name: "combine_text".to_string(),
            config_schema: combine_text.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::Conditional,
            name: "conditional".to_string(),
            config_schema: conditional.config_schema(),
        },
        BrickSchemaResponse {
            brick_type: BrickType::RulesEngine,
            name: "rules_engine".to_string(),
            config_schema: rules_engine.config_schema(),
        },
    ];
    
    Json(BrickListResponse { bricks })
}

async fn get_brick_schema(
    axum::extract::Path(brick_type): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let schema = match brick_type.as_str() {
        "openai" => {
            let brick = OpenAiBrick;
            brick.config_schema()
        }
        "nvidia" => {
            let brick = NvidiaBrick;
            brick.config_schema()
        }
        "hubspot" => {
            let brick = HubSpotBrick;
            brick.config_schema()
        }
        "notion" => {
            let brick = NotionBrick;
            brick.config_schema()
        }
        "odoo" => {
            let brick = OdooBrick;
            brick.config_schema()
        }
        "n8n" => {
            let brick = N8nBrick;
            brick.config_schema()
        }
        "field_mapping" => {
            let brick = FieldMappingBrick;
            brick.config_schema()
        }
        "combine_text" => {
            let brick = CombineTextBrick;
            brick.config_schema()
        }
        "conditional" => {
            let brick = ConditionalBrick;
            brick.config_schema()
        }
        "rules_engine" => {
            let brick = RulesEngineBrick;
            brick.config_schema()
        }
        _ => return Err(axum::http::StatusCode::NOT_FOUND),
    };
    
    Ok(Json(schema))
}

