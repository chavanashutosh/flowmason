use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleFlowRequest {
    pub flow_id: String,
    pub cron_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleFlowResponse {
    pub job_id: String,
    pub flow_id: String,
    pub cron_expression: String,
    pub scheduled_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledFlowResponse {
    pub flow_id: String,
    pub cron_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledFlowsResponse {
    pub flows: Vec<ScheduledFlowResponse>,
}

