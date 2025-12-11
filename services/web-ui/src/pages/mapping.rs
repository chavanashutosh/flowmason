use dioxus::prelude::*;

#[derive(Clone, Debug)]
struct MappingRule {
    source: String,
    target: String,
}

#[component]
pub fn Mapping() -> Element {
    let mut input_json = use_signal(|| r#"{
  "user": {
    "name": "John Doe",
    "age": 30
  }
}"#.to_string());
    let mut output_json = use_signal(|| "{}".to_string());
    let mut mapping_rules = use_signal(|| Vec::<MappingRule>::new());

    let add_rule = move |_| {
        mapping_rules.write().push(MappingRule {
            source: String::new(),
            target: String::new(),
        });
    };

    let mut update_rule = move |index: usize, field: &str, value: String| {
        let mut rules = mapping_rules.write();
        if let Some(rule) = rules.get_mut(index) {
            match field {
                "source" => rule.source = value,
                "target" => rule.target = value,
                _ => {}
            }
        }
    };

    let mut remove_rule = move |index: usize| {
        mapping_rules.write().remove(index);
    };

    let apply_mapping = move |_| {
        match serde_json::from_str::<serde_json::Value>(&input_json.read()) {
            Ok(input) => {
                let mut output = serde_json::Map::new();
                for rule in mapping_rules.read().iter() {
                    if !rule.source.is_empty() && !rule.target.is_empty() {
                        if let Some(value) = get_nested_value(&input, &rule.source) {
                            set_nested_value(&mut output, &rule.target, value);
                        }
                    }
                }
                output_json.set(serde_json::to_string_pretty(&serde_json::Value::Object(output)).unwrap_or_default());
            }
            Err(_) => {
                // In a real app, show an error message
            }
        }
    };

    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center gap-3",
                span { class: "text-3xl", "🔗" }
                div {
                    h1 { class: "text-3xl font-bold text-gray-900", "Field Mapping Builder" }
                    p { class: "text-gray-600 mt-1", "Build dynamic field mappings between data sources" }
                }
            }

            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                div { class: "bg-white rounded-lg shadow p-6",
                    h2 { class: "text-xl font-semibold text-gray-900 mb-6", "Input JSON" }
                    textarea {
                        class: "w-full font-mono text-sm border border-gray-300 rounded p-3",
                        rows: 12,
                        value: "{input_json.read()}",
                        oninput: move |e| input_json.set(e.value()),
                        placeholder: "Enter input JSON...",
                    }
                }

                div { class: "bg-white rounded-lg shadow p-6",
                    h2 { class: "text-xl font-semibold text-gray-900 mb-6", "Output JSON" }
                    textarea {
                        class: "w-full font-mono text-sm bg-gray-50 border border-gray-300 rounded p-3",
                        rows: 12,
                        readonly: true,
                        value: "{output_json.read()}",
                        placeholder: "Output will appear here...",
                    }
                }
            }

            div { class: "bg-white rounded-lg shadow p-6",
                div { class: "flex items-center justify-between mb-6",
                    h2 { class: "text-xl font-semibold text-gray-900", "Mapping Rules" }
                    button {
                        class: "px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200",
                        onclick: add_rule,
                        "Add Rule"
                    }
                }

                if mapping_rules.read().is_empty() {
                    div { class: "text-center py-12 text-gray-500",
                        p { "No mapping rules yet. Click \"Add Rule\" to create one." }
                    }
                } else {
                    div { class: "space-y-6",
                        for (index, rule) in mapping_rules.read().iter().enumerate() {
                            div { class: "bg-gray-50 rounded-lg p-4",
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Source Path" }
                                        input {
                                            class: "w-full px-3 py-2 border border-gray-300 rounded",
                                            r#type: "text",
                                            value: "{rule.source}",
                                            oninput: move |e| update_rule(index, "source", e.value()),
                                            placeholder: "user.name",
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-2", "Target Path" }
                                        div { class: "flex gap-3",
                                            input {
                                                class: "flex-1 px-3 py-2 border border-gray-300 rounded",
                                                r#type: "text",
                                                value: "{rule.target}",
                                                oninput: move |e| update_rule(index, "target", e.value()),
                                                placeholder: "customer.name",
                                            }
                                            button {
                                                class: "px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-lg hover:bg-red-700",
                                                onclick: move |_| remove_rule(index),
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "mt-6 flex justify-end",
                    button {
                        class: "px-4 py-2 text-sm font-medium text-white bg-gradient-to-r from-purple-500 to-blue-500 rounded-lg hover:from-purple-600 hover:to-blue-600",
                        onclick: apply_mapping,
                        "Apply Mapping"
                    }
                }
            }
        }
    }
}

fn get_nested_value(obj: &serde_json::Value, path: &str) -> Option<serde_json::Value> {
    path.split('.').try_fold(obj, |current, key| {
        current.get(key)
    }).cloned()
}

fn set_nested_value(obj: &mut serde_json::Map<String, serde_json::Value>, path: &str, value: serde_json::Value) {
    let mut keys: Vec<&str> = path.split('.').collect();
    let last_key = keys.pop().unwrap();
    let target = keys.iter().fold(obj, |current, key| {
        current.entry(key.to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .unwrap()
    });
    target.insert(last_key.to_string(), value);
}

