use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType};
use serde_json::{json, Value};

pub struct OpenAiBrick;

#[async_trait]
impl Brick for OpenAiBrick {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::OpenAi
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "OpenAI API key"
                },
                "model_name": {
                    "type": "string",
                    "description": "Model to use (e.g., gpt-4, gpt-3.5-turbo)",
                    "default": "gpt-3.5-turbo"
                },
                "prompt_template": {
                    "type": "string",
                    "description": "Prompt template with {{field}} placeholders"
                },
                "temperature": {
                    "type": "number",
                    "description": "Temperature for generation",
                    "default": 0.7
                },
                "max_tokens": {
                    "type": "number",
                    "description": "Maximum tokens to generate",
                    "default": 1000
                }
            },
            "required": ["api_key", "prompt_template"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let api_key = config
            .get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("api_key is required".to_string()))?;

        let model_name = config
            .get("model_name")
            .and_then(|v| v.as_str())
            .unwrap_or("gpt-3.5-turbo");

        let prompt_template = config
            .get("prompt_template")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("prompt_template is required".to_string()))?;

        let temperature = config
            .get("temperature")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.7);

        let max_tokens = config
            .get("max_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(1000);

        // Replace placeholders in prompt template
        let prompt = replace_placeholders(prompt_template, &input);

        // Call OpenAI API
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model_name,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "temperature": temperature,
                "max_tokens": max_tokens
            }))
            .send()
            .await
            .map_err(|e| BrickError::NetworkError(format!("OpenAI API error: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(BrickError::ExecutionError(format!(
                "OpenAI API returned error: {}",
                error_text
            )));
        }

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse response: {}", e)))?;

        // Extract the content from the response
        let content = response_json
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| BrickError::ExecutionError("Invalid response format".to_string()))?;

        // Parse token usage
        let token_usage = response_json
            .get("usage")
            .and_then(|u| u.get("total_tokens"))
            .and_then(|t| t.as_u64());

        // Return structured JSON
        Ok(json!({
            "content": content,
            "model": model_name,
            "token_usage": token_usage,
            "input": input
        }))
    }
}

fn replace_placeholders(template: &str, input: &Value) -> String {
    let mut result = template.to_string();
    
    if let Some(obj) = input.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
    }
    
    result
}

