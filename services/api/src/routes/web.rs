use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use askama::Template;

use crate::dto::{CreateFlowRequest, FlowResponse};
use crate::routes::FlowState;
use crate::templates::{BaseTemplate, components};
use flowmason_core::types::Template;

pub fn routes() -> Router<FlowState> {
    Router::new()
        .route("/", get(dashboard))
        .route("/flows", get(flows_list).post(flows_create))
        .route("/flows/new", get(flows_new))
        .route("/flows/:id", get(flows_detail).post(flows_delete))
        .route("/flows/:id/edit", get(flows_edit).post(flows_update))
        .route("/flows/:id/run", post(flows_run))
        .route("/templates", get(templates))
        .route("/templates/:id/instantiate", post(instantiate_template))
        .route("/executions", get(executions_list))
        .route("/executions/:id", get(executions_detail))
        .route("/scheduler", get(scheduler).post(scheduler_create))
        .route("/scheduler/:flow_id", post(scheduler_delete))
        .route("/metering", get(metering))
        .route("/mapping", get(mapping))
        .route("/documentation", get(documentation))
        .route("/settings", get(settings))
}

async fn dashboard(
    State(state): State<FlowState>,
) -> Result<Html<String>, StatusCode> {
    // Get flows, executions, and scheduled flows
    let flows = state.flow_repo.list().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // For now, we'll use placeholder data for executions and scheduled flows
    // In a full implementation, you'd inject ExecutionState and SchedulerState
    let total_flows = flows.len();
    let total_executions = 0; // Would come from ExecutionState
    let scheduled_flows = 0; // Would come from SchedulerState
    
    let recent_executions_html = if total_executions == 0 {
        components::empty_state(
            "No recent executions",
            "Run a flow to see execution history here.",
            Some("Create Flow"),
            Some("/flows/new"),
        )
    } else {
        String::from("<p class=\"text-gray-500\">Execution history would appear here</p>")
    };
    
    let stats_html = format!(
        r#"<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
            {}
            {}
            {}
            {}
        </div>"#,
        components::stats_card("Total Flows", &total_flows.to_string(), None),
        components::stats_card("Total Executions", &total_executions.to_string(), None),
        components::stats_card("Scheduled Flows", &scheduled_flows.to_string(), None),
        components::stats_card("Usage Today", "0", None),
    );
    
    let content = format!(
        r#"<div class="space-y-8">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
                    <p class="text-sm text-gray-500 mt-1">Overview of your automation platform</p>
                </div>
                <a href="/flows/new">
                    <button class="px-6 py-2.5 bg-primary-600 hover:bg-primary-700 text-white font-medium rounded-lg transition-colors h-10">
                        Create Flow
                    </button>
                </a>
            </div>
            {}
            <div class="bg-white border border-gray-200 rounded-lg shadow-sm">
                <div class="px-6 py-4 border-b border-gray-200">
                    <h2 class="text-xl font-semibold text-gray-900">Recent Executions</h2>
                </div>
                <div class="p-6">
                    {}
                </div>
            </div>
        </div>"#,
        stats_html, recent_executions_html
    );
    
    let template = BaseTemplate {
        title: "Dashboard".to_string(),
        content,
        current_path: "/".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn flows_list(
    State(state): State<FlowState>,
) -> Result<Html<String>, StatusCode> {
    let flows = state.flow_repo.list().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let flows_responses: Vec<FlowResponse> = flows.into_iter().map(FlowResponse::from).collect();
    
    let flows_table = if flows_responses.is_empty() {
        components::empty_state(
            "No flows created yet",
            "Get started by creating a new flow",
            Some("Create New Flow"),
            Some("/flows/new"),
        )
    } else {
        let headers = vec!["Name", "Status", "Created", "Actions"];
        let rows: Vec<Vec<String>> = flows_responses
            .iter()
            .map(|flow| {
                let status_badge = components::status_badge(if flow.active { "active" } else { "inactive" });
                let actions = format!(
                    r#"<div class="flex gap-2">
                        <form method="post" action="/flows/{}/run" style="display: inline;">
                            <button type="submit" class="flex items-center px-3 py-1.5 text-sm text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded transition-colors">
                                <svg class="w-3.5 h-3.5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                Run
                            </button>
                        </form>
                        <a href="/flows/{}" class="flex items-center px-3 py-1.5 text-sm text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded transition-colors">
                            <svg class="w-3.5 h-3.5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                            </svg>
                            Edit
                        </a>
                        <form method="post" action="/flows/{}" style="display: inline;" onsubmit="return confirm('Are you sure you want to delete this flow?');">
                            <button type="submit" class="flex items-center px-3 py-1.5 text-sm text-red-600 hover:text-red-900 hover:bg-red-50 rounded transition-colors">
                                Delete
                            </button>
                        </form>
                    </div>"#,
                    flow.id, flow.id, flow.id
                );
                vec![
                    format!(
                        r#"<div>
                            <div class="text-sm font-medium text-gray-900">{}</div>
                            {}
                        </div>"#,
                        flow.name,
                        flow.description.as_ref().map(|d| format!(r#"<div class="text-sm text-gray-500">{}</div>"#, d)).unwrap_or_default()
                    ),
                    status_badge,
                    flow.created_at.clone(),
                    actions,
                ]
            })
            .collect();
        
        components::data_table(&headers, &rows)
    };
    
    let content = format!(
        r#"<div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">Flows</h1>
                    <p class="text-gray-600 mt-1">Manage your automation flows</p>
                </div>
                <a href="/flows/new">
                    <button class="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg">
                        Create Flow
                    </button>
                </a>
            </div>
            {}
        </div>"#,
        flows_table
    );
    
    let template = BaseTemplate {
        title: "Flows".to_string(),
        content,
        current_path: "/flows".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn flows_new() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div>
                <h1 class="text-3xl font-bold text-gray-900">Create New Flow</h1>
                <p class="text-gray-600 mt-1">Build a new automation flow</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                <form method="post" action="/flows" class="space-y-6">
                    <div>
                        <label for="name" class="block text-sm font-medium text-gray-700 mb-2">Flow Name</label>
                        <input type="text" id="name" name="name" required class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500">
                    </div>
                    <div>
                        <label for="description" class="block text-sm font-medium text-gray-700 mb-2">Description</label>
                        <textarea id="description" name="description" rows="3" class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"></textarea>
                    </div>
                    <div class="flex justify-end space-x-3">
                        <a href="/flows" class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200">
                            Cancel
                        </a>
                        <button type="submit" class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700">
                            Create Flow
                        </button>
                    </div>
                </form>
            </div>
        </div>"#
    );
    
    let template = BaseTemplate {
        title: "Create Flow".to_string(),
        content,
        current_path: "/flows".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

#[derive(Deserialize)]
struct FlowForm {
    name: String,
    description: Option<String>,
}

async fn flows_create(
    State(state): State<FlowState>,
    Form(form): Form<FlowForm>,
) -> Result<Redirect, StatusCode> {
    let request = CreateFlowRequest {
        name: form.name,
        description: form.description,
        bricks: vec![],
    };
    
    // Create flow using the API logic
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let flow = flowmason_core::types::Flow {
        id: id.clone(),
        name: request.name,
        description: request.description,
        bricks: vec![],
        active: true,
        created_at: now,
        updated_at: now,
    };
    
    state.flow_repo.create(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to(&format!("/flows/{}", id)))
}

async fn flows_detail(
    State(state): State<FlowState>,
    Path(id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let flow_response = FlowResponse::from(flow.clone());
    let status_badge = components::status_badge(if flow.active { "active" } else { "inactive" });
    
    let content = format!(
        r#"<div class="space-y-6">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <a href="/flows" class="flex items-center px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                        </svg>
                        Back
                    </a>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">{}</h1>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <form method="post" action="/flows/{}/run" style="display: inline;">
                        <button type="submit" class="flex items-center px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors">
                            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            Run Flow
                        </button>
                    </form>
                    <a href="/flows/{}/edit" class="flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                        </svg>
                        Edit
                    </a>
                    <form method="post" action="/flows/{}" style="display: inline;" onsubmit="return confirm('Are you sure you want to delete this flow?');">
                        <button type="submit" class="flex items-center px-4 py-2 text-sm font-medium text-red-600 bg-white border border-red-300 rounded-lg hover:bg-red-50 transition-colors">
                            Delete
                        </button>
                    </form>
                </div>
            </div>
            <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">Status</label>
                        <div>{}</div>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
                        <p class="text-gray-900">{}</p>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">Created</label>
                        <p class="text-gray-900">{}</p>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">Bricks</label>
                        <p class="text-gray-500">No bricks configured yet. <a href="/flows/{}/edit" class="text-primary-600 hover:text-primary-700">Add bricks</a></p>
                    </div>
                </div>
            </div>
        </div>"#,
        flow_response.name,
        id,
        id,
        id,
        status_badge,
        flow_response.description.as_deref().unwrap_or("No description"),
        flow_response.created_at,
        id
    );
    
    let template = BaseTemplate {
        title: flow_response.name.clone(),
        content,
        current_path: "/flows".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn flows_edit(
    State(state): State<FlowState>,
    Path(id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let flow_response = FlowResponse::from(flow);
    
    let content = format!(
        r#"<div class="space-y-6">
            <div>
                <h1 class="text-3xl font-bold text-gray-900">Edit Flow</h1>
                <p class="text-gray-600 mt-1">Update flow configuration</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                <form method="post" action="/flows/{}/edit" class="space-y-6">
                    <div>
                        <label for="name" class="block text-sm font-medium text-gray-700 mb-2">Flow Name</label>
                        <input type="text" id="name" name="name" value="{}" required class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500">
                    </div>
                    <div>
                        <label for="description" class="block text-sm font-medium text-gray-700 mb-2">Description</label>
                        <textarea id="description" name="description" rows="3" class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500">{}</textarea>
                    </div>
                    <div class="flex justify-end space-x-3">
                        <a href="/flows/{}" class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200">
                            Cancel
                        </a>
                        <button type="submit" class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700">
                            Save Changes
                        </button>
                    </div>
                </form>
            </div>
        </div>"#,
        id,
        flow_response.name,
        flow_response.description.as_deref().unwrap_or(""),
        id
    );
    
    let template = BaseTemplate {
        title: "Edit Flow".to_string(),
        content,
        current_path: "/flows".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

#[derive(Deserialize)]
struct FlowUpdateForm {
    name: Option<String>,
    description: Option<String>,
}

async fn flows_update(
    State(state): State<FlowState>,
    Path(id): Path<String>,
    Form(form): Form<FlowUpdateForm>,
) -> Result<Redirect, StatusCode> {
    let mut flow = state.flow_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if let Some(name) = form.name {
        flow.name = name;
    }
    if let Some(description) = form.description {
        flow.description = Some(description);
    }
    flow.updated_at = chrono::Utc::now();
    
    state.flow_repo.update(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to(&format!("/flows/{}", id)))
}

async fn flows_delete(
    State(state): State<FlowState>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    state.flow_repo.delete(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/flows"))
}

async fn flows_run(
    State(_state): State<FlowState>,
    Path(_id): Path<String>,
) -> Result<Redirect, StatusCode> {
    // This would need ExecutionState - for now, redirect to executions
    // In a full implementation, you'd execute the flow here
    Ok(Redirect::to("/executions"))
}

async fn templates(
    State(state): State<FlowState>,
) -> Result<Html<String>, StatusCode> {
    let templates_list = state.template_repo.list(None, true, Some(100), Some(0))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let template_cards: String = if templates_list.is_empty() {
        components::empty_state(
            "No templates available",
            "Templates will appear here once they are created.",
            None,
            None,
        )
    } else {
        templates_list.iter()
            .map(|t| {
                let category_badge = if t.is_system {
                    format!(r#"<span class="px-2 py-1 text-xs font-semibold rounded bg-blue-100 text-blue-800">System</span>"#)
                } else {
                    format!(r#"<span class="px-2 py-1 text-xs font-semibold rounded bg-gray-100 text-gray-800">User</span>"#)
                };
                
                let name = t.name.replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;");
                let desc = t.description.as_deref().unwrap_or("No description")
                    .replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;");
                let category = t.category.replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;");
                let id = t.id.replace('"', "&quot;");
                
                format!(
                    r#"<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow">
                        <div class="flex items-start justify-between mb-2">
                            <h3 class="text-lg font-semibold text-gray-900">{}</h3>
                            {}
                        </div>
                        <p class="text-sm text-gray-600 mb-2">{}</p>
                        <p class="text-xs text-gray-500 mb-4">Category: {}</p>
                        <a href="/templates/{}/instantiate" class="text-primary-600 hover:text-primary-700 text-sm font-medium">
                            Use Template →
                        </a>
                    </div>"#,
                    name, category_badge, desc, category, id
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    
    let content = format!(
        r#"<div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">Templates</h1>
                    <p class="text-gray-600 mt-1">Choose a template to get started</p>
                </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {}
            </div>
        </div>"#,
        template_cards
    );
    
    let template = BaseTemplate {
        title: "Templates".to_string(),
        content,
        current_path: "/templates".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn executions_list() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">Execution History</h1>
                    <p class="text-gray-600 mt-1">View and monitor flow execution history</p>
                </div>
                <a href="/executions">
                    <button class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200">
                        Refresh
                    </button>
                </a>
            </div>
            {}
        </div>"#,
        components::empty_state(
            "No executions yet",
            "Run a flow to see execution history here.",
            Some("Create Flow"),
            Some("/flows/new"),
        )
    );
    
    let template = BaseTemplate {
        title: "Executions".to_string(),
        content,
        current_path: "/executions".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn executions_detail(
    Path(id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div>
                <h1 class="text-3xl font-bold text-gray-900">Execution Details</h1>
                <p class="text-gray-600 mt-1">Execution ID: {}</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                <p class="text-gray-500">Execution details would appear here</p>
            </div>
        </div>"#,
        id
    );
    
    let template = BaseTemplate {
        title: "Execution Details".to_string(),
        content,
        current_path: "/executions".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn scheduler() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">Scheduler</h1>
                    <p class="text-gray-600 mt-1">Manage scheduled flows</p>
                </div>
            </div>
            {}
        </div>"#,
        components::empty_state(
            "No scheduled flows",
            "Schedule a flow to run automatically on a schedule.",
            Some("Schedule Flow"),
            Some("/flows"),
        )
    );
    
    let template = BaseTemplate {
        title: "Scheduler".to_string(),
        content,
        current_path: "/scheduler".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

#[derive(Deserialize)]
struct SchedulerForm {
    #[allow(dead_code)]
    flow_id: String,
    #[allow(dead_code)]
    cron_expression: String,
}

async fn scheduler_create(
    Form(_form): Form<SchedulerForm>,
) -> Result<Redirect, StatusCode> {
    // This would need SchedulerState - for now, just redirect
    // Form fields are intentionally unused as this is a placeholder implementation
    Ok(Redirect::to("/scheduler"))
}

async fn scheduler_delete(
    Path(_flow_id): Path<String>,
) -> Result<Redirect, StatusCode> {
    // This would need SchedulerState - for now, just redirect
    Ok(Redirect::to("/scheduler"))
}

async fn metering() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div class="flex items-center gap-3">
                <span class="text-3xl">📈</span>
                <div>
                    <h1 class="text-3xl font-bold text-gray-900">Usage & Metering</h1>
                    <p class="text-gray-600 mt-1">Monitor usage and quotas for each brick type</p>
                </div>
            </div>
            {}
        </div>"#,
        components::empty_state(
            "No usage data available",
            "Usage statistics will appear here once you start using flows.",
            None,
            None,
        )
    );
    
    let template = BaseTemplate {
        title: "Metering".to_string(),
        content,
        current_path: "/metering".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn mapping() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div>
                <h1 class="text-3xl font-bold text-gray-900">Field Mapping</h1>
                <p class="text-gray-600 mt-1">Map fields between different data structures</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                <p class="text-gray-500">Field mapping interface coming soon.</p>
            </div>
        </div>"#
    );
    
    let template = BaseTemplate {
        title: "Mapping".to_string(),
        content,
        current_path: "/mapping".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn documentation() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div>
                <h1 class="text-3xl font-bold text-gray-900 flex items-center gap-3">
                    <span class="text-3xl">📚</span>
                    Documentation
                </h1>
                <p class="text-gray-600 mt-2">Learn how to use FlowMason to build automation workflows</p>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">Getting Started</h2>
                    <div class="space-y-4 text-sm text-gray-600">
                        <div>
                            <h3 class="font-medium text-gray-900 mb-1">What is FlowMason?</h3>
                            <p>FlowMason is a visual automation platform that allows you to build powerful workflows by connecting different services and APIs together.</p>
                        </div>
                        <div>
                            <h3 class="font-medium text-gray-900 mb-1">Your First Flow</h3>
                            <p>1. Navigate to Flows and click "Create Flow"<br>2. Give your flow a name and description<br>3. Add bricks to define your workflow steps<br>4. Configure each brick with the required parameters<br>5. Save and test your flow</p>
                        </div>
                    </div>
                </div>
                <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">Creating Flows</h2>
                    <div class="space-y-4 text-sm text-gray-600">
                        <div>
                            <h3 class="font-medium text-gray-900 mb-1">Flow Builder</h3>
                            <p>The visual flow builder lets you drag and connect nodes to create your automation workflow.</p>
                        </div>
                        <div>
                            <h3 class="font-medium text-gray-900 mb-1">Adding Bricks</h3>
                            <p>Click "Add Brick" to see available integrations. Select a brick type and configure it.</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>"#
    );
    
    let template = BaseTemplate {
        title: "Documentation".to_string(),
        content,
        current_path: "/documentation".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn settings() -> Result<Html<String>, StatusCode> {
    let content = format!(
        r#"<div class="space-y-6">
            <div>
                <h1 class="text-3xl font-bold text-gray-900">Settings</h1>
                <p class="text-gray-600 mt-1">Manage your account and preferences</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                <p class="text-gray-500">Settings page coming soon.</p>
            </div>
        </div>"#
    );
    
    let template = BaseTemplate {
        title: "Settings".to_string(),
        content,
        current_path: "/settings".to_string(),
    };
    
    Ok(Html(template.render().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

async fn instantiate_template(
    State(state): State<FlowState>,
    Path(id): Path<String>,
) -> Result<Redirect, StatusCode> {
    let template = state.template_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let flow_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let mut flow = template.flow_config.clone();
    flow.id = flow_id.clone();
    flow.name = format!("{} (Copy)", template.name);
    flow.description = template.description.clone();
    flow.created_at = now;
    flow.updated_at = now;
    
    state.flow_repo.create(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to(&format!("/flows/{}", flow_id)))
}
