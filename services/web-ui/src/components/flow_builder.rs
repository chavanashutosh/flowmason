use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use serde_json::Value;
use crate::api::{ApiClient, BrickTypeInfo, BrickConfig, CreateFlowRequest};
use crate::components::brick_palette::BrickPalette;
use crate::components::brick_node::BrickNode;
use crate::components::flow_metadata_form::FlowMetadataForm;

#[component]
fn BrickNodeWrapper(
    index: usize,
    brick: BrickConfig,
    schema: Option<Value>,
    on_delete: EventHandler<usize>,
    on_config_update: EventHandler<(usize, Value)>,
    on_drag_start: EventHandler<usize>,
    on_drag_over: EventHandler<usize>,
    on_drop: EventHandler<(usize, usize)>,
    is_dragging: bool,
) -> Element {
    let config_signal = use_signal(|| brick.config.clone());
    let mut last_config = use_signal(|| brick.config.clone());
    
    // Sync config changes back to parent (only when actually changed)
    use_effect(move || {
        let current_config = config_signal.read().clone();
        let last = last_config.read().clone();
        if current_config != last {
            last_config.set(current_config.clone());
            on_config_update.call((index, current_config));
        }
    });
    
    if let Some(schema_val) = schema {
        rsx! {
            BrickNode {
                index: index,
                brick_type: brick.brick_type.clone(),
                config: config_signal,
                schema: schema_val,
                on_delete: on_delete,
                on_config_update: on_config_update,
                on_drag_start: on_drag_start,
                on_drag_over: on_drag_over,
                on_drop: on_drop,
                is_dragging: is_dragging,
            }
        }
    } else {
        rsx! {
            div { class: "mb-4 p-4 bg-white border border-gray-200 rounded-lg",
                "{brick.brick_type}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FlowBuilderProps {
    #[props(default)]
    pub flow_id: Option<String>,
}

#[component]
pub fn FlowBuilder(props: FlowBuilderProps) -> Element {
    let flow_id = props.flow_id;
    let is_editing = flow_id.is_some();
    
    let flow_name = use_signal(|| String::new());
    let flow_description = use_signal(|| Option::<String>::None);
    let bricks = use_signal(|| Vec::<BrickConfig>::new());
    let available_bricks = use_signal(|| Vec::<BrickTypeInfo>::new());
    let brick_schemas = use_signal(|| std::collections::HashMap::<String, Value>::new());
    let loading = use_signal(|| true);
    let saving = use_signal(|| false);
    let drag_source = use_signal(|| Option::<usize>::None);
    let error_message = use_signal(|| Option::<String>::None);

    // Load available bricks
    use_future(move || {
        let mut available_bricks = available_bricks;
        let mut brick_schemas = brick_schemas;
        let mut loading = loading;
        async move {
            match ApiClient::bricks_list().await {
                Ok(response) => {
                    available_bricks.set(response.bricks.clone());
                    // Pre-fetch schemas
                    let mut schemas = std::collections::HashMap::new();
                    for brick in response.bricks.iter() {
                        if let Ok(schema) = ApiClient::bricks_get_schema(&brick.name).await {
                            schemas.insert(brick.name.clone(), schema);
                        }
                    }
                    brick_schemas.set(schemas);
                }
                Err(e) => {
                    log::error!("Failed to load bricks: {}", e);
                }
            }
            loading.set(false);
        }
    });

    // Load existing flow if editing
    if let Some(id) = flow_id.clone() {
        use_future(move || {
            let id = id.clone();
            let mut flow_name = flow_name;
            let mut flow_description = flow_description;
            let mut bricks = bricks;
            async move {
                match ApiClient::flows_get(&id).await {
                    Ok(flow) => {
                        flow_name.set(flow.name);
                        flow_description.set(flow.description);
                        bricks.set(flow.bricks);
                    }
                    Err(e) => {
                        log::error!("Failed to load flow: {}", e);
                    }
                }
            }
        });
    }

    let add_brick = {
        let mut bricks = bricks;
        let brick_schemas = brick_schemas;
        let available_bricks = available_bricks;
        move |brick_type: String| {
            // Find the brick info - match by name (from palette) or brick_type
            let available_bricks_read = available_bricks.read();
            let brick_info_opt = available_bricks_read.iter().find(|b| {
                b.name == brick_type || 
                b.brick_type == brick_type ||
                b.name.replace('_', "") == brick_type.replace('_', "")
            });
            
            if let Some(brick_info) = brick_info_opt {
                let schema = brick_schemas.read()
                    .get(&brick_info.name)
                    .cloned()
                    .unwrap_or_else(|| {
                        // Fallback schema
                        serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        })
                    });
                
                // Create default config based on schema
                let mut default_config = Value::Object(serde_json::Map::new());
                if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                    for (key, prop) in props.iter() {
                        if let Some(default_val) = prop.get("default") {
                            if let Some(obj) = default_config.as_object_mut() {
                                obj.insert(key.clone(), default_val.clone());
                            }
                        }
                    }
                }
                
                let mut current_bricks = bricks.read().clone();
                current_bricks.push(BrickConfig {
                    brick_type: brick_info.brick_type.clone(),
                    config: default_config,
                });
                bricks.set(current_bricks);
            }
        }
    };

    let delete_brick = {
        let mut bricks = bricks;
        move |index: usize| {
            let mut current_bricks = bricks.read().clone();
            if index < current_bricks.len() {
                current_bricks.remove(index);
                bricks.set(current_bricks);
            }
        }
    };

    let handle_drag_start = {
        let mut drag_source = drag_source;
        move |index: usize| {
            drag_source.set(Some(index));
        }
    };

    let handle_drag_over = move |_index: usize| {
        // Visual feedback handled in BrickNode
    };

    let handle_drop = {
        let mut bricks = bricks;
        let mut drag_source = drag_source;
        move |(source_idx, target_idx): (usize, usize)| {
            let mut current_bricks = bricks.read().clone();
            if source_idx < current_bricks.len() && target_idx < current_bricks.len() && source_idx != target_idx {
                let brick = current_bricks.remove(source_idx);
                current_bricks.insert(target_idx, brick);
                bricks.set(current_bricks);
            }
            drag_source.set(None);
        }
    };

    let mut canvas_drag_over = use_signal(|| false);
    
    let mut handle_canvas_drop = {
        let mut bricks = bricks;
        let brick_schemas = brick_schemas.clone();
        let available_bricks = available_bricks.clone();
        let mut canvas_drag_over = canvas_drag_over.clone();
        move |brick_type: String| {
            let available_bricks_read = available_bricks.read();
            let brick_info_opt = available_bricks_read.iter().find(|b| {
                b.name == brick_type || 
                b.brick_type == brick_type ||
                b.name.replace('_', "") == brick_type.replace('_', "")
            });
            
            if let Some(brick_info) = brick_info_opt {
                let schema = brick_schemas.read()
                    .get(&brick_info.name)
                    .cloned()
                    .unwrap_or_else(|| {
                        serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        })
                    });
                
                let mut default_config = Value::Object(serde_json::Map::new());
                if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                    for (key, prop) in props.iter() {
                        if let Some(default_val) = prop.get("default") {
                            if let Some(obj) = default_config.as_object_mut() {
                                obj.insert(key.clone(), default_val.clone());
                            }
                        }
                    }
                }
                
                let mut current_bricks = bricks.read().clone();
                current_bricks.push(BrickConfig {
                    brick_type: brick_info.brick_type.clone(),
                    config: default_config,
                });
                bricks.set(current_bricks);
            }
            canvas_drag_over.set(false);
        }
    };

    let handle_save = {
        let flow_name = flow_name.clone();
        let flow_description = flow_description.clone();
        let bricks = bricks.clone();
        let mut saving = saving.clone();
        let mut error_message = error_message.clone();
        let flow_id = flow_id.clone();
        move |_| {
            let name = flow_name.read().clone();
            if name.trim().is_empty() {
                error_message.set(Some("Flow name is required".to_string()));
                return;
            }

            let bricks_data = bricks.read().clone();
            saving.set(true);
            error_message.set(None);

            let request = CreateFlowRequest {
                name: name.clone(),
                description: flow_description.read().clone(),
                bricks: bricks_data,
            };

            let flow_id_clone = flow_id.clone();
            let mut saving_clone = saving.clone();
            let mut error_message_clone = error_message.clone();
            spawn(async move {
                let result = if let Some(id) = flow_id_clone {
                    ApiClient::flows_update(&id, request).await
                } else {
                    ApiClient::flows_create(request).await
                };

                saving_clone.set(false);
                match result {
                    Ok(_) => {
                        // Navigate to flows list
                        #[cfg(target_arch = "wasm32")]
                        {
                            let window = web_sys::window().unwrap();
                            let _ = window.location().set_href("/flows");
                        }
                    }
                    Err(e) => {
                        error_message_clone.set(Some(format!("Failed to save flow: {}", e)));
                    }
                }
            });
        }
    };

    let handle_cancel = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let _ = window.location().set_href("/flows");
        }
    };

    rsx! {
        div { class: "flex h-[calc(100vh-200px)] gap-4",
            if *loading.read() {
                div { class: "flex items-center justify-center w-full h-full",
                    "Loading bricks..."
                }
            } else {
                div {
                    // Brick Palette
                    BrickPalette {
                        available_bricks: available_bricks.read().clone(),
                        on_add_brick: add_brick,
                    }
                    
                    // Main flow builder area
                    div { class: "flex-1 flex flex-col overflow-hidden",
                        // Flow metadata form
                        FlowMetadataForm {
                            name: flow_name,
                            description: flow_description,
                            on_save: handle_save,
                            on_cancel: handle_cancel,
                            is_editing: is_editing,
                        }
                        
                        // Error message
                        if let Some(error) = error_message.read().as_ref() {
                            div { class: "mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm",
                                "{error}"
                            }
                        }
                        
                        // Flow canvas
                        div { 
                            class: format!(
                                "flex-1 bg-gray-50 rounded-lg border-2 p-6 overflow-y-auto transition-colors {}",
                                if *canvas_drag_over.read() { "border-primary-500 border-dashed bg-primary-50" } else { "border-gray-200" }
                            ),
                            ondragover: move |evt: Event<DragData>| {
                                evt.prevent_default();
                                canvas_drag_over.set(true);
                            },
                            ondragleave: move |_| {
                                canvas_drag_over.set(false);
                            },
                            ondrop: move |evt: Event<DragData>| {
                                evt.prevent_default();
                                #[cfg(target_arch = "wasm32")]
                                {
                                    use wasm_bindgen::JsCast;
                                    use web_sys::DragEvent;
                                    let web_evt = evt.as_web_event();
                                    if let Some(drag_evt) = web_evt.dyn_ref::<DragEvent>() {
                                        if let Some(data_transfer) = drag_evt.data_transfer() {
                                            if let Ok(data) = data_transfer.get_data("text/plain") {
                                                if data.starts_with("palette:") {
                                                    let brick_type = data.strip_prefix("palette:").unwrap_or(&data).to_string();
                                                    handle_canvas_drop(brick_type);
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            h3 { class: "text-sm font-medium text-gray-700 mb-4", "Flow Sequence" }
                            
                            if bricks.read().is_empty() {
                                div { class: "text-center py-12 text-gray-500",
                                    p { class: "mb-2", "No bricks added yet" }
                                    p { class: "text-sm", "Drag a brick from the palette or click to add it to your flow" }
                                }
                            } else {
                                div { class: "space-y-0",
                                    for (idx, brick) in bricks.read().iter().enumerate() {
                                        {
                                            let schema_key = available_bricks.read()
                                                .iter()
                                                .find(|b| b.brick_type == brick.brick_type || b.name == brick.brick_type)
                                                .map(|b| b.name.clone())
                                                .unwrap_or_else(|| brick.brick_type.clone());
                                            
                                            let drag_source_val = *drag_source.read();
                                            let is_dragging = drag_source_val.map(|i| i == idx).unwrap_or(false);
                                            
                                            let brick_clone = brick.clone();
                                            let schema_val = brick_schemas.read().get(&schema_key).cloned();
                                            
                                            rsx! {
                                                BrickNodeWrapper {
                                                    index: idx,
                                                    brick: brick_clone,
                                                    schema: schema_val,
                                                    on_delete: delete_brick,
                                                    on_config_update: {
                                                        let mut bricks = bricks.clone();
                                                        move |(idx, config): (usize, Value)| {
                                                            let mut current_bricks = bricks.read().clone();
                                                            if idx < current_bricks.len() {
                                                                current_bricks[idx].config = config;
                                                                bricks.set(current_bricks);
                                                            }
                                                        }
                                                    },
                                                    on_drag_start: handle_drag_start,
                                                    on_drag_over: handle_drag_over,
                                                    on_drop: handle_drop,
                                                    is_dragging: is_dragging,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Save button at bottom
                        if *saving.read() {
                            div { class: "mt-4 text-center text-gray-500",
                                "Saving..."
                            }
                        }
                    }
                }
            }
        }
    }
}
