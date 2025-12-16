use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType, Mapper, MappingRule, MappingDirection, MergeStrategy, SplitStrategy};
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
                                "description": "Single source path (legacy format, e.g., 'user.name')"
                            },
                            "target_path": {
                                "type": "string",
                                "description": "Single target path (legacy format, e.g., 'customer_name')"
                            },
                            "source_paths": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                },
                                "description": "Multiple source paths for multi-directional mapping"
                            },
                            "target_paths": {
                                "type": "array",
                                "items": {
                                    "type": "string"
                                },
                                "description": "Multiple target paths for multi-directional mapping"
                            },
                            "direction": {
                                "type": "string",
                                "enum": ["forward", "backward", "bidirectional"],
                                "default": "forward",
                                "description": "Mapping direction"
                            },
                            "merge_strategy": {
                                "type": "string",
                                "enum": ["concat", "merge_object", "array", "first", "last"],
                                "description": "Strategy for merging multiple sources to one target"
                            },
                            "split_strategy": {
                                "type": "string",
                                "enum": ["copy", "extract", "transform_each"],
                                "description": "Strategy for splitting one source to multiple targets"
                            }
                        }
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
            // Support both legacy (single path) and new (multi-path) formats
            let source_paths: Vec<String> = if let Some(paths) = mapping.get("source_paths").and_then(|v| v.as_array()) {
                paths.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            } else if let Some(path) = mapping.get("source_path").and_then(|v| v.as_str()) {
                vec![path.to_string()]
            } else {
                return Err(BrickError::ConfigError("Either source_path or source_paths is required".to_string()));
            };

            let target_paths: Vec<String> = if let Some(paths) = mapping.get("target_paths").and_then(|v| v.as_array()) {
                paths.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            } else if let Some(path) = mapping.get("target_path").and_then(|v| v.as_str()) {
                vec![path.to_string()]
            } else {
                return Err(BrickError::ConfigError("Either target_path or target_paths is required".to_string()));
            };

            let direction = mapping
                .get("direction")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "backward" => MappingDirection::Backward,
                    "bidirectional" => MappingDirection::Bidirectional,
                    _ => MappingDirection::Forward,
                })
                .unwrap_or(MappingDirection::Forward);

            let merge_strategy = mapping
                .get("merge_strategy")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "concat" => MergeStrategy::Concat,
                    "merge_object" => MergeStrategy::MergeObject,
                    "array" => MergeStrategy::Array,
                    "first" => MergeStrategy::First,
                    "last" => MergeStrategy::Last,
                    _ => MergeStrategy::Concat,
                });

            let split_strategy = mapping
                .get("split_strategy")
                .and_then(|v| v.as_str())
                .map(|s| match s {
                    "extract" => SplitStrategy::Extract,
                    "transform_each" => SplitStrategy::TransformEach,
                    _ => SplitStrategy::Copy,
                });

            rules.push(MappingRule {
                source_path: source_paths.first().cloned().unwrap_or_default(),
                target_path: target_paths.first().cloned().unwrap_or_default(),
                source_paths,
                target_paths,
                direction,
                transform: None,
                merge_strategy,
                split_strategy,
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


