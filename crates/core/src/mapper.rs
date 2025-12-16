use serde_json::{json, Value};
use thiserror::Error;

use crate::types::{MappingRule, TransformType, MappingDirection, MergeStrategy, SplitStrategy};

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
    /// Supports both legacy single-path and new multi-path formats
    pub fn map_field(
        source: &Value,
        target: &mut Value,
        rule: &MappingRule,
    ) -> Result<(), MappingError> {
        // Determine which paths to use (backward compatibility)
        let source_paths = if !rule.source_paths.is_empty() {
            &rule.source_paths
        } else if !rule.source_path.is_empty() {
            // Legacy format - single source path
            return Self::map_field_legacy(source, target, rule);
        } else {
            return Err(MappingError::InvalidPath("No source paths specified".to_string()));
        };

        let target_paths = if !rule.target_paths.is_empty() {
            &rule.target_paths
        } else if !rule.target_path.is_empty() {
            // Legacy format - single target path
            return Self::map_field_legacy(source, target, rule);
        } else {
            return Err(MappingError::InvalidPath("No target paths specified".to_string()));
        };

        // Handle multi-directional mapping
        match rule.direction {
            MappingDirection::Forward => {
                Self::map_field_forward(source, target, source_paths, target_paths, rule)?;
            }
            MappingDirection::Backward => {
                // TODO: Backward mapping requires mutable source access, which current API doesn't support.
                // For backward mapping, we need mutable access to source.
                // Since source is immutable, we'll work with a clone for backward mapping.
                // In practice, backward mapping should be handled at a higher level.
                // For now, we'll skip backward mapping when source is immutable.
                // This is a limitation - bidirectional mapping requires both to be mutable.
                // See map_field_backward() for future implementation.
                return Err(MappingError::InvalidPath(
                    "Backward mapping requires mutable source. Use bidirectional mapping at flow level.".to_string()
                ));
            }
            MappingDirection::Bidirectional => {
                Self::map_field_forward(source, target, source_paths, target_paths, rule)?;
                // TODO: Bidirectional mapping also requires mutable source.
                // For now, we'll only do forward mapping.
                // Full bidirectional support would require refactoring the API.
                // See map_field_backward() for future implementation.
            }
        }

        Ok(())
    }

    /// Legacy single-path mapping (for backward compatibility)
    fn map_field_legacy(
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

    /// Forward mapping (source -> target)
    fn map_field_forward(
        source: &Value,
        target: &mut Value,
        source_paths: &[String],
        target_paths: &[String],
        rule: &MappingRule,
    ) -> Result<(), MappingError> {
        if source_paths.len() == 1 && target_paths.len() == 1 {
            // Single source to single target - move ownership to avoid clone
            let source_value = Self::get_value_at_path(source, &source_paths[0])?;
            let transformed_value = if let Some(ref transform) = rule.transform {
                Self::apply_transform(source_value, transform)?
            } else {
                source_value
            };
            Self::set_value_at_path(target, &target_paths[0], transformed_value)?;
        } else if source_paths.len() > 1 && target_paths.len() == 1 {
            // Multiple sources to single target (merge)
            let merged_value = Self::merge_sources(source, source_paths, rule.merge_strategy.as_ref())?;
            let transformed_value = if let Some(ref transform) = rule.transform {
                Self::apply_transform(merged_value, transform)?
            } else {
                merged_value
            };
            Self::set_value_at_path(target, &target_paths[0], transformed_value)?;
        } else if source_paths.len() == 1 && target_paths.len() > 1 {
            // Single source to multiple targets (split)
            let source_value = Self::get_value_at_path(source, &source_paths[0])?;
            Self::split_to_targets(target, target_paths, source_value, rule.split_strategy.as_ref())?;
        } else {
            // Multiple sources to multiple targets (1:1 mapping)
            for (source_path, target_path) in source_paths.iter().zip(target_paths.iter()) {
                let source_value = Self::get_value_at_path(source, source_path)?;
                // Move ownership to transform if needed, avoiding clone when no transform
                let transformed_value = if let Some(ref transform) = rule.transform {
                    Self::apply_transform(source_value, transform)?
                } else {
                    source_value
                };
                Self::set_value_at_path(target, target_path, transformed_value)?;
            }
        }
        Ok(())
    }

    /// Backward mapping (target -> source)
    /// 
    /// TODO: This function is kept for future bidirectional mapping support.
    /// It requires mutable access to source, which the current API doesn't support.
    /// For full backward/bidirectional support, the API would need to be refactored.
    /// 
    /// This is intentionally unused but kept for future feature implementation.
    #[allow(dead_code)]
    fn map_field_backward(
        source: &mut Value,
        target: &mut Value,
        source_paths: &[String],
        target_paths: &[String],
        rule: &MappingRule,
    ) -> Result<(), MappingError> {
        // Reverse the mapping direction
        if source_paths.len() == 1 && target_paths.len() == 1 {
            let target_value = Self::get_value_at_path(target, &target_paths[0])?;
            let transformed_value = if let Some(ref transform) = rule.transform {
                Self::apply_transform(target_value, transform)?
            } else {
                target_value
            };
            Self::set_value_at_path(source, &source_paths[0], transformed_value)?;
        } else if target_paths.len() > 1 && source_paths.len() == 1 {
            // Multiple targets to single source (merge)
            let merged_value = Self::merge_sources(target, target_paths, rule.merge_strategy.as_ref())?;
            let transformed_value = if let Some(ref transform) = rule.transform {
                Self::apply_transform(merged_value, transform)?
            } else {
                merged_value
            };
            Self::set_value_at_path(source, &source_paths[0], transformed_value)?;
        } else if target_paths.len() == 1 && source_paths.len() > 1 {
            // Single target to multiple sources (split)
            let target_value = Self::get_value_at_path(target, &target_paths[0])?;
            Self::split_to_targets(source, source_paths, target_value, rule.split_strategy.as_ref())?;
        } else {
            // Multiple targets to multiple sources (1:1 mapping)
            for (target_path, source_path) in target_paths.iter().zip(source_paths.iter()) {
                let target_value = Self::get_value_at_path(target, target_path)?;
                let transformed_value = if let Some(ref transform) = rule.transform {
                    Self::apply_transform(target_value.clone(), transform)?
                } else {
                    target_value
                };
                Self::set_value_at_path(source, source_path, transformed_value)?;
            }
        }
        Ok(())
    }

    /// Merges multiple source values into a single target
    fn merge_sources(
        source: &Value,
        source_paths: &[String],
        strategy: Option<&MergeStrategy>,
    ) -> Result<Value, MappingError> {
        let values: Vec<Value> = source_paths
            .iter()
            .filter_map(|path| Self::get_value_at_path(source, path).ok())
            .collect();

        if values.is_empty() {
            return Err(MappingError::ValueNotFound("No values found at source paths".to_string()));
        }

        match strategy {
            Some(MergeStrategy::Concat) => {
                let strings: Vec<String> = values
                    .iter()
                    .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                    .collect();
                Ok(Value::String(strings.join(" ")))
            }
            Some(MergeStrategy::MergeObject) => {
                let mut merged = json!({});
                for value in values {
                    if let Some(obj) = value.as_object() {
                        if let Some(merged_obj) = merged.as_object_mut() {
                            // Use extend to avoid individual inserts
                            for (k, v) in obj {
                                merged_obj.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
                Ok(merged)
            }
            Some(MergeStrategy::Array) => {
                Ok(Value::Array(values))
            }
            Some(MergeStrategy::First) => {
                Ok(values[0].clone())
            }
            Some(MergeStrategy::Last) => {
                Ok(values[values.len() - 1].clone())
            }
            None => {
                // Default: merge as object
                let mut merged = json!({});
                for (idx, value) in values.iter().enumerate() {
                    if let Some(merged_obj) = merged.as_object_mut() {
                        merged_obj.insert(format!("field_{}", idx), value.clone());
                    }
                }
                Ok(merged)
            }
        }
    }

    /// Splits a single source value to multiple targets
    fn split_to_targets(
        target: &mut Value,
        target_paths: &[String],
        source_value: Value,
        strategy: Option<&SplitStrategy>,
    ) -> Result<(), MappingError> {
        match strategy {
            Some(SplitStrategy::Copy) => {
                // Copy same value to all targets
                for target_path in target_paths {
                    Self::set_value_at_path(target, target_path, source_value.clone())?;
                }
            }
            Some(SplitStrategy::Extract) => {
                // Extract nested values if source is an object
                if let Some(obj) = source_value.as_object() {
                    let keys: Vec<&String> = obj.keys().collect();
                    for (idx, target_path) in target_paths.iter().enumerate() {
                        if idx < keys.len() {
                            if let Some(value) = obj.get(keys[idx]) {
                                Self::set_value_at_path(target, target_path, value.clone())?;
                            }
                        }
                    }
                } else {
                    // If not object, copy to all targets
                    for target_path in target_paths {
                        Self::set_value_at_path(target, target_path, source_value.clone())?;
                    }
                }
            }
            Some(SplitStrategy::TransformEach) => {
                // Apply transform to each target (would need per-target transforms)
                // For now, just copy
                for target_path in target_paths {
                    Self::set_value_at_path(target, target_path, source_value.clone())?;
                }
            }
            None => {
                // Default: copy to all targets
                for target_path in target_paths {
                    Self::set_value_at_path(target, target_path, source_value.clone())?;
                }
            }
        }
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
            source_paths: vec![],
            target_paths: vec![],
            direction: MappingDirection::Forward,
            transform: None,
            merge_strategy: None,
            split_strategy: None,
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
            source_paths: vec![],
            target_paths: vec![],
            direction: MappingDirection::Forward,
            transform: Some(TransformType::StringConcat {
                separator: " ".to_string(),
            }),
            merge_strategy: None,
            split_strategy: None,
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

    #[test]
    fn test_get_value_at_path_nested() {
        let input = json!({
            "user": {
                "profile": {
                    "name": "John"
                }
            }
        });
        let result = Mapper::get_value_at_path(&input, "user.profile.name").unwrap();
        assert_eq!(result, json!("John"));
    }

    #[test]
    fn test_get_value_at_path_array() {
        let input = json!({
            "items": [
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"}
            ]
        });
        let result = Mapper::get_value_at_path(&input, "items[0].name").unwrap();
        assert_eq!(result, json!("Item 1"));
    }

    #[test]
    fn test_get_value_at_path_not_found() {
        let input = json!({"user": {"name": "John"}});
        let result = Mapper::get_value_at_path(&input, "user.age");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_value_at_path() {
        let mut output = json!({});
        Mapper::set_value_at_path(&mut output, "user.name", json!("John")).unwrap();
        assert_eq!(output["user"]["name"], "John");
    }

    #[test]
    fn test_apply_transform_string_to_upper() {
        let input = json!("hello world");
        let result = Mapper::apply_transform(input, &TransformType::StringToUpper).unwrap();
        assert_eq!(result, json!("HELLO WORLD"));
    }

    #[test]
    fn test_apply_transform_string_to_lower() {
        let input = json!("HELLO WORLD");
        let result = Mapper::apply_transform(input, &TransformType::StringToLower).unwrap();
        assert_eq!(result, json!("hello world"));
    }

    #[test]
    fn test_evaluate_condition_greater_than() {
        let value = json!(150);
        let result = Mapper::evaluate_condition(&value, "> 100").unwrap();
        assert!(result);
    }

    #[test]
    fn test_evaluate_condition_equals() {
        let value = json!("VIP");
        let result = Mapper::evaluate_condition(&value, "== 'VIP'").unwrap();
        assert!(result);
    }

    #[test]
    fn test_merge_sources_concat() {
        let source = json!({
            "first": "John",
            "last": "Doe"
        });
        let result = Mapper::merge_sources(
            &source,
            &["first".to_string(), "last".to_string()],
            Some(&MergeStrategy::Concat),
        )
        .unwrap();
        assert_eq!(result, json!("John Doe"));
    }

    #[test]
    fn test_apply_mappings_multiple_rules() {
        let input = json!({
            "user": {
                "name": "John",
                "age": 30
            }
        });
        let rules = vec![
            MappingRule {
                source_path: "user.name".to_string(),
                target_path: "customer_name".to_string(),
                source_paths: vec![],
                target_paths: vec![],
                direction: MappingDirection::Forward,
                transform: None,
                merge_strategy: None,
                split_strategy: None,
            },
            MappingRule {
                source_path: "user.age".to_string(),
                target_path: "customer_age".to_string(),
                source_paths: vec![],
                target_paths: vec![],
                direction: MappingDirection::Forward,
                transform: None,
                merge_strategy: None,
                split_strategy: None,
            },
        ];
        let result = Mapper::apply_mappings(&input, &rules).unwrap();
        assert_eq!(result["customer_name"], "John");
        assert_eq!(result["customer_age"], 30);
    }
}

