use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FlowExecutionJob {
    pub job_id: String,
    pub flow_id: String,
    pub input_payload: Value,
}

pub struct JobQueue {
    sender: mpsc::UnboundedSender<FlowExecutionJob>,
}

impl JobQueue {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<FlowExecutionJob>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (Self { sender }, receiver)
    }

    pub fn enqueue(&self, flow_id: String, input_payload: Value) -> String {
        let job_id = Uuid::new_v4().to_string();
        let job = FlowExecutionJob {
            job_id: job_id.clone(),
            flow_id,
            input_payload,
        };
        
        if let Err(e) = self.sender.send(job) {
            tracing::error!(error = %e, "Failed to enqueue job");
        } else {
            tracing::info!(job_id = %job_id, "Job enqueued");
        }
        
        job_id
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        let (sender, _) = mpsc::unbounded_channel();
        Self { sender }
    }
}
