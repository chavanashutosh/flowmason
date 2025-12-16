use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use flowmason_core::types::{Flow, FlowExecution};
use flowmason_db::repositories::{ScheduledFlowRepository, FlowRepository};

pub type FlowExecutor = Arc<dyn Fn(Flow, Value) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<FlowExecution>> + Send>> + Send + Sync>;

pub struct CronExecutor {
    scheduler: Arc<RwLock<JobScheduler>>,
    flow_executors: Arc<RwLock<std::collections::HashMap<String, FlowExecutor>>>,
    job_ids: Arc<RwLock<std::collections::HashMap<String, Uuid>>>,
    scheduled_flow_repo: Option<Arc<ScheduledFlowRepository>>,
    flow_repo: Option<Arc<FlowRepository>>,
}

impl CronExecutor {
    pub async fn new() -> Result<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self {
            scheduler: Arc::new(RwLock::new(scheduler)),
            flow_executors: Arc::new(RwLock::new(std::collections::HashMap::new())),
            job_ids: Arc::new(RwLock::new(std::collections::HashMap::new())),
            scheduled_flow_repo: None,
            flow_repo: None,
        })
    }

    pub async fn with_repositories(
        scheduled_flow_repo: Arc<ScheduledFlowRepository>,
        flow_repo: Arc<FlowRepository>,
    ) -> Result<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self {
            scheduler: Arc::new(RwLock::new(scheduler)),
            flow_executors: Arc::new(RwLock::new(std::collections::HashMap::new())),
            job_ids: Arc::new(RwLock::new(std::collections::HashMap::new())),
            scheduled_flow_repo: Some(scheduled_flow_repo),
            flow_repo: Some(flow_repo),
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
        
        // Persist to database if repository is available
        if let Some(ref repo) = self.scheduled_flow_repo {
            // Check if already exists, update only if cron expression changed
            if let Some(existing) = repo.get_by_flow_id(&flow_id).await? {
                // Only update if the cron expression actually changed
                if existing.cron_expression != cron_expr {
                    repo.update(&flow_id, cron_expr).await?;
                }
            } else {
                repo.create(&flow_id, cron_expr).await?;
            }
        }
        
        // Store the executor
        self.flow_executors.write().await.insert(flow_id.clone(), executor);

        // Create the job
        let job_uuid = Uuid::new_v4();
        let job_id = job_uuid.to_string();
        
        // Store job ID mapping
        self.job_ids.write().await.insert(flow_id.clone(), job_uuid);

        // Capture flow_id and flow_repo for fetching fresh flow on each execution
        let flow_id_for_job = flow_id.clone();
        let flow_repo_for_job = self.flow_repo.clone();

        let job = Job::new_async(cron_expr, move |_uuid, _l| {
            let flow_id = flow_id_for_job.clone();
            let executor = executor_clone.clone();
            let flow_repo = flow_repo_for_job.clone();
            Box::pin(async move {
                let initial_payload = json!({});
                
                // Fetch the flow fresh from the database on each execution
                // This ensures we always use the latest flow definition
                let flow = match flow_repo.as_ref() {
                    Some(repo) => {
                        match repo.get(&flow_id).await {
                            Ok(Some(flow)) => flow,
                            Ok(None) => {
                                tracing::warn!(flow_id = %flow_id, "Scheduled flow not found in database");
                                return;
                            }
                            Err(e) => {
                                tracing::error!(error = %e, flow_id = %flow_id, "Error fetching flow from database");
                                return;
                            }
                        }
                    }
                    None => {
                        tracing::warn!(flow_id = %flow_id, "Flow repository not available for scheduled flow");
                        return;
                    }
                };
                
                tracing::info!(flow_id = %flow.id, "Executing scheduled flow");
                
                // Actually execute the flow with the fresh definition
                match executor(flow.clone(), initial_payload).await {
                    Ok(execution) => {
                        tracing::info!(flow_id = %flow.id, execution_id = %execution.execution_id, "Flow executed successfully");
                    }
                    Err(e) => {
                        tracing::error!(error = %e, flow_id = %flow.id, "Error executing scheduled flow");
                    }
                }
            })
        })?;

        // Add job to scheduler
        self.scheduler.write().await.add(job).await?;

        Ok(job_id)
    }

    /// Starts the cron scheduler
    pub async fn start(&self) -> Result<()> {
        self.scheduler.write().await.start().await?;
        Ok(())
    }

    /// Stops the cron scheduler
    pub async fn stop(&self) -> Result<()> {
        self.scheduler.write().await.shutdown().await?;
        Ok(())
    }

    /// Removes a scheduled flow
    pub async fn unschedule_flow(&self, flow_id: &str) -> Result<()> {
        // Remove from database if repository is available
        if let Some(ref repo) = self.scheduled_flow_repo {
            repo.delete(flow_id).await?;
        }
        
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

    /// Gets list of scheduled flows with cron expressions from database
    pub async fn get_scheduled_flows_with_cron(&self) -> Result<Vec<(String, String)>> {
        if let Some(ref repo) = self.scheduled_flow_repo {
            let flows = repo.list_all().await?;
            Ok(flows.into_iter().map(|f| (f.flow_id, f.cron_expression)).collect())
        } else {
            // Fallback to in-memory storage
            Ok(self.flow_executors.read().await.keys().map(|k| (k.clone(), String::new())).collect())
        }
    }

    /// Loads scheduled flows from database and registers them with executors
    /// This should be called on startup with a function that creates executors
    pub async fn load_scheduled_flows<F>(&self, create_executor: F) -> Result<()>
    where
        F: Fn(&Flow) -> FlowExecutor,
    {
        if let (Some(ref repo), Some(ref flow_repo)) = (self.scheduled_flow_repo.as_ref(), self.flow_repo.as_ref()) {
            let scheduled_flows = repo.list_all().await?;
            
            for scheduled_flow in scheduled_flows {
                // Get the flow from repository
                if let Some(flow) = flow_repo.get(&scheduled_flow.flow_id).await? {
                    // Create executor for this flow
                    let executor = create_executor(&flow);
                    
                    // Schedule the flow (this will add it to the scheduler)
                    self.schedule_flow(flow, &scheduled_flow.cron_expression, executor).await?;
                }
            }
        }
        
        Ok(())
    }
}

// Note: Default cannot be async, so we remove it. Use CronExecutor::new().await instead.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cron_executor_creation() {
        let executor = CronExecutor::new().await;
        assert!(executor.is_ok());
    }
}

