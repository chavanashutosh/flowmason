use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::api::{ApiClient, Flow};
use crate::components::ui::{StatusBadge, status_badge::Status, Breadcrumbs, breadcrumbs::BreadcrumbItem};
use crate::components::ui::confirm_modal::ConfirmModal;
use crate::components::ui::icons::{Play, Edit, ArrowLeft};
use crate::router::Route;

#[component]
pub fn Flows() -> Element {
    let flows = use_signal(|| Vec::<Flow>::new());
    let loading = use_signal(|| true);
    let delete_modal_open = use_signal(|| false);
    let selected_flow_id = use_signal(|| String::new());

    use_future(move || {
        let mut flows = flows;
        let mut loading = loading;
        async move {
            match ApiClient::flows_list().await {
                Ok(data) => flows.set(data),
                Err(e) => log::error!("Failed to fetch flows: {}", e),
            }
            loading.set(false);
        }
    });

    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900", "Flows" }
                    p { class: "text-gray-600 mt-1", "Manage your automation flows" }
                }
                Link {
                    to: Route::NewFlow {},
                    button { class: "px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg",
                        "Create Flow"
                    }
                }
            }

            if *loading.read() {
                div { class: "flex items-center justify-center h-64", "Loading..." }
            } else if flows.read().is_empty() {
                div { class: "bg-white rounded-lg shadow p-12 text-center",
                    p { class: "text-gray-600 font-medium mb-2", "No flows created yet" }
                    p { class: "text-sm text-gray-500 mb-6", "Get started by creating a new flow" }
                    Link {
                        to: Route::NewFlow {},
                        button { class: "px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg",
                            "Create New Flow"
                        }
                    }
                }
            } else {
                div { class: "bg-white rounded-lg shadow overflow-hidden",
                    table { class: "min-w-full divide-y divide-gray-200",
                        thead { class: "bg-gray-50",
                            tr {
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Name" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Status" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Created" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Actions" }
                            }
                        }
                        tbody { class: "bg-white divide-y divide-gray-200",
                            for flow in flows.read().iter() {
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        div {
                                            div { class: "text-sm font-medium text-gray-900", "{flow.name}" }
                                            if let Some(desc) = &flow.description {
                                                div { class: "text-sm text-gray-500", "{desc}" }
                                            }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap",
                                        StatusBadge {
                                            status: if flow.active { Status::Active } else { Status::Inactive }
                                        }
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "{flow.created_at}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                        div { class: "flex gap-2",
                                            button {
                                                class: "flex items-center px-3 py-1.5 text-sm text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded transition-colors",
                                                onclick: {
                                                    let flow_id = flow.id.clone();
                                                    let flows = flows.clone();
                                                    move |_| {
                                                        let flow_id = flow_id.clone();
                                                        let mut flows = flows.clone();
                                                        spawn(async move {
                                                            match ApiClient::executions_execute(&flow_id, serde_json::json!({})).await {
                                                                Ok(_) => {
                                                                    // Refresh flows list
                                                                    if let Ok(updated_flows) = ApiClient::flows_list().await {
                                                                        flows.set(updated_flows);
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    log::error!("Failed to run flow: {}", e);
                                                                }
                                                            }
                                                        });
                                                    }
                                                },
                                                Play { size: 14, class: "mr-1".to_string() }
                                                span { "Run" }
                                            }
                                            Link {
                                                to: Route::FlowDetail { id: flow.id.clone() },
                                                class: "flex items-center px-3 py-1.5 text-sm text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded transition-colors",
                                                Edit { size: 14, class: "mr-1".to_string() }
                                                span { "Edit" }
                                            }
                                            button {
                                                class: "flex items-center px-3 py-1.5 text-sm text-red-600 hover:text-red-900 hover:bg-red-50 rounded transition-colors",
                                                onclick: {
                                                    let flow_id = flow.id.clone();
                                                    let mut selected_flow_id = selected_flow_id.clone();
                                                    let mut delete_modal_open = delete_modal_open.clone();
                                                    move |_| {
                                                        selected_flow_id.set(flow_id.clone());
                                                        delete_modal_open.set(true);
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ConfirmModal {
                show: *delete_modal_open.read(),
                title: "Delete Flow".to_string(),
                message: "Are you sure you want to delete this flow? This action cannot be undone.".to_string(),
                on_confirm: {
                    let flows = flows.clone();
                    let selected_flow_id = selected_flow_id.clone();
                    let mut delete_modal_open = delete_modal_open.clone();
                    move |_| {
                        let flow_id = selected_flow_id.read().clone();
                        let mut flows = flows.clone();
                        spawn(async move {
                            if let Err(e) = ApiClient::flows_delete(&flow_id).await {
                                log::error!("Failed to delete flow: {}", e);
                            } else {
                                let mut current_flows = flows.read().clone();
                                current_flows.retain(|f| f.id != flow_id);
                                flows.set(current_flows);
                            }
                        });
                        delete_modal_open.set(false);
                    }
                },
                on_cancel: {
                    let mut delete_modal_open = delete_modal_open.clone();
                    move |_| delete_modal_open.set(false)
                },
                confirm_text: "Delete".to_string(),
            }
        }
    }
}

#[component]
pub fn FlowDetail(id: String) -> Element {
    let flow = use_signal(|| Option::<Flow>::None);
    let loading = use_signal(|| true);
    let mut delete_modal_open = use_signal(|| false);
    let running = use_signal(|| false);
    let id_clone = id.clone();

    use_future(move || {
        let id = id_clone.clone();
        let mut flow = flow;
        let mut loading = loading;
        async move {
            match ApiClient::flows_get(&id).await {
                Ok(data) => flow.set(Some(data)),
                Err(e) => log::error!("Failed to fetch flow: {}", e),
            }
            loading.set(false);
        }
    });

    let delete_flow = {
        let id = id.clone();
        move |_| {
            let id = id.clone();
            spawn(async move {
                if let Err(e) = ApiClient::flows_delete(&id).await {
                    log::error!("Failed to delete flow: {}", e);
                } else {
                    // Navigate to flows list
                    #[cfg(target_arch = "wasm32")]
                    {
                        let window = web_sys::window().unwrap();
                        let _ = window.location().set_href("/flows");
                    }
                }
            });
        }
    };

    let run_flow = {
        let id = id.clone();
        let running = running.clone();
        move |_| {
            let id = id.clone();
            let mut running = running.clone();
            running.set(true);
            spawn(async move {
                match ApiClient::executions_execute(&id, serde_json::json!({})).await {
                    Ok(execution) => {
                        log::info!("Flow executed: {}", execution.execution_id);
                        // Navigate to executions page
                        #[cfg(target_arch = "wasm32")]
                        {
                            let window = web_sys::window().unwrap();
                            let _ = window.location().set_href("/executions");
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to run flow: {}", e);
                    }
                }
                running.set(false);
            });
        }
    };

    rsx! {
        div { class: "space-y-6",
            Breadcrumbs {
                items: vec![
                    BreadcrumbItem { label: "Dashboard".to_string(), route: Some(Route::Dashboard {}) },
                    BreadcrumbItem { label: "Flows".to_string(), route: Some(Route::Flows {}) },
                    BreadcrumbItem { 
                        label: flow.read().as_ref().map(|f| f.name.clone()).unwrap_or_else(|| "Flow Detail".to_string()), 
                        route: None 
                    },
                ]
            }

            div { class: "flex items-center justify-between",
                div { class: "flex items-center gap-4",
                    Link {
                        to: Route::Flows {},
                        class: "flex items-center px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors",
                        ArrowLeft { size: 16, class: "mr-2".to_string() }
                        span { "Back" }
                    }
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900", 
                            if let Some(flow_data) = flow.read().as_ref() {
                                "{flow_data.name}"
                            } else {
                                "Flow Detail"
                            }
                        }
                    }
                }
                if let Some(_) = flow.read().as_ref() {
                    div { class: "flex items-center gap-2",
                        button {
                            class: "flex items-center px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors",
                            disabled: *running.read(),
                            onclick: run_flow,
                            Play { size: 16, class: "mr-2".to_string() }
                            if *running.read() {
                                span { "Running..." }
                            } else {
                                span { "Run Flow" }
                            }
                        }
                        Link {
                            to: Route::EditFlow { id: id.clone() },
                            class: "flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors",
                            Edit { size: 16, class: "mr-2".to_string() }
                            span { "Edit" }
                        }
                        button {
                            class: "flex items-center px-4 py-2 text-sm font-medium text-red-600 bg-white border border-red-300 rounded-lg hover:bg-red-50 transition-colors",
                            onclick: move |_| delete_modal_open.set(true),
                            "Delete"
                        }
                    }
                }
            }

            if *loading.read() {
                div { class: "flex items-center justify-center h-64", "Loading..." }
            } else if let Some(flow_data) = flow.read().as_ref() {
                div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6",
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div {
                            h3 { class: "text-sm font-medium text-gray-500 mb-2", "Description" }
                            if let Some(desc) = &flow_data.description {
                                p { class: "text-gray-900", "{desc}" }
                            } else {
                                p { class: "text-gray-400 italic", "No description" }
                            }
                        }
                        div {
                            h3 { class: "text-sm font-medium text-gray-500 mb-2", "Status" }
                            StatusBadge {
                                status: if flow_data.active { Status::Active } else { Status::Inactive }
                            }
                        }
                        div {
                            h3 { class: "text-sm font-medium text-gray-500 mb-2", "Flow ID" }
                            p { class: "text-sm font-mono text-gray-900", "{id}" }
                        }
                        div {
                            h3 { class: "text-sm font-medium text-gray-500 mb-2", "Created" }
                            p { class: "text-sm text-gray-900", "{flow_data.created_at}" }
                        }
                    }
                    div { class: "mt-6 pt-6 border-t border-gray-200",
                        h3 { class: "text-sm font-medium text-gray-500 mb-4", "Bricks" }
                        if flow_data.bricks.is_empty() {
                            p { class: "text-gray-400 italic", "No bricks configured" }
                        } else {
                            div { class: "space-y-2",
                                for (idx, brick) in flow_data.bricks.iter().enumerate() {
                                    div { class: "flex items-center justify-between p-3 bg-gray-50 rounded",
                                        div {
                                            span { class: "text-sm font-medium text-gray-900", "{brick.brick_type}" }
                                        }
                                        span { class: "text-xs text-gray-500", "Brick {idx + 1}" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "bg-white rounded-lg shadow p-12 text-center",
                    p { class: "text-gray-500", "Flow not found" }
                    Link {
                        to: Route::Flows {},
                        class: "inline-block mt-4 px-4 py-2 text-sm font-medium text-primary-600 hover:text-primary-700",
                        "Back to Flows"
                    }
                }
            }

            ConfirmModal {
                show: *delete_modal_open.read(),
                title: "Delete Flow".to_string(),
                message: "Are you sure you want to delete this flow? This action cannot be undone.".to_string(),
                on_confirm: delete_flow,
                on_cancel: {
                    let mut delete_modal_open = delete_modal_open.clone();
                    move |_| delete_modal_open.set(false)
                },
                confirm_text: "Delete".to_string(),
            }
        }
    }
}

#[component]
pub fn NewFlow() -> Element {
    use crate::components::flow_builder::FlowBuilder;
    
    rsx! {
        div { class: "space-y-6",
            FlowBuilder { flow_id: None }
        }
    }
}

#[component]
pub fn EditFlow(id: String) -> Element {
    use crate::components::flow_builder::FlowBuilder;
    
    rsx! {
        div { class: "space-y-6",
            FlowBuilder { flow_id: Some(id) }
        }
    }
}

