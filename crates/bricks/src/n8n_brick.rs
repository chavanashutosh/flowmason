use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType};
use serde_json::{json, Value};

pub struct N8nBrick;

#[async_trait]
impl Brick for N8nBrick {
    fn name(&self) -> &'static str {
        "n8n"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::N8n
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "webhook_url": {
                    "type": "string",
                    "description": "n8n webhook URL"
                },
                "method": {
                    "type": "string",
                    "enum": ["POST", "GET", "PUT"],
                    "description": "HTTP method",
                    "default": "POST"
                }
            },
            "required": ["webhook_url"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let webhook_url = config
            .get("webhook_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("webhook_url is required".to_string()))?;

        let method = config
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("POST");

        let client = reqwest::Client::new();
        let request = match method {
            "POST" => client.post(webhook_url),
            "GET" => client.get(webhook_url),
            "PUT" => client.put(webhook_url),
            _ => return Err(BrickError::ConfigError(format!("Unsupported method: {}", method))),
        };

        let response = request
            .json(&input)
            .send()
            .await
            .map_err(|e| BrickError::NetworkError(format!("n8n webhook error: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(BrickError::ExecutionError(format!(
                "n8n webhook returned error: {}",
                error_text
            )));
        }

        let response_json: Value = response
            .json()
            .await
            .unwrap_or_else(|_| json!({"status": "success", "input": input}));

        Ok(response_json)
    }
}

