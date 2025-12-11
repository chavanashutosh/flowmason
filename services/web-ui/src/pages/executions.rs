use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::api::{ApiClient, Execution};
use crate::components::ui::{StatusBadge, status_badge::Status, Breadcrumbs, breadcrumbs::BreadcrumbItem};
use crate::router::Route;

#[component]
pub fn Executions() -> Element {
    let executions = use_signal(|| Vec::<Execution>::new());
    let loading = use_signal(|| true);

    let refresh = move |_| {
        let mut executions = executions;
        let mut loading = loading;
        spawn(async move {
            loading.set(true);
            match ApiClient::executions_list().await {
                Ok(data) => executions.set(data),
                Err(e) => log::error!("Failed to fetch executions: {}", e),
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        refresh(());
    });

    rsx! {
        div { class: "space-y-6",
            Breadcrumbs {
                items: vec![
                    BreadcrumbItem { label: "Dashboard".to_string(), route: Some(Route::Dashboard {}) },
                    BreadcrumbItem { label: "Executions".to_string(), route: None },
                ]
            }

            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900", "Execution History" }
                    p { class: "text-gray-600 mt-1", "View and monitor flow execution history" }
                }
                button {
                    class: "px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200",
                    onclick: move |_| refresh(()),
                    "Refresh"
                }
            }

            if *loading.read() {
                div { class: "flex items-center justify-center h-64", "Loading..." }
            } else if executions.read().is_empty() {
                div { class: "bg-white rounded-lg shadow p-12 text-center",
                    span { class: "text-6xl mb-6 block", "⏱️" }
                    p { class: "text-gray-500", "No executions yet" }
                }
            } else {
                div { class: "bg-white rounded-lg shadow overflow-hidden",
                    table { class: "min-w-full divide-y divide-gray-200",
                        thead { class: "bg-gray-50",
                            tr {
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Execution ID" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Flow ID" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Status" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Started" }
                            }
                        }
                                        tbody { class: "bg-white divide-y divide-gray-200",
                                            for execution in executions.read().iter() {
                                                tr { class: "hover:bg-gray-50 cursor-pointer",
                                                    onclick: {
                                                        let execution_id = execution.execution_id.clone();
                                                        move |_| {
                                                            // Navigate to execution detail (if we add that route later)
                                                            // For now, just log - can add execution detail page later
                                                            log::info!("View execution: {}", execution_id);
                                                        }
                                                    },
                                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900",
                                                        "{execution.execution_id.chars().take(8).collect::<String>()}..."
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap text-sm",
                                                        Link {
                                                            to: Route::FlowDetail { id: execution.flow_id.clone() },
                                                            class: "text-primary-600 hover:text-primary-900 hover:underline",
                                                            onclick: move |e: MouseEvent| { e.stop_propagation(); },
                                                            "{execution.flow_id}"
                                                        }
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap",
                                                        StatusBadge {
                                                            status: match execution.status.as_str() {
                                                                "completed" => Status::Completed,
                                                                "failed" => Status::Failed,
                                                                "running" => Status::Running,
                                                                _ => Status::Pending,
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

