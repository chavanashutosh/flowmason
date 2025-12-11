use serde::{Deserialize, Serialize};
use anyhow::Result;
use gloo_net::http::Request;

const API_BASE_URL: &str = "/api/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bricks: Vec<BrickConfig>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrickConfig {
    pub brick_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFlowRequest {
    pub name: String,
    pub description: Option<String>,
    pub bricks: Vec<BrickConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub execution_id: String,
    pub flow_id: String,
    pub status: String,
    pub input_payload: serde_json::Value,
    pub output_payload: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

impl Execution {
    #[allow(dead_code)]
    pub fn from_json(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledFlow {
    pub flow_id: String,
    pub cron_expression: String,
    pub next_run_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BrickTypeInfo {
    #[serde(rename = "brick_type")]
    pub brick_type: String, // Serialized as snake_case from backend BrickType enum
    pub name: String,
    #[serde(rename = "config_schema")]
    pub config_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickListResponse {
    pub bricks: Vec<BrickTypeInfo>,
}

pub struct ApiClient;

impl ApiClient {
    pub async fn flows_list() -> Result<Vec<Flow>> {
        let response = Request::get(&format!("{}/flows", API_BASE_URL))
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch flows: {}", response.status_text());
        }
        
        let text = response.text().await?;
        if text.is_empty() {
            return Ok(vec![]);
        }
        
        Ok(serde_json::from_str(&text)?)
    }

    pub async fn flows_get(id: &str) -> Result<Flow> {
        let response = Request::get(&format!("{}/flows/{}", API_BASE_URL, id))
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch flow: {}", response.status_text());
        }
        
        Ok(response.json().await?)
    }

    pub async fn flows_create(flow: CreateFlowRequest) -> Result<Flow> {
        let response = Request::post(&format!("{}/flows", API_BASE_URL))
            .json(&flow)?
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create flow: {}", text);
        }
        
        let text = response.text().await?;
        if text.is_empty() {
            anyhow::bail!("Empty response from server");
        }
        
        Ok(serde_json::from_str(&text)?)
    }

    #[allow(dead_code)]
    pub async fn flows_update(id: &str, flow: CreateFlowRequest) -> Result<Flow> {
        let response = Request::put(&format!("{}/flows/{}", API_BASE_URL, id))
            .json(&flow)?
            .send()
            .await?;
        
        if !response.ok() {
            anyhow::bail!("Failed to update flow");
        }
        
        Ok(response.json().await?)
    }

    pub async fn flows_delete(id: &str) -> Result<()> {
        let response = Request::delete(&format!("{}/flows/{}", API_BASE_URL, id))
            .send()
            .await?;
        
        if !response.ok() {
            anyhow::bail!("Failed to delete flow");
        }
        
        Ok(())
    }

    pub async fn executions_list() -> Result<Vec<Execution>> {
        let response = Request::get(&format!("{}/executions", API_BASE_URL))
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch executions");
        }
        
        let text = response.text().await?;
        if text.is_empty() {
            return Ok(vec![]);
        }
        
        Ok(serde_json::from_str(&text)?)
    }

    #[allow(dead_code)]
    pub async fn executions_get(id: &str) -> Result<Execution> {
        let response = Request::get(&format!("{}/executions/{}", API_BASE_URL, id))
            .send()
            .await?;
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch execution");
        }
        
        Ok(response.json().await?)
    }

    #[allow(dead_code)]
    pub async fn executions_list_by_flow(flow_id: &str) -> Result<Vec<Execution>> {
        let response = Request::get(&format!("{}/executions/flow/{}", API_BASE_URL, flow_id))
            .send()
            .await?;
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch flow executions");
        }
        
        Ok(response.json().await?)
    }

    #[allow(dead_code)]
    pub async fn executions_execute(flow_id: &str, input_payload: serde_json::Value) -> Result<Execution> {
        let response = Request::post(&format!("{}/executions", API_BASE_URL))
            .json(&serde_json::json!({
                "flow_id": flow_id,
                "input_payload": input_payload,
            }))?
            .send()
            .await?;
        
        if !response.ok() {
            let error: serde_json::Value = response.json().await.unwrap_or_default();
            anyhow::bail!("Execution failed: {}", error.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error"));
        }
        
        Ok(response.json().await?)
    }

    pub async fn scheduler_list_scheduled_flows() -> Result<Vec<ScheduledFlow>> {
        let response = Request::get(&format!("{}/scheduler/flows", API_BASE_URL))
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch scheduled flows");
        }
        
        let text = response.text().await?;
        if text.is_empty() {
            return Ok(vec![]);
        }
        
        let data: serde_json::Value = serde_json::from_str(&text)?;
        Ok(data.get("flows").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default())
    }

    pub async fn scheduler_schedule_flow(flow_id: &str, cron_expression: &str) -> Result<ScheduledFlow> {
        let response = Request::post(&format!("{}/scheduler/flows", API_BASE_URL))
            .json(&serde_json::json!({
                "flow_id": flow_id,
                "cron_expression": cron_expression,
            }))?
            .send()
            .await?;
        
        if !response.ok() {
            anyhow::bail!("Failed to schedule flow");
        }
        
        Ok(response.json().await?)
    }

    pub async fn scheduler_unschedule_flow(flow_id: &str) -> Result<()> {
        let response = Request::delete(&format!("{}/scheduler/flows/{}", API_BASE_URL, flow_id))
            .send()
            .await?;
        
        if !response.ok() {
            anyhow::bail!("Failed to unschedule flow");
        }
        
        Ok(())
    }

    pub async fn bricks_list() -> Result<BrickListResponse> {
        let response = Request::get(&format!("{}/bricks", API_BASE_URL))
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch bricks: {}", response.status_text());
        }
        
        Ok(response.json().await?)
    }

    pub async fn bricks_get_schema(brick_type: &str) -> Result<serde_json::Value> {
        let response = Request::get(&format!("{}/bricks/{}/schema", API_BASE_URL, brick_type))
            .send()
            .await?;
        
        if response.status() == 431 {
            anyhow::bail!("Request headers too large. Please clear your browser cookies and try again.");
        }
        
        if !response.ok() {
            anyhow::bail!("Failed to fetch brick schema: {}", response.status_text());
        }
        
        Ok(response.json().await?)
    }

    pub async fn usage_get_stats() -> Result<serde_json::Value> {
        let response = Request::get(&format!("{}/usage/stats", API_BASE_URL))
            .send()
            .await?;
        Ok(response.json().await?)
    }
}

