use dioxus::prelude::*;
use crate::api::ApiClient;

#[derive(Clone, Debug)]
struct Template {
    name: String,
    description: String,
    category: String,
    flow: TemplateFlow,
}

#[derive(Clone, Debug)]
struct TemplateFlow {
    name: String,
    description: String,
    bricks: Vec<serde_json::Value>,
}

#[component]
pub fn Templates() -> Element {
    let templates = use_signal(|| vec![
        Template {
            name: "Customer Data Processing".to_string(),
            description: "Map customer data and process with AI".to_string(),
            category: "Data Processing".to_string(),
            flow: TemplateFlow {
                name: "Customer Data Processing".to_string(),
                description: "Process customer data with field mapping and AI".to_string(),
                bricks: vec![],
            },
        },
        Template {
            name: "HubSpot Deal Creation".to_string(),
            description: "Create deals in HubSpot from form submissions".to_string(),
            category: "CRM Integration".to_string(),
            flow: TemplateFlow {
                name: "HubSpot Deal Creation".to_string(),
                description: "Automatically create deals in HubSpot".to_string(),
                bricks: vec![],
            },
        },
    ]);

    let loading_template = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "space-y-6",
            div {
                h1 { class: "text-3xl font-bold text-gray-900", "Templates" }
                p { class: "text-gray-600 mt-1", "Choose a template to get started" }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                for template in templates.read().iter() {
                    div { class: "bg-white rounded-lg shadow p-6 hover:shadow-lg transition-shadow",
                        div { class: "flex items-center justify-between mb-4",
                            h3 { class: "text-lg font-semibold text-gray-900", "{template.name}" }
                            span { class: "px-2 py-1 text-xs font-semibold rounded bg-blue-100 text-blue-800",
                                "{template.category}"
                            }
                        }
                        p { class: "text-gray-600 text-sm mb-4", "{template.description}" }
                        button {
                            class: "w-full px-4 py-2 text-sm font-medium text-white bg-gradient-to-r from-purple-500 to-blue-500 rounded-lg hover:from-purple-600 hover:to-blue-600",
                            disabled: loading_template.read().is_some(),
                            onclick: {
                                let template_clone = template.clone();
                                let mut loading_template = loading_template.clone();
                                let template_name = template.name.clone();
                                move |_| {
                                    loading_template.set(Some(template_name.clone()));
                                    let template = template_clone.clone();
                                    let mut loading_template = loading_template.clone();
                                    spawn(async move {
                                        match ApiClient::flows_create(crate::api::CreateFlowRequest {
                                            name: template.flow.name,
                                            description: Some(template.flow.description),
                                            bricks: template.flow.bricks.into_iter().map(|b| {
                                                crate::api::BrickConfig {
                                                    brick_type: b.get("brick_type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                                    config: b.get("config").cloned().unwrap_or_default(),
                                                }
                                            }).collect(),
                                        }).await {
                                            Ok(_) => {
                                                // Navigate to flows page
                                            }
                                            Err(e) => {
                                                log::error!("Failed to create flow from template: {}", e);
                                            }
                                        }
                                        loading_template.set(None);
                                    });
                                }
                            },
                            if loading_template.read().as_ref().map(|s| s == &template.name).unwrap_or(false) {
                                "Loading..."
                            } else {
                                "Use Template"
                            }
                        }
                    }
                }
            }
        }
    }
}

