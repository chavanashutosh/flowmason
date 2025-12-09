use serde_json::{json, Value};
use thiserror::Error;

use crate::types::{MappingRule, TransformType};

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Value not found at path: {0}")]
    ValueNotFound(String),
    
    #[error("Type mismatch: {0}")]
    TypeMismatch(String),
    
    #[error("Transform error: {0}")]
    TransformError(String),
}

pub struct Mapper;

impl Mapper {
    /// Maps a value from source path to target path in the output JSON
    pub fn map_field(
        source: &Value,
        target: &mut Value,
        rule: &MappingRule,
    ) -> Result<(), MappingError> {
        let source_value = Self::get_value_at_path(source, &rule.source_path)?;
        let transformed_value = if let Some(ref transform) = rule.transform {
            Self::apply_transform(source_value, transform)?
        } else {
            source_value
        };
        Self::set_value_at_path(target, &rule.target_path, transformed_value)?;
        Ok(())
    }

    /// Applies multiple mapping rules to transform input to output
    pub fn apply_mappings(
        input: &Value,
        rules: &[MappingRule],
    ) -> Result<Value, MappingError> {
        let mut output = json!({});
        
        for rule in rules {
            Self::map_field(input, &mut output, rule)?;
        }
        
        Ok(output)
    }

    /// Gets a value from a JSON path (e.g., "user.name" or "items[0].title")
    pub fn get_value_at_path(value: &Value, path: &str) -> Result<Value, MappingError> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for part in parts {
            if part.contains('[') {
                // Handle array access like "items[0]"
                let bracket_pos = part.find('[').ok_or_else(|| {
                    MappingError::InvalidPath(format!("Invalid array syntax in path: {}", path))
                })?;
                let key = &part[..bracket_pos];
                let index_str = &part[bracket_pos + 1..part.len() - 1];
                let index: usize = index_str.parse().map_err(|_| {
                    MappingError::InvalidPath(format!("Invalid array index: {}", index_str))
                })?;

                current = current
                    .get(key)
                    .ok_or_else(|| MappingError::ValueNotFound(format!("Key not found: {}", key)))?
                    .get(index)
                    .ok_or_else(|| {
                        MappingError::ValueNotFound(format!("Index {} not found in array", index))
                    })?;
            } else {
                current = current
                    .get(part)
                    .ok_or_else(|| MappingError::ValueNotFound(format!("Path not found: {}", path)))?;
            }
        }

        Ok(current.clone())
    }

    /// Sets a value at a JSON path
    fn set_value_at_path(
        value: &mut Value,
        path: &str,
        new_value: Value,
    ) -> Result<(), MappingError> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part - set the value
                if let Some(obj) = current.as_object_mut() {
                    obj.insert(part.to_string(), new_value.clone());
                } else {
                    return Err(MappingError::TypeMismatch(
                        "Target path does not point to an object".to_string(),
                    ));
                }
            } else {
                // Navigate through the path
                if !current.get(part).is_some() {
                    if let Some(obj) = current.as_object_mut() {
                        obj.insert(part.to_string(), json!({}));
                    }
                }
                current = current
                    .get_mut(part)
                    .ok_or_else(|| MappingError::InvalidPath(format!("Path not found: {}", path)))?;
            }
        }

        Ok(())
    }

    /// Applies a transform to a value
    fn apply_transform(value: Value, transform: &TransformType) -> Result<Value, MappingError> {
        match transform {
            TransformType::StringConcat { separator } => {
                if let Value::Array(arr) = value {
                    let strings: Vec<String> = arr
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect();
                    Ok(Value::String(strings.join(separator)))
                } else {
                    Err(MappingError::TypeMismatch(
                        "StringConcat requires an array of strings".to_string(),
                    ))
                }
            }
            TransformType::NumberAdd => {
                if let Value::Number(n) = value {
                    Ok(Value::Number(n.clone()))
                } else {
                    Err(MappingError::TypeMismatch("NumberAdd requires a number".to_string()))
                }
            }
            TransformType::NumberMultiply => {
                if let Value::Number(n) = value {
                    Ok(Value::Number(n.clone()))
                } else {
                    Err(MappingError::TypeMismatch(
                        "NumberMultiply requires a number".to_string(),
                    ))
                }
            }
            TransformType::StringToUpper => {
                if let Value::String(s) = value {
                    Ok(Value::String(s.to_uppercase()))
                } else {
                    Err(MappingError::TypeMismatch("StringToUpper requires a string".to_string()))
                }
            }
            TransformType::StringToLower => {
                if let Value::String(s) = value {
                    Ok(Value::String(s.to_lowercase()))
                } else {
                    Err(MappingError::TypeMismatch("StringToLower requires a string".to_string()))
                }
            }
            TransformType::Conditional {
                condition,
                true_value,
                false_value,
            } => {
                // Simple condition evaluation - can be enhanced
                let condition_met = Self::evaluate_condition(&value, condition)?;
                Ok(if condition_met {
                    true_value.clone()
                } else {
                    false_value.clone()
                })
            }
        }
    }

    /// Evaluates a simple condition (e.g., "> 1000", "== 'VIP'")
    pub fn evaluate_condition(value: &Value, condition: &str) -> Result<bool, MappingError> {
        if condition.starts_with("> ") {
            if let (Some(num), Some(threshold)) = (
                value.as_f64(),
                condition[2..].trim().parse::<f64>().ok(),
            ) {
                Ok(num > threshold)
            } else {
                Err(MappingError::TransformError(
                    "Invalid numeric condition".to_string(),
                ))
            }
        } else if condition.starts_with("< ") {
            if let (Some(num), Some(threshold)) = (
                value.as_f64(),
                condition[2..].trim().parse::<f64>().ok(),
            ) {
                Ok(num < threshold)
            } else {
                Err(MappingError::TransformError(
                    "Invalid numeric condition".to_string(),
                ))
            }
        } else if condition.starts_with("== ") {
            let expected = condition[3..].trim().trim_matches('"').trim_matches('\'');
            Ok(value.as_str().map(|s| s == expected).unwrap_or(false))
        } else {
            Err(MappingError::TransformError(format!(
                "Unsupported condition: {}",
                condition
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_map_field() {
        let input = json!({
            "user": {
                "name": "John Doe",
                "age": 30
            }
        });
        let mut output = json!({});
        let rule = MappingRule {
            source_path: "user.name".to_string(),
            target_path: "customer_name".to_string(),
            transform: None,
        };

        Mapper::map_field(&input, &mut output, &rule).unwrap();
        assert_eq!(output["customer_name"], "John Doe");
    }

    #[test]
    fn test_combine_text() {
        let input = json!({
            "first": "John",
            "last": "Doe"
        });
        let mut output = json!({});
        let rule = MappingRule {
            source_path: "first".to_string(),
            target_path: "full_name".to_string(),
            transform: Some(TransformType::StringConcat {
                separator: " ".to_string(),
            }),
        };

        // This would need array input for concat, but demonstrates the concept
        let array_input = json!(["John", "Doe"]);
        let result = Mapper::apply_transform(
            array_input,
            &TransformType::StringConcat {
                separator: " ".to_string(),
            },
        )
        .unwrap();
        assert_eq!(result, json!("John Doe"));
    }
}

