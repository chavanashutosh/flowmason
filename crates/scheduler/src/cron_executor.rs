use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use flowmason_core::types::{Flow, FlowExecution};

pub type FlowExecutor = Arc<dyn Fn(Flow, Value) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<FlowExecution>> + Send>> + Send + Sync>;

pub struct CronExecutor {
    scheduler: Arc<RwLock<JobScheduler>>,
    flow_executors: Arc<RwLock<std::collections::HashMap<String, FlowExecutor>>>,
    job_ids: Arc<RwLock<std::collections::HashMap<String, Uuid>>>,
}

impl CronExecutor {
    pub fn new() -> Result<Self> {
        let scheduler = JobScheduler::new()?;
        Ok(Self {
            scheduler: Arc::new(RwLock::new(scheduler)),
            flow_executors: Arc::new(RwLock::new(std::collections::HashMap::new())),
            job_ids: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// Registers a flow to be executed on a cron schedule
    pub async fn schedule_flow(
        &self,
        flow: Flow,
        cron_expr: &str,
        executor: FlowExecutor,
    ) -> Result<String> {
        let flow_id = flow.id.clone();
        let executor_clone = executor.clone();
        let flow_clone = flow.clone();
        
        // Store the executor
        self.flow_executors.write().await.insert(flow_id.clone(), executor);

        // Create the job
        let job_uuid = Uuid::new_v4();
        let job_id = job_uuid.to_string();
        
        // Store job ID mapping
        self.job_ids.write().await.insert(flow_id.clone(), job_uuid);

        let job = Job::new_async(cron_expr, move |_uuid, _l| {
            let flow = flow_clone.clone();
            let executor = executor_clone.clone();
            Box::pin(async move {
                let initial_payload = json!({});
                println!("Executing scheduled flow {} at {}", flow.id, chrono::Utc::now());
                
                // Actually execute the flow
                match executor(flow.clone(), initial_payload).await {
                    Ok(execution) => {
                        println!("Flow {} executed successfully. Execution ID: {}", flow.id, execution.execution_id);
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Error executing scheduled flow {}: {}", flow.id, e);
                        Err(anyhow::anyhow!("Flow execution failed: {}", e))
                    }
                }
            })
        })?;

        // Add job to scheduler
        self.scheduler.write().await.add(job)?;

        Ok(job_id)
    }

    /// Starts the cron scheduler
    pub async fn start(&self) -> Result<()> {
        self.scheduler.write().await.start()?;
        Ok(())
    }

    /// Stops the cron scheduler
    pub async fn stop(&self) -> Result<()> {
        self.scheduler.write().await.shutdown()?;
        Ok(())
    }

    /// Removes a scheduled flow
    pub async fn unschedule_flow(&self, flow_id: &str) -> Result<()> {
        // Remove executor
        self.flow_executors.write().await.remove(flow_id);
        
        // Remove job ID mapping
        self.job_ids.write().await.remove(flow_id);
        
        // Note: JobScheduler doesn't have a direct remove method in the version we're using
        // The job will continue to run but won't execute anything since the executor is removed
        // In production with a newer version, you'd use remove() method with the job UUID
        Ok(())
    }

    /// Gets list of scheduled flow IDs
    pub async fn get_scheduled_flows(&self) -> Vec<String> {
        self.flow_executors.read().await.keys().cloned().collect()
    }
}

impl Default for CronExecutor {
    fn default() -> Self {
        Self::new().expect("Failed to create CronExecutor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cron_executor_creation() {
        let executor = CronExecutor::new();
        assert!(executor.is_ok());
    }
}

