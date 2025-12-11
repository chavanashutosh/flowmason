use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use crate::api::BrickTypeInfo;

#[component]
pub fn BrickPalette(
    available_bricks: Vec<BrickTypeInfo>,
    on_add_brick: EventHandler<String>,
) -> Element {
    let integration_bricks = available_bricks
        .iter()
        .filter(|b| is_integration_brick(&b.brick_type))
        .cloned()
        .collect::<Vec<_>>();
    
    let processing_bricks = available_bricks
        .iter()
        .filter(|b| !is_integration_brick(&b.brick_type))
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div { class: "w-64 bg-gray-50 border-r border-gray-200 p-4 overflow-y-auto h-full",
            h2 { class: "text-lg font-semibold text-gray-900 mb-4", "Brick Palette" }
            
            div { class: "space-y-6",
                if !integration_bricks.is_empty() {
                    div {
                        h3 { class: "text-sm font-medium text-gray-700 mb-2 uppercase tracking-wide", "Integrations" }
                        div { class: "space-y-2",
                            for brick in integration_bricks.iter() {
                                {
                                    let brick_type = brick.brick_type.clone();
                                    let brick_name = brick.name.clone();
                                    let brick_type_drag = brick_type.clone();
                                    rsx! {
                                        div {
                                            draggable: "true",
                                            class: "w-full text-left px-3 py-2 bg-white border border-gray-200 rounded-lg hover:border-primary-500 hover:bg-primary-50 transition-colors cursor-move",
                                            ondragstart: move |evt: Event<DragData>| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    use wasm_bindgen::JsCast;
                                                    use web_sys::DragEvent;
                                                    let web_evt = evt.as_web_event();
                                                    if let Some(drag_evt) = web_evt.dyn_ref::<DragEvent>() {
                                                        if let Some(data_transfer) = drag_evt.data_transfer() {
                                                            let _ = data_transfer.set_data("text/plain", &format!("palette:{}", brick_type_drag));
                                                        }
                                                    }
                                                }
                                            },
                                            onclick: move |_| {
                                                on_add_brick.call(brick_type.clone());
                                            },
                                            div {
                                                class: "flex items-center justify-between",
                                                span {
                                                    class: "text-sm font-medium text-gray-900",
                                                    "{format_brick_name(&brick_name)}"
                                                }
                                                span {
                                                    class: "text-xs text-gray-400",
                                                    "⋮⋮"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if !processing_bricks.is_empty() {
                    div {
                        h3 { class: "text-sm font-medium text-gray-700 mb-2 uppercase tracking-wide", "Processing" }
                        div { class: "space-y-2",
                            for brick in processing_bricks.iter() {
                                {
                                    let brick_type = brick.brick_type.clone();
                                    let brick_name = brick.name.clone();
                                    let brick_type_drag = brick_type.clone();
                                    rsx! {
                                        div {
                                            draggable: "true",
                                            class: "w-full text-left px-3 py-2 bg-white border border-gray-200 rounded-lg hover:border-primary-500 hover:bg-primary-50 transition-colors cursor-move",
                                            ondragstart: move |evt: Event<DragData>| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    use wasm_bindgen::JsCast;
                                                    use web_sys::DragEvent;
                                                    let web_evt = evt.as_web_event();
                                                    if let Some(drag_evt) = web_evt.dyn_ref::<DragEvent>() {
                                                        if let Some(data_transfer) = drag_evt.data_transfer() {
                                                            let _ = data_transfer.set_data("text/plain", &format!("palette:{}", brick_type_drag));
                                                        }
                                                    }
                                                }
                                            },
                                            onclick: move |_| {
                                                on_add_brick.call(brick_type.clone());
                                            },
                                            div {
                                                class: "flex items-center justify-between",
                                                span {
                                                    class: "text-sm font-medium text-gray-900",
                                                    "{format_brick_name(&brick_name)}"
                                                }
                                                span {
                                                    class: "text-xs text-gray-400",
                                                    "⋮⋮"
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

fn is_integration_brick(brick_type: &str) -> bool {
    matches!(
        brick_type,
        "openai" | "nvidia" | "hubspot" | "notion" | "odoo" | "n8n"
    )
}

fn format_brick_name(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

