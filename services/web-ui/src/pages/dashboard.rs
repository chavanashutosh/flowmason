use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::api::ApiClient;
use crate::components::ui::{StatsCard, OnboardingPanel, StatusBadge, EmptyState, empty_state::EmptyStateIcon};
use crate::components::ui::icons::{Workflow, Clock, Calendar, TrendingUp};
use crate::router::Route;

#[component]
pub fn Dashboard() -> Element {
    let stats = use_signal(|| DashboardStats {
        total_flows: 0,
        total_executions: 0,
        scheduled_flows: 0,
        recent_executions: vec![],
    });
    let loading = use_signal(|| true);

    use_future(move || {
        let mut stats = stats;
        let mut loading = loading;
        async move {
            match fetch_dashboard_data().await {
                Ok(data) => {
                    stats.set(data);
                }
                Err(e) => {
                    log::error!("Failed to fetch dashboard data: {}", e);
                }
            }
            loading.set(false);
        }
    });

    rsx! {
        div { class: "space-y-8",
            // Title row with action button
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900", "Dashboard" }
                    p { class: "text-sm text-gray-500 mt-1", "Overview of your automation platform" }
                }
                Link {
                    to: Route::NewFlow {},
                    button {
                        class: "px-6 py-2.5 bg-primary-600 hover:bg-primary-700 text-white font-medium rounded-lg transition-colors h-10",
                        "Create Flow"
                    }
                }
            }

            OnboardingPanel {}

            if *loading.read() {
                div { class: "flex items-center justify-center h-64",
                    "Loading..."
                }
            } else {
                div { class: "space-y-8",
                    // Metrics grid - full width 2x2 or 1x4
                    div { class: "grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4",
                        StatsCard {
                            title: "Total Flows",
                            value: stats.read().total_flows.to_string(),
                            icon: Some(rsx! { Workflow { size: 24, class: "text-primary-600".to_string() } }),
                        }
                        StatsCard {
                            title: "Total Executions",
                            value: stats.read().total_executions.to_string(),
                            icon: Some(rsx! { Clock { size: 24, class: "text-primary-600".to_string() } }),
                        }
                        StatsCard {
                            title: "Scheduled Flows",
                            value: stats.read().scheduled_flows.to_string(),
                            icon: Some(rsx! { Calendar { size: 24, class: "text-primary-600".to_string() } }),
                        }
                        StatsCard {
                            title: "Usage Today",
                            value: "0".to_string(),
                            icon: Some(rsx! { TrendingUp { size: 24, class: "text-primary-600".to_string() } }),
                        }
                    }

                    // Recent Executions - full width section
                    div { class: "bg-white border border-gray-200 rounded-lg shadow-sm",
                        div { class: "px-6 py-4 border-b border-gray-200",
                            h2 { class: "text-xl font-semibold text-gray-900", "Recent Executions" }
                        }
                        div { class: "p-6",
                            if stats.read().recent_executions.is_empty() {
                                EmptyState {
                                    title: "No recent executions".to_string(),
                                    description: "Run a flow to see execution history here.".to_string(),
                                    action_label: Some("Create Flow".to_string()),
                                    action_route: Some(Route::NewFlow {}),
                                    action_onclick: None,
                                    icon: EmptyStateIcon::Clock,
                                }
                            } else {
                                div { class: "overflow-x-auto",
                                    table { class: "min-w-full divide-y divide-gray-200",
                                        thead { class: "bg-gray-50",
                                            tr {
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Execution ID" }
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Status" }
                                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider", "Created" }
                                            }
                                        }
                                        tbody { class: "bg-white divide-y divide-gray-200",
                                            for execution in stats.read().recent_executions.iter() {
                                                tr { class: "hover:bg-gray-50",
                                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900",
                                                        "{execution.execution_id.chars().take(12).collect::<String>()}..."
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap",
                                                        StatusBadge {
                                                            status: match execution.status.as_str() {
                                                                "completed" => crate::components::ui::status_badge::Status::Completed,
                                                                "failed" => crate::components::ui::status_badge::Status::Failed,
                                                                "running" => crate::components::ui::status_badge::Status::Running,
                                                                _ => crate::components::ui::status_badge::Status::Pending,
                                                            }
                                                        }
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                                        "{execution.created_at}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct DashboardStats {
    total_flows: usize,
    total_executions: usize,
    scheduled_flows: usize,
    recent_executions: Vec<ExecutionSummary>,
}

#[derive(Clone, Debug)]
struct ExecutionSummary {
    execution_id: String,
    status: String,
    created_at: String,
}

async fn fetch_dashboard_data() -> anyhow::Result<DashboardStats> {
    let flows = ApiClient::flows_list().await.unwrap_or_default();
    let executions = ApiClient::executions_list().await.unwrap_or_default();
    let scheduled = ApiClient::scheduler_list_scheduled_flows().await.unwrap_or_default();

    let recent_executions = executions
        .iter()
        .take(5)
        .map(|e| ExecutionSummary {
            execution_id: e.execution_id.clone(),
            status: e.status.clone(),
            created_at: e.created_at.clone(),
        })
        .collect();

    Ok(DashboardStats {
        total_flows: flows.len(),
        total_executions: executions.len(),
        scheduled_flows: scheduled.len(),
        recent_executions,
    })
}

