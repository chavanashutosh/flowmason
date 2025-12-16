use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum BrickType {
    OpenAi,
    Nvidia,
    HubSpot,
    Notion,
    Odoo,
    N8n,
    FieldMapping,
    CombineText,
    Conditional,
    RulesEngine,
}

impl BrickType {
    /// Returns the string representation of the brick type
    /// This avoids repeated format! allocations
    pub fn as_str(&self) -> &'static str {
        match self {
            BrickType::OpenAi => "OpenAi",
            BrickType::Nvidia => "Nvidia",
            BrickType::HubSpot => "HubSpot",
            BrickType::Notion => "Notion",
            BrickType::Odoo => "Odoo",
            BrickType::N8n => "N8n",
            BrickType::FieldMapping => "FieldMapping",
            BrickType::CombineText => "CombineText",
            BrickType::Conditional => "Conditional",
            BrickType::RulesEngine => "RulesEngine",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickConfig {
    pub brick_type: BrickType,
    pub config: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bricks: Vec<BrickConfig>,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowExecution {
    pub flow_id: String,
    pub execution_id: String,
    pub status: ExecutionStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub input_payload: Value,
    pub output_payload: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLog {
    pub id: String,
    pub brick_name: String,
    pub flow_id: String,
    pub execution_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cost_unit: f64,
    pub token_usage: Option<i64>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    pub brick_type: BrickType,
    pub daily_limit: u64,
    pub monthly_limit: Option<u64>,
    pub current_daily_usage: u64,
    pub current_monthly_usage: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    #[serde(default = "default_single_path")]
    pub source_path: String,
    #[serde(default = "default_single_path")]
    pub target_path: String,
    #[serde(default)]
    pub source_paths: Vec<String>,
    #[serde(default)]
    pub target_paths: Vec<String>,
    #[serde(default)]
    pub direction: MappingDirection,
    pub transform: Option<TransformType>,
    #[serde(default)]
    pub merge_strategy: Option<MergeStrategy>,
    #[serde(default)]
    pub split_strategy: Option<SplitStrategy>,
}

fn default_single_path() -> String {
    String::new()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MappingDirection {
    Forward,
    Backward,
    Bidirectional,
}

impl Default for MappingDirection {
    fn default() -> Self {
        MappingDirection::Forward
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MergeStrategy {
    Concat,
    MergeObject,
    Array,
    First,
    Last,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SplitStrategy {
    Copy,
    Extract,
    TransformEach,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformType {
    StringConcat { separator: String },
    NumberAdd,
    NumberMultiply,
    StringToUpper,
    StringToLower,
    Conditional { condition: String, true_value: Value, false_value: Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub flow_config: Flow,
    pub is_system: bool,
    pub created_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    Regex,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleCondition {
    #[serde(rename = "field")]
    Field {
        path: String,
        operator: Operator,
        value: Value,
    },
    #[serde(rename = "and")]
    And {
        conditions: Vec<RuleCondition>,
    },
    #[serde(rename = "or")]
    Or {
        conditions: Vec<RuleCondition>,
    },
    #[serde(rename = "not")]
    Not {
        condition: Box<RuleCondition>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleAction {
    #[serde(rename = "set_field")]
    SetField {
        path: String,
        value: Value,
    },
    #[serde(rename = "transform")]
    Transform {
        transform: TransformType,
    },
    #[serde(rename = "branch")]
    Branch {
        flow_id: String,
    },
    #[serde(rename = "skip_bricks")]
    SkipBricks {
        count: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub condition: RuleCondition,
    pub actions: Vec<RuleAction>,
}

