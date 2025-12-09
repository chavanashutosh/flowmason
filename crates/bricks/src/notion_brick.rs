use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType};
use serde_json::{json, Value};
use reqwest::Client;

pub struct NotionBrick;

#[async_trait]
impl Brick for NotionBrick {
    fn name(&self) -> &'static str {
        "notion"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::Notion
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "Notion API key"
                },
                "database_id": {
                    "type": "string",
                    "description": "Notion database ID"
                },
                "operation": {
                    "type": "string",
                    "enum": ["get_pages", "create_page", "update_page"],
                    "description": "Operation to perform",
                    "default": "get_pages"
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
            .unwrap_or("get_pages");

        match operation {
            "get_pages" => self.get_pages(api_key, config.clone()).await,
            "create_page" => self.create_page(api_key, input, config.clone()).await,
            "update_page" => self.update_page(api_key, input, config.clone()).await,
            _ => Err(BrickError::ConfigError(format!("Unknown operation: {}", operation))),
        }
    }
}

impl NotionBrick {
    async fn get_pages(&self, api_key: &str, config: Value) -> Result<Value, BrickError> {
        let database_id = config
            .get("database_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("database_id is required".to_string()))?;

        let client = Client::new();
        let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&json!({}))
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to Notion API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "Notion API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse Notion response: {}", e)))?;

        Ok(data)
    }

    async fn create_page(&self, api_key: &str, input: Value, config: Value) -> Result<Value, BrickError> {
        let database_id = config
            .get("database_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("database_id is required".to_string()))?;

        let client = Client::new();
        let url = "https://api.notion.com/v1/pages";
        
        // Build page properties from input
        let properties = if input.get("properties").is_some() {
            input["properties"].clone()
        } else {
            // Try to extract title from input
            let title = input.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("New Page");
            json!({
                "title": {
                    "title": [
                        {
                            "text": {
                                "content": title
                            }
                        }
                    ]
                }
            })
        };

        let payload = json!({
            "parent": {
                "database_id": database_id
            },
            "properties": properties
        });

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to Notion API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "Notion API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse Notion response: {}", e)))?;

        Ok(data)
    }

    async fn update_page(&self, api_key: &str, input: Value, _config: Value) -> Result<Value, BrickError> {
        let page_id = input
            .get("page_id")
            .or_else(|| input.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("page_id is required in input".to_string()))?;

        let client = Client::new();
        let url = format!("https://api.notion.com/v1/pages/{}", page_id);
        
        // Extract properties from input
        let properties = if input.get("properties").is_some() {
            input["properties"].clone()
        } else {
            // Remove id/page_id from properties if present
            let mut props = input.clone();
            props.as_object_mut().map(|m| {
                m.remove("id");
                m.remove("page_id");
            });
            props
        };

        let payload = json!({
            "properties": properties
        });

        let response = client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to Notion API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "Notion API error ({}): {}",
                status, error_text
            )));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse Notion response: {}", e)))?;

        Ok(data)
    }
}

