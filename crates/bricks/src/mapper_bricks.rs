use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType, Mapper, MappingRule};
use serde_json::{json, Value};

pub struct FieldMappingBrick;

#[async_trait]
impl Brick for FieldMappingBrick {
    fn name(&self) -> &'static str {
        "field_mapping"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::FieldMapping
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "mappings": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "source_path": {
                                "type": "string",
                                "description": "Path to source field (e.g., 'user.name')"
                            },
                            "target_path": {
                                "type": "string",
                                "description": "Path to target field (e.g., 'customer_name')"
                            }
                        },
                        "required": ["source_path", "target_path"]
                    }
                }
            },
            "required": ["mappings"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let mappings_array = config
            .get("mappings")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BrickError::ConfigError("mappings array is required".to_string()))?;

        let mut rules = Vec::new();
        for mapping in mappings_array {
            let source_path = mapping
                .get("source_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BrickError::ConfigError("source_path is required".to_string()))?
                .to_string();
            let target_path = mapping
                .get("target_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BrickError::ConfigError("target_path is required".to_string()))?
                .to_string();

            rules.push(MappingRule {
                source_path,
                target_path,
                transform: None,
            });
        }

        Mapper::apply_mappings(&input, &rules)
            .map_err(|e| BrickError::ExecutionError(format!("Mapping error: {}", e)))
    }
}

pub struct CombineTextBrick;

#[async_trait]
impl Brick for CombineTextBrick {
    fn name(&self) -> &'static str {
        "combine_text"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::CombineText
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "fields": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Array of field paths to combine"
                },
                "separator": {
                    "type": "string",
                    "description": "Separator to use when combining",
                    "default": " "
                },
                "output_field": {
                    "type": "string",
                    "description": "Field name for the combined result",
                    "default": "combined_text"
                }
            },
            "required": ["fields", "output_field"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let fields = config
            .get("fields")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BrickError::ConfigError("fields array is required".to_string()))?;

        let separator = config
            .get("separator")
            .and_then(|v| v.as_str())
            .unwrap_or(" ");

        let output_field = config
            .get("output_field")
            .and_then(|v| v.as_str())
            .unwrap_or("combined_text");

        let mut values = Vec::new();
        for field_path in fields {
            let path = field_path
                .as_str()
                .ok_or_else(|| BrickError::ConfigError("Field path must be a string".to_string()))?;
            
            let value = Mapper::get_value_at_path(&input, path)
                .map_err(|e| BrickError::InvalidInput(format!("Failed to get field {}: {}", path, e)))?;
            
            let str_value = value.as_str()
                .unwrap_or(&value.to_string())
                .to_string();
            values.push(str_value);
        }

        let combined = values.join(separator);
        let mut output = input.clone();
        if let Some(obj) = output.as_object_mut() {
            obj.insert(output_field.to_string(), Value::String(combined));
        }

        Ok(output)
    }
}

pub struct ConditionalBrick;

#[async_trait]
impl Brick for ConditionalBrick {
    fn name(&self) -> &'static str {
        "conditional"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::Conditional
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "condition_field": {
                    "type": "string",
                    "description": "Field path to evaluate condition on"
                },
                "condition": {
                    "type": "string",
                    "description": "Condition expression (e.g., '> 1000', \"== 'VIP'\")"
                },
                "true_value": {
                    "description": "Value to set if condition is true"
                },
                "false_value": {
                    "description": "Value to set if condition is false"
                },
                "output_field": {
                    "type": "string",
                    "description": "Field name for the result",
                    "default": "status"
                }
            },
            "required": ["condition_field", "condition", "true_value", "false_value", "output_field"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let condition_field = config
            .get("condition_field")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("condition_field is required".to_string()))?;

        let condition = config
            .get("condition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("condition is required".to_string()))?;

        let true_value = config
            .get("true_value")
            .ok_or_else(|| BrickError::ConfigError("true_value is required".to_string()))?
            .clone();

        let false_value = config
            .get("false_value")
            .ok_or_else(|| BrickError::ConfigError("false_value is required".to_string()))?
            .clone();

        let output_field = config
            .get("output_field")
            .and_then(|v| v.as_str())
            .unwrap_or("status");

        let field_value = Mapper::get_value_at_path(&input, condition_field)
            .map_err(|e| BrickError::InvalidInput(format!("Failed to get field {}: {}", condition_field, e)))?;

        let condition_met = Mapper::evaluate_condition(&field_value, condition)
            .map_err(|e| BrickError::ExecutionError(format!("Condition evaluation error: {}", e)))?;

        let result_value = if condition_met { true_value } else { false_value };
        let mut output = input.clone();
        if let Some(obj) = output.as_object_mut() {
            obj.insert(output_field.to_string(), result_value);
        }

        Ok(output)
    }
}


