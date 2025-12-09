use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

use crate::types::BrickType;

#[derive(Debug, Error)]
pub enum BrickError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait Brick: Send + Sync {
    /// Returns the name of the brick
    fn name(&self) -> &'static str;
    
    /// Returns the brick type
    fn brick_type(&self) -> BrickType;
    
    /// Returns the JSON schema for the brick's configuration
    fn config_schema(&self) -> Value;
    
    /// Validates the configuration against the schema
    fn validate_config(&self, config: &Value) -> Result<(), BrickError> {
        // Basic validation
        if !config.is_object() {
            return Err(BrickError::ConfigError("Config must be a JSON object".to_string()));
        }

        let schema = self.config_schema();
        
        // Check required fields
        if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
            for field in required {
                if let Some(field_name) = field.as_str() {
                    if !config.get(field_name).is_some() {
                        return Err(BrickError::ConfigError(
                            format!("Required field '{}' is missing", field_name)
                        ));
                    }
                }
            }
        }

        // Validate field types if properties are defined
        if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
            for (field_name, field_schema) in properties {
                if let Some(config_value) = config.get(field_name) {
                    if let Some(field_type) = field_schema.get("type").and_then(|v| v.as_str()) {
                        let type_matches = match field_type {
                            "string" => config_value.is_string(),
                            "number" => config_value.is_number(),
                            "integer" => config_value.is_i64() || config_value.is_u64(),
                            "boolean" => config_value.is_boolean(),
                            "array" => config_value.is_array(),
                            "object" => config_value.is_object(),
                            _ => true, // Unknown type, skip validation
                        };

                        if !type_matches {
                            return Err(BrickError::ConfigError(
                                format!("Field '{}' must be of type '{}'", field_name, field_type)
                            ));
                        }
                    }

                    // Validate enum values if present
                    if let Some(enum_values) = field_schema.get("enum").and_then(|v| v.as_array()) {
                        let value_str = config_value.as_str();
                        if let Some(val) = value_str {
                            let is_valid = enum_values.iter().any(|ev| {
                                ev.as_str().map(|s| s == val).unwrap_or(false)
                            });
                            if !is_valid {
                                return Err(BrickError::ConfigError(
                                    format!("Field '{}' must be one of: {:?}", field_name, 
                                        enum_values.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
    
    /// Executes the brick with the given input payload
    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError>;
}

#[derive(Debug, Clone)]
pub struct BrickExecutionResult {
    pub output: Value,
    pub metadata: Option<Value>,
    pub cost_unit: f64,
    pub token_usage: Option<u64>,
}

impl BrickExecutionResult {
    pub fn new(output: Value) -> Self {
        Self {
            output,
            metadata: None,
            cost_unit: 0.0,
            token_usage: None,
        }
    }
    
    pub fn with_cost(output: Value, cost_unit: f64) -> Self {
        Self {
            output,
            metadata: None,
            cost_unit,
            token_usage: None,
        }
    }
    
    pub fn with_tokens(output: Value, cost_unit: f64, token_usage: u64) -> Self {
        Self {
            output,
            metadata: None,
            cost_unit,
            token_usage: Some(token_usage),
        }
    }
}

