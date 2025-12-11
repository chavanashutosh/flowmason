use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use serde_json::Value;
use crate::components::brick_config_form::BrickConfigForm;

#[component]
pub fn BrickNode(
    index: usize,
    brick_type: String,
    config: Signal<Value>,
    schema: Value,
    on_delete: EventHandler<usize>,
    #[props(default)]
    on_config_update: EventHandler<(usize, Value)>,
    on_drag_start: EventHandler<usize>,
    on_drag_over: EventHandler<usize>,
    on_drop: EventHandler<(usize, usize)>,
    is_dragging: bool,
) -> Element {
    let mut drag_over = use_signal(|| false);
    
    rsx! {
        div {
            class: if *drag_over.read() {
                "relative mb-4 transition-all"
            } else {
                "relative mb-4"
            },
            draggable: "true",
            ondragstart: move |_evt: Event<DragData>| {
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::JsCast;
                    use web_sys::DragEvent;
                    let web_evt = _evt.as_web_event();
                    if let Some(drag_evt) = web_evt.dyn_ref::<DragEvent>() {
                        if let Some(data_transfer) = drag_evt.data_transfer() {
                            let _ = data_transfer.set_data("text/plain", &index.to_string());
                        }
                    }
                }
                on_drag_start.call(index);
            },
            ondragover: move |evt: Event<DragData>| {
                evt.prevent_default();
                drag_over.set(true);
                on_drag_over.call(index);
            },
            ondragleave: move |_| {
                drag_over.set(false);
            },
            ondrop: move |evt: Event<DragData>| {
                evt.prevent_default();
                drag_over.set(false);
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::JsCast;
                    use web_sys::DragEvent;
                    let web_evt = evt.as_web_event();
                    if let Some(drag_evt) = web_evt.dyn_ref::<DragEvent>() {
                        if let Some(data_transfer) = drag_evt.data_transfer() {
                            if let Ok(data) = data_transfer.get_data("text/plain") {
                                if let Ok(source_idx) = data.parse::<usize>() {
                                    if source_idx != index {
                                        on_drop.call((source_idx, index));
                                    }
                                }
                            }
                        }
                    }
                }
            },
            
            // Connector line above (except first brick)
            if index > 0 {
                div { 
                    class: "absolute left-6 top-0 w-0.5 h-4 bg-gray-300 -translate-y-full",
                }
            }
            
            // Brick card
            div { 
                class: format!(
                    "bg-white border-2 rounded-lg shadow-sm transition-all {} {}",
                    if is_dragging { "opacity-50 border-primary-500" } else { "border-gray-200" },
                    if *drag_over.read() { "border-primary-500 bg-primary-50" } else { "" }
                ),
                div { class: "flex items-start gap-3 p-4",
                    // Drag handle
                    div { 
                        class: "cursor-move text-gray-400 hover:text-gray-600 mt-1",
                        "⋮⋮"
                    }
                    
                    // Brick content
                    div { class: "flex-1",
                        div { class: "flex items-center justify-between mb-2",
                            div { class: "flex items-center gap-2",
                                div { 
                                    class: "w-2 h-2 rounded-full bg-primary-500",
                                }
                                span { class: "text-sm font-semibold text-gray-900", 
                                    "{format_brick_name(&brick_type)}"
                                }
                            }
                            button {
                                class: "text-red-600 hover:text-red-800 hover:bg-red-50 px-2 py-1 rounded transition-colors",
                                onclick: move |_| on_delete.call(index),
                                "Delete"
                            }
                        }
                        
                        BrickConfigForm {
                            schema: schema.clone(),
                            config: config,
                            brick_name: brick_type.clone(),
                        }
                    }
                }
            }
            
            // Connector line below
            div { 
                class: "absolute left-6 bottom-0 w-0.5 h-4 bg-gray-300 translate-y-full",
            }
        }
    }
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

