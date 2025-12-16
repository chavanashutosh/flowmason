use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType};
use serde_json::{json, Value};
use crate::http_client::{get_client, execute_with_default_retry};

pub struct NvidiaBrick;

#[derive(Debug, Clone)]
pub enum NvidiaEndpoint {
    Asr,  // Speech to text
    Ocr,  // Optical character recognition
    TextGeneration,
}

#[async_trait]
impl Brick for NvidiaBrick {
    fn name(&self) -> &'static str {
        "nvidia"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::Nvidia
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "NVIDIA API key"
                },
                "endpoint": {
                    "type": "string",
                    "enum": ["asr", "ocr", "text_generation"],
                    "description": "NVIDIA endpoint to use"
                },
                "model": {
                    "type": "string",
                    "description": "Model name (optional)"
                }
            },
            "required": ["api_key", "endpoint"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let api_key = config
            .get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("api_key is required".to_string()))?;

        let endpoint_str = config
            .get("endpoint")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("endpoint is required".to_string()))?;

        let endpoint = match endpoint_str {
            "asr" => NvidiaEndpoint::Asr,
            "ocr" => NvidiaEndpoint::Ocr,
            "text_generation" => NvidiaEndpoint::TextGeneration,
            _ => {
                return Err(BrickError::ConfigError(format!(
                    "Invalid endpoint: {}. Must be one of: asr, ocr, text_generation",
                    endpoint_str
                )));
            }
        };

        match endpoint {
            NvidiaEndpoint::Asr => self.execute_asr(api_key, input).await,
            NvidiaEndpoint::Ocr => self.execute_ocr(api_key, input).await,
            NvidiaEndpoint::TextGeneration => self.execute_text_generation(api_key, input, config.clone()).await,
        }
    }
}

impl NvidiaBrick {
    async fn execute_asr(&self, api_key: &str, input: Value) -> Result<Value, BrickError> {
        // Extract audio URL or base64 data
        let audio_data = input
            .get("audio_url")
            .or_else(|| input.get("audio_base64"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::InvalidInput("audio_url or audio_base64 is required".to_string()))?;

        // Call NVIDIA ASR API using shared HTTP client with retry logic
        let client = get_client();
        let response = execute_with_default_retry(
            client
                .post("https://api.nvidia.com/v1/speech/asr")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "audio": audio_data
                }))
        )
        .await
        .map_err(|e| BrickError::NetworkError(format!("NVIDIA ASR API error: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| BrickError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(BrickError::ExecutionError(format!(
                "NVIDIA ASR API returned error: {}",
                error_text
            )));
        }

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse response: {}", e)))?;

        Ok(json!({
            "text": response_json.get("text").unwrap_or(&json!(null)),
            "confidence": response_json.get("confidence").unwrap_or(&json!(null)),
            "input": input
        }))
    }

    async fn execute_ocr(&self, api_key: &str, input: Value) -> Result<Value, BrickError> {
        // Extract image URL or base64 data
        let image_data = input
            .get("image_url")
            .or_else(|| input.get("image_base64"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::InvalidInput("image_url or image_base64 is required".to_string()))?;

        // Call NVIDIA OCR API using shared HTTP client
        let client = get_client();
        let response = execute_with_default_retry(
            client
                .post("https://api.nvidia.com/v1/vision/ocr")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "image": image_data
                }))
        )
        .await
        .map_err(|e| BrickError::NetworkError(format!("NVIDIA OCR API error: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| BrickError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(BrickError::ExecutionError(format!(
                "NVIDIA OCR API returned error: {}",
                error_text
            )));
        }

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse response: {}", e)))?;

        Ok(json!({
            "text": response_json.get("text").unwrap_or(&json!(null)),
            "bounding_boxes": response_json.get("bounding_boxes").unwrap_or(&json!([])),
            "input": input
        }))
    }

    async fn execute_text_generation(
        &self,
        api_key: &str,
        input: Value,
        config: Value,
    ) -> Result<Value, BrickError> {
        let model = config
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("nvidia/meta/llama-3-8b-instruct");

        let prompt = input
            .get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::InvalidInput("prompt is required".to_string()))?;

        // Call NVIDIA Text Generation API using shared HTTP client
        let client = get_client();
        let response = execute_with_default_retry(
            client
                .post("https://api.nvidia.com/v1/text/generation")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "model": model,
                    "prompt": prompt,
                    "max_tokens": 1000
                }))
        )
        .await
        .map_err(|e| BrickError::NetworkError(format!("NVIDIA Text Generation API error: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| BrickError::NetworkError(format!("Failed to read error response: {}", e)))?;
            return Err(BrickError::ExecutionError(format!(
                "NVIDIA Text Generation API returned error: {}",
                error_text
            )));
        }

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to parse response: {}", e)))?;

        Ok(json!({
            "text": response_json.get("text").unwrap_or(&json!(null)),
            "model": model,
            "input": input
        }))
    }
}

