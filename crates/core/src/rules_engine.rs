use serde_json::Value;
use crate::types::{Rule, RuleCondition, RuleAction, Operator};
use crate::mapper::Mapper;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RulesEngineError {
    #[error("Field not found: {0}")]
    FieldNotFound(String),
    
    #[error("Type mismatch: {0}")]
    TypeMismatch(String),
    
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
    
    #[error("Action execution error: {0}")]
    ActionError(String),
}

pub struct RulesEngine;

impl RulesEngine {
    /// Evaluate a rule condition against input data
    pub fn evaluate_condition(
        condition: &RuleCondition,
        input: &Value,
    ) -> Result<bool, RulesEngineError> {
        match condition {
            RuleCondition::Field { path, operator, value } => {
                Self::evaluate_field_condition(input, path, operator, value)
            }
            RuleCondition::And { conditions } => {
                for cond in conditions {
                    if !Self::evaluate_condition(cond, input)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            RuleCondition::Or { conditions } => {
                for cond in conditions {
                    if Self::evaluate_condition(cond, input)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            RuleCondition::Not { condition } => {
                Ok(!Self::evaluate_condition(condition, input)?)
            }
        }
    }

    fn evaluate_field_condition(
        input: &Value,
        path: &str,
        operator: &Operator,
        expected_value: &Value,
    ) -> Result<bool, RulesEngineError> {
        let field_value = Mapper::get_value_at_path(input, path)
            .map_err(|_| RulesEngineError::FieldNotFound(path.to_string()))?;

        match operator {
            Operator::Equals => Ok(field_value == *expected_value),
            Operator::NotEquals => Ok(field_value != *expected_value),
            Operator::GreaterThan => {
                Self::compare_numbers(&field_value, expected_value, |a, b| a > b)
            }
            Operator::LessThan => {
                Self::compare_numbers(&field_value, expected_value, |a, b| a < b)
            }
            Operator::GreaterThanOrEqual => {
                Self::compare_numbers(&field_value, expected_value, |a, b| a >= b)
            }
            Operator::LessThanOrEqual => {
                Self::compare_numbers(&field_value, expected_value, |a, b| a <= b)
            }
            Operator::Contains => {
                if let (Some(field_str), Some(expected_str)) = (field_value.as_str(), expected_value.as_str()) {
                    Ok(field_str.contains(expected_str))
                } else {
                    Err(RulesEngineError::TypeMismatch("Contains requires string values".to_string()))
                }
            }
            Operator::StartsWith => {
                if let (Some(field_str), Some(expected_str)) = (field_value.as_str(), expected_value.as_str()) {
                    Ok(field_str.starts_with(expected_str))
                } else {
                    Err(RulesEngineError::TypeMismatch("StartsWith requires string values".to_string()))
                }
            }
            Operator::EndsWith => {
                if let (Some(field_str), Some(expected_str)) = (field_value.as_str(), expected_value.as_str()) {
                    Ok(field_str.ends_with(expected_str))
                } else {
                    Err(RulesEngineError::TypeMismatch("EndsWith requires string values".to_string()))
                }
            }
            Operator::Regex => {
                if let (Some(field_str), Some(pattern_str)) = (field_value.as_str(), expected_value.as_str()) {
                    let re = regex::Regex::new(pattern_str)
                        .map_err(|e| RulesEngineError::EvaluationError(format!("Invalid regex: {}", e)))?;
                    Ok(re.is_match(field_str))
                } else {
                    Err(RulesEngineError::TypeMismatch("Regex requires string values".to_string()))
                }
            }
            Operator::In => {
                if let Some(array) = expected_value.as_array() {
                    Ok(array.contains(&field_value))
                } else {
                    Err(RulesEngineError::TypeMismatch("In operator requires array value".to_string()))
                }
            }
            Operator::NotIn => {
                if let Some(array) = expected_value.as_array() {
                    Ok(!array.contains(&field_value))
                } else {
                    Err(RulesEngineError::TypeMismatch("NotIn operator requires array value".to_string()))
                }
            }
            Operator::IsNull => Ok(field_value.is_null()),
            Operator::IsNotNull => Ok(!field_value.is_null()),
        }
    }

    fn compare_numbers<F>(a: &Value, b: &Value, cmp: F) -> Result<bool, RulesEngineError>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        let a_num = a.as_f64()
            .or_else(|| a.as_i64().map(|i| i as f64))
            .or_else(|| a.as_u64().map(|u| u as f64))
            .ok_or_else(|| RulesEngineError::TypeMismatch("Expected numeric value".to_string()))?;
        
        let b_num = b.as_f64()
            .or_else(|| b.as_i64().map(|i| i as f64))
            .or_else(|| b.as_u64().map(|u| u as f64))
            .ok_or_else(|| RulesEngineError::TypeMismatch("Expected numeric value".to_string()))?;
        
        Ok(cmp(a_num, b_num))
    }

    /// Execute rule actions on input data
    pub fn execute_actions(
        actions: &[RuleAction],
        input: &mut Value,
    ) -> Result<Vec<RuleActionResult>, RulesEngineError> {
        let mut results = Vec::new();
        
        for action in actions {
            match action {
                RuleAction::SetField { path, value } => {
                    Mapper::set_value_at_path(input, path, value.clone())
                        .map_err(|e| RulesEngineError::ActionError(format!("Failed to set field {}: {}", path, e)))?;
                    results.push(RuleActionResult::SetField { path: path.clone() });
                }
                RuleAction::Transform { transform } => {
                    // Transform actions are handled by the mapper
                    results.push(RuleActionResult::Transform);
                }
                RuleAction::Branch { flow_id } => {
                    results.push(RuleActionResult::Branch { flow_id: flow_id.clone() });
                }
                RuleAction::SkipBricks { count } => {
                    results.push(RuleActionResult::SkipBricks { count: *count });
                }
            }
        }
        
        Ok(results)
    }

    /// Evaluate rules and return matching rule with actions
    pub fn evaluate_rules(
        rules: &[Rule],
        input: &Value,
    ) -> Result<Option<&Rule>, RulesEngineError> {
        for rule in rules {
            if Self::evaluate_condition(&rule.condition, input)? {
                return Ok(Some(rule));
            }
        }
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub enum RuleActionResult {
    SetField { path: String },
    Transform,
    Branch { flow_id: String },
    SkipBricks { count: usize },
}
