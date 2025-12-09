use anyhow::Result;
use flowmason_core::{FlowRunner, types::Flow};
use flowmason_bricks::*;
use serde_json::Value;

pub struct SyncExecutor;

impl SyncExecutor {
    /// Executes a flow synchronously
    pub async fn execute_flow(
        flow: &Flow,
        input: Value,
    ) -> Result<Value> {
        // Create brick instances
        let mut bricks: Vec<Box<dyn flowmason_core::Brick>> = Vec::new();
        
        for brick_config in &flow.bricks {
            let brick: Box<dyn flowmason_core::Brick> = match brick_config.brick_type {
                flowmason_core::types::BrickType::OpenAi => Box::new(OpenAiBrick),
                flowmason_core::types::BrickType::Nvidia => Box::new(NvidiaBrick),
                flowmason_core::types::BrickType::HubSpot => Box::new(HubSpotBrick),
                flowmason_core::types::BrickType::Notion => Box::new(NotionBrick),
                flowmason_core::types::BrickType::Odoo => Box::new(OdooBrick),
                flowmason_core::types::BrickType::N8n => Box::new(N8nBrick),
                flowmason_core::types::BrickType::FieldMapping => Box::new(FieldMappingBrick),
                flowmason_core::types::BrickType::CombineText => Box::new(CombineTextBrick),
                flowmason_core::types::BrickType::Conditional => Box::new(ConditionalBrick),
            };
            bricks.push(brick);
        }
        
        // Execute flow (without quota/usage tracking for worker)
        let configs: Vec<Value> = flow.bricks.iter().map(|b| b.config.clone()).collect();
        FlowRunner::execute_flow(bricks, configs, input, None)
            .await
            .map_err(|e| anyhow::anyhow!("Flow execution failed: {}", e))
    }
}

