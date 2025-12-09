use serde::{Deserialize, Serialize};
use serde_json::Value;
use flowmason_core::types::{ExecutionStatus, FlowExecution as CoreFlowExecution};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteFlowRequest {
    pub flow_id: String,
    pub input_payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowExecutionResponse {
    pub flow_id: String,
    pub execution_id: String,
    pub status: ExecutionStatus,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub input_payload: Value,
    pub output_payload: Option<Value>,
    pub error: Option<String>,
}

impl From<CoreFlowExecution> for FlowExecutionResponse {
    fn from(exec: CoreFlowExecution) -> Self {
        Self {
            flow_id: exec.flow_id,
            execution_id: exec.execution_id,
            status: exec.status,
            started_at: exec.started_at.to_rfc3339(),
            completed_at: exec.completed_at.map(|d| d.to_rfc3339()),
            input_payload: exec.input_payload,
            output_payload: exec.output_payload,
            error: exec.error,
        }
    }
}

