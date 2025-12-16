use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType, RulesEngine, Rule};
use serde_json::{json, Value};

pub struct RulesEngineBrick;

#[async_trait]
impl Brick for RulesEngineBrick {
    fn name(&self) -> &'static str {
        "rules_engine"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::RulesEngine
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "rules": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Rule name"
                            },
                            "condition": {
                                "description": "Rule condition"
                            },
                            "actions": {
                                "type": "array",
                                "description": "Actions to execute if condition matches"
                            }
                        },
                        "required": ["name", "condition", "actions"]
                    },
                    "description": "Array of rules to evaluate"
                },
                "default_actions": {
                    "type": "array",
                    "description": "Actions to execute if no rules match"
                }
            },
            "required": ["rules"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let rules_array = config
            .get("rules")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BrickError::ConfigError("rules array is required".to_string()))?;

        let mut rules = Vec::new();
        for rule_json in rules_array {
            let rule: Rule = serde_json::from_value(rule_json.clone())
                .map_err(|e| BrickError::ConfigError(format!("Invalid rule: {}", e)))?;
            rules.push(rule);
        }

        let default_actions = config
            .get("default_actions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Evaluate rules
        let matching_rule = RulesEngine::evaluate_rules(&rules, &input)
            .map_err(|e| BrickError::ExecutionError(format!("Rule evaluation error: {}", e)))?;

        let mut output = input.clone();

        let mut skip_bricks_count = None;
        
        if let Some(rule) = matching_rule {
            // Execute actions from matching rule
            let results = RulesEngine::execute_actions(&rule.actions, &mut output)
                .map_err(|e| BrickError::ExecutionError(format!("Action execution error: {}", e)))?;
            
            // Check for SkipBricks action result
            for result in results {
                if let flowmason_core::RuleActionResult::SkipBricks { count } = result {
                    skip_bricks_count = Some(count);
                }
            }
        } else if !default_actions.is_empty() {
            // Execute default actions if no rule matches
            let actions: Vec<flowmason_core::types::RuleAction> = serde_json::from_value(
                json!(default_actions)
            ).map_err(|e| BrickError::ConfigError(format!("Invalid default actions: {}", e)))?;
            
            let results = RulesEngine::execute_actions(&actions, &mut output)
                .map_err(|e| BrickError::ExecutionError(format!("Default action execution error: {}", e)))?;
            
            // Check for SkipBricks action result
            for result in results {
                if let flowmason_core::RuleActionResult::SkipBricks { count } = result {
                    skip_bricks_count = Some(count);
                }
            }
        }

        // Add metadata about which rule matched and branching info
        if let Some(obj) = output.as_object_mut() {
            if let Some(rule) = matching_rule {
                obj.insert("_matched_rule".to_string(), json!(rule.name));
            } else {
                obj.insert("_matched_rule".to_string(), Value::Null);
            }
            
            if let Some(skip_count) = skip_bricks_count {
                obj.insert("_skip_bricks".to_string(), json!(skip_count));
            }
        }

        Ok(output)
    }

    fn validate_config(&self, config: &Value) -> Result<(), BrickError> {
        if !config.is_object() {
            return Err(BrickError::ConfigError("Config must be an object".to_string()));
        }

        if config.get("rules").and_then(|v| v.as_array()).is_none() {
            return Err(BrickError::ConfigError("rules array is required".to_string()));
        }

        Ok(())
    }
}
