use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn BrickConfigForm(
    schema: Value,
    config: Signal<Value>,
    brick_name: String,
) -> Element {
    let mut expanded = use_signal(|| false);
    
    let properties = schema
        .get("properties")
        .and_then(|p| p.as_object())
        .map(|obj| obj.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    
    let required_fields = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    rsx! {
        div { class: "border border-gray-200 rounded-lg",
            button {
                class: "w-full px-4 py-3 flex items-center justify-between bg-gray-50 hover:bg-gray-100 transition-colors rounded-t-lg",
                onclick: move |_| {
                    let current = *expanded.read();
                    expanded.set(!current);
                },
                div { class: "flex items-center gap-2",
                    span { class: "text-sm font-medium text-gray-900", 
                        {format!("Configure {}", format_brick_name(&brick_name))}
                    }
                }
                if *expanded.read() {
                    span { class: "text-gray-500", "▼" }
                } else {
                    span { class: "text-gray-500", "▶" }
                }
            }
            
            if *expanded.read() {
                div { class: "p-4 space-y-4 bg-white rounded-b-lg",
                    if properties.is_empty() {
                        p { class: "text-sm text-gray-500", "No configuration required for this brick." }
                    } else {
                        for (field_name, field_schema) in properties.iter() {
                            ConfigField {
                                field_name: field_name.to_string(),
                                field_schema: (*field_schema).clone(),
                                config: config,
                                is_required: required_fields.iter().any(|&r| r == field_name.as_str()),
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_select_options(options: &[Value]) -> Vec<Element> {
    let mut result = Vec::new();
    result.push(rsx! {
        option { value: "", "Select..." }
    });
    for enum_val in options.iter() {
        if let Some(val_str) = enum_val.as_str() {
            if !val_str.is_empty() {
                result.push(rsx! {
                    option { value: "{val_str}", "{val_str}" }
                });
            }
        }
    }
    result
}

#[component]
fn ConfigField(
    field_name: String,
    field_schema: Value,
    config: Signal<Value>,
    is_required: bool,
) -> Element {
    let field_type = field_schema
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("string");
    
    let description = field_schema
        .get("description")
        .and_then(|d| d.as_str());
    
    let default_value = field_schema.get("default");
    
    let current_value = config.read()
        .get(&field_name)
        .cloned()
        .or_else(|| default_value.cloned());

    // Prepare enum options with "Select..." option as first item
    let enum_options = if let Some(enum_values) = field_schema.get("enum").and_then(|e| e.as_array()) {
        let mut all_options = vec![Value::String("".to_string())];
        all_options.extend(enum_values.iter().cloned());
        Some(all_options)
    } else {
        None
    };

    rsx! {
        div {
            label { 
                class: "block text-sm font-medium text-gray-700 mb-1",
                if is_required {
                    span { class: "text-red-500", "* " }
                }
                span { "{format_field_name(&field_name)}" }
            }
            if let Some(desc) = description {
                p { class: "text-xs text-gray-500 mb-2", "{desc}" }
            }
            
            if field_type == "string" {
                if let Some(ref enum_vals) = enum_options {
                    // Enum/Select field
                    select {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500",
                        value: current_value.as_ref().and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default(),
                        onchange: move |evt| {
                            let mut config_value = config.read().clone();
                            if !config_value.is_object() {
                                config_value = Value::Object(serde_json::Map::new());
                            }
                            if let Some(obj) = config_value.as_object_mut() {
                                obj.insert(field_name.clone(), Value::String(evt.value().to_string()));
                                config.set(Value::Object(obj.clone()));
                            }
                        },
                        option { value: "", "Select..." }
                        for enum_val in enum_vals.iter() {
                            if let Some(val_str) = enum_val.as_str() {
                                if !val_str.is_empty() {
                                    option { value: "{val_str}", "{val_str}" }
                                }
                            }
                        }
                    }
                } else {
                    // Regular text input
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500",
                        r#type: "text",
                        placeholder: format_field_placeholder(&field_name),
                        value: current_value.as_ref().and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default(),
                        oninput: move |evt| {
                            let mut config_value = config.read().clone();
                            if !config_value.is_object() {
                                config_value = Value::Object(serde_json::Map::new());
                            }
                            if let Some(obj) = config_value.as_object_mut() {
                                obj.insert(field_name.clone(), Value::String(evt.value().to_string()));
                                config.set(Value::Object(obj.clone()));
                            } else {
                                let mut new_obj = serde_json::Map::new();
                                new_obj.insert(field_name.clone(), Value::String(evt.value().to_string()));
                                config.set(Value::Object(new_obj));
                            }
                        },
                    }
                    if field_name.contains("template") || field_name.contains("prompt") {
                        p { class: "text-xs text-gray-500 mt-1", 
                            "Use {{field_name}} for template variables"
                        }
                    }
                }
            } else if field_type == "number" {
                input {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500",
                    r#type: "number",
                    step: if field_schema.get("type").and_then(|t| t.as_str()) == Some("integer") { "1" } else { "any" },
                    placeholder: default_value.and_then(|v| v.as_f64().map(|n| n.to_string())).unwrap_or_default(),
                    value: current_value.and_then(|v| v.as_f64().map(|n| n.to_string())).unwrap_or_default(),
                    oninput: move |evt| {
                        if let Ok(num) = evt.value().parse::<f64>() {
                            let mut config_value = config.read().clone();
                            if !config_value.is_object() {
                                config_value = Value::Object(serde_json::Map::new());
                            }
                            if let Some(obj) = config_value.as_object_mut() {
                                if let Some(n) = serde_json::Number::from_f64(num) {
                                    obj.insert(field_name.clone(), Value::Number(n));
                                    config.set(Value::Object(obj.clone()));
                                }
                            }
                        }
                    },
                }
            } else if field_type == "boolean" {
                input {
                    class: "w-4 h-4 text-primary-600 border-gray-300 rounded focus:ring-primary-500",
                    r#type: "checkbox",
                    checked: current_value.and_then(|v| v.as_bool()).unwrap_or(false),
                    onchange: move |evt| {
                        let mut config_value = config.read().clone();
                        if !config_value.is_object() {
                            config_value = Value::Object(serde_json::Map::new());
                        }
                        if let Some(obj) = config_value.as_object_mut() {
                            obj.insert(field_name.clone(), Value::Bool(evt.checked()));
                            config.set(Value::Object(obj.clone()));
                        }
                    },
                }
            } else if field_type == "array" {
                ArrayField {
                    field_name: field_name.clone(),
                    field_schema: field_schema.clone(),
                    config: config,
                }
            } else if field_type == "object" {
                ObjectField {
                    field_name: field_name.clone(),
                    field_schema: field_schema.clone(),
                    config: config,
                }
            } else {
                textarea {
                    class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 font-mono text-sm",
                    rows: 4,
                    placeholder: "Enter JSON value",
                    value: serde_json::to_string(&current_value.unwrap_or(Value::Null)).unwrap_or_default(),
                    oninput: move |evt| {
                        if let Ok(parsed) = serde_json::from_str::<Value>(&evt.value()) {
                            let mut config_value = config.read().clone();
                            if !config_value.is_object() {
                                config_value = Value::Object(serde_json::Map::new());
                            }
                            if let Some(obj) = config_value.as_object_mut() {
                                obj.insert(field_name.clone(), parsed);
                                config.set(Value::Object(obj.clone()));
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn ArrayField(
    field_name: String,
    field_schema: Value,
    config: Signal<Value>,
) -> Element {
    let items_schema = field_schema.get("items").cloned().unwrap_or(Value::Null);
    let current_array = config.read()
        .get(&field_name)
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    rsx! {
        div { class: "space-y-2",
            for (idx, item) in current_array.iter().enumerate() {
                div { class: "flex gap-2 items-start",
                    div { class: "flex-1",
                        if items_schema.get("type").and_then(|t| t.as_str()) == Some("object") {
                            // Object array item (like mappings)
                            ObjectArrayItem {
                                item_schema: items_schema.clone(),
                                item_value: Signal::new(item.clone()),
                                on_update: {
                                    let mut config = config.clone();
                                    let field_name = field_name.clone();
                                    let idx = idx;
                                    move |new_value| {
                                        let mut config_value = config.read().clone();
                                        if !config_value.is_object() {
                                            config_value = Value::Object(serde_json::Map::new());
                                        }
                                        if let Some(obj) = config_value.as_object_mut() {
                                            if let Some(arr) = obj.get_mut(&field_name).and_then(|v| v.as_array_mut()) {
                                                arr[idx] = new_value;
                                                config.set(Value::Object(obj.clone()));
                                            }
                                        }
                                    }
                                },
                            }
                        } else {
                            input {
                                class: "w-full px-3 py-2 border border-gray-300 rounded-lg",
                                value: item.as_str().unwrap_or(""),
                                oninput: {
                                    let mut config = config.clone();
                                    let field_name = field_name.clone();
                                    let idx = idx;
                                    move |evt: Event<FormData>| {
                                        let mut config_value = config.read().clone();
                                        if !config_value.is_object() {
                                            config_value = Value::Object(serde_json::Map::new());
                                        }
                                        if let Some(obj) = config_value.as_object_mut() {
                                            if let Some(arr) = obj.get_mut(&field_name).and_then(|v| v.as_array_mut()) {
                                                arr[idx] = Value::String(evt.value().to_string());
                                                config.set(Value::Object(obj.clone()));
                                            }
                                        }
                                    }
                                },
                            }
                        }
                    }
                    button {
                        class: "px-2 py-1 text-red-600 hover:bg-red-50 rounded",
                        onclick: {
                            let mut config = config.clone();
                            let field_name = field_name.clone();
                            let idx = idx;
                            move |_| {
                                let mut config_value = config.read().clone();
                                if let Some(obj) = config_value.as_object_mut() {
                                    if let Some(arr) = obj.get_mut(&field_name).and_then(|v| v.as_array_mut()) {
                                        arr.remove(idx);
                                        config.set(Value::Object(obj.clone()));
                                    }
                                }
                            }
                        },
                        "Remove"
                    }
                }
            }
            button {
                class: "w-full px-3 py-2 text-sm text-primary-600 border border-primary-300 rounded-lg hover:bg-primary-50",
                onclick: {
                    let mut config = config.clone();
                    let field_name = field_name.clone();
                    let items_schema = items_schema.clone();
                    move |_| {
                        let mut config_value = config.read().clone();
                        if !config_value.is_object() {
                            config_value = Value::Object(serde_json::Map::new());
                        }
                        if let Some(obj) = config_value.as_object_mut() {
                            let arr = obj.entry(field_name.clone()).or_insert_with(|| Value::Array(vec![]));
                            if let Some(arr) = arr.as_array_mut() {
                                if items_schema.get("type").and_then(|t| t.as_str()) == Some("object") {
                                    arr.push(Value::Object(serde_json::Map::new()));
                                } else {
                                    arr.push(Value::String(String::new()));
                                }
                                config.set(Value::Object(obj.clone()));
                            }
                        }
                    }
                },
                "+ Add Item"
            }
        }
    }
}

#[component]
fn ObjectArrayItem(
    item_schema: Value,
    item_value: Signal<Value>,
    on_update: EventHandler<Value>,
) -> Element {
    let properties = item_schema
        .get("properties")
        .and_then(|p| p.as_object())
        .map(|obj| obj.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    
    rsx! {
        div { class: "space-y-2 p-3 border border-gray-200 rounded-lg bg-gray-50",
            for (prop_name, _prop_schema) in properties.iter() {
                div {
                    label { class: "block text-xs font-medium text-gray-700 mb-1", "{format_field_name(prop_name)}" }
                    input {
                        class: "w-full px-2 py-1 text-sm border border-gray-300 rounded",
                        value: item_value.read().get(prop_name).and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_default(),
                        oninput: {
                            let item_value = item_value.clone();
                            let prop_name = prop_name.to_string();
                            let on_update = on_update.clone();
                            move |evt: Event<FormData>| {
                                let mut obj = item_value.read().clone();
                                if !obj.is_object() {
                                    obj = Value::Object(serde_json::Map::new());
                                }
                                if let Some(obj_map) = obj.as_object_mut() {
                                    obj_map.insert(prop_name.clone(), Value::String(evt.value().to_string()));
                                    on_update.call(Value::Object(obj_map.clone()));
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ObjectField(
    field_name: String,
    field_schema: Value,
    config: Signal<Value>,
) -> Element {
    let properties = field_schema
        .get("properties")
        .and_then(|p| p.as_object())
        .map(|obj| obj.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    
    rsx! {
        div { class: "space-y-3 p-3 border border-gray-200 rounded-lg bg-gray-50",
            for (prop_name, prop_schema) in properties.iter() {
                ConfigField {
                    field_name: format!("{}.{}", field_name, prop_name),
                    field_schema: (*prop_schema).clone(),
                    config: config,
                    is_required: false,
                }
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
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_field_name(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_field_placeholder(name: &str) -> String {
    format!("Enter {}", name.replace('_', " "))
}

