use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType};
use serde_json::{json, Value};
use reqwest::Client;

pub struct HubSpotBrick;

#[async_trait]
impl Brick for HubSpotBrick {
    fn name(&self) -> &'static str {
        "hubspot"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::HubSpot
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "HubSpot API key"
                },
                "operation": {
                    "type": "string",
                    "enum": ["get_deals", "create_deal", "update_deal", "get_contacts"],
                    "description": "Operation to perform",
                    "default": "get_deals"
                }
            },
            "required": ["api_key", "operation"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let api_key = config
            .get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("api_key is required".to_string()))?;

        let operation = config
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("get_deals");

        match operation {
            "get_deals" => self.get_deals(api_key).await,
            "create_deal" => self.create_deal(api_key, input).await,
            "update_deal" => self.update_deal(api_key, input).await,
            "get_contacts" => self.get_contacts(api_key).await,
            _ => Err(BrickError::ConfigError(format!("Unknown operation: {}", operation))),
        }
    }
}

impl HubSpotBrick {
    async fn get_deals(&self, api_key: &str) -> Result<Value, BrickError> {
        let client = Client::new();
        let url = "https://api.hubapi.com/crm/v3/objects/deals";
        
        let response = client
            .get(url)
            .query(&[("hapikey", api_key)])
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to HubSpot API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "HubSpot API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse HubSpot response: {}", e)))?;

        Ok(data)
    }

    async fn create_deal(&self, api_key: &str, input: Value) -> Result<Value, BrickError> {
        let client = Client::new();
        let url = "https://api.hubapi.com/crm/v3/objects/deals";
        
        // Extract properties from input or use input directly as properties
        let properties = if input.get("properties").is_some() {
            input["properties"].clone()
        } else {
            input.clone()
        };

        let payload = json!({
            "properties": properties
        });

        let response = client
            .post(url)
            .query(&[("hapikey", api_key)])
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to HubSpot API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "HubSpot API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse HubSpot response: {}", e)))?;

        Ok(data)
    }

    async fn update_deal(&self, api_key: &str, input: Value) -> Result<Value, BrickError> {
        let deal_id = input
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("Deal ID is required in input".to_string()))?;

        let client = Client::new();
        let url = format!("https://api.hubapi.com/crm/v3/objects/deals/{}", deal_id);
        
        // Extract properties from input
        let properties = if input.get("properties").is_some() {
            input["properties"].clone()
        } else {
            // Remove id from properties if present
            let mut props = input.clone();
            props.as_object_mut().map(|m| m.remove("id"));
            props
        };

        let payload = json!({
            "properties": properties
        });

        let response = client
            .patch(&url)
            .query(&[("hapikey", api_key)])
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to HubSpot API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "HubSpot API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse HubSpot response: {}", e)))?;

        Ok(data)
    }

    async fn get_contacts(&self, api_key: &str) -> Result<Value, BrickError> {
        let client = Client::new();
        let url = "https://api.hubapi.com/crm/v3/objects/contacts";
        
        let response = client
            .get(url)
            .query(&[("hapikey", api_key)])
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to HubSpot API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "HubSpot API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse HubSpot response: {}", e)))?;

        Ok(data)
    }
}

