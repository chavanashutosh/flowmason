use dioxus::prelude::*;

#[component]
pub fn FlowMetadataForm(
    name: Signal<String>,
    description: Signal<Option<String>>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
    is_editing: bool,
) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6",
            h2 { class: "text-lg font-semibold text-gray-900 mb-4", 
                if is_editing { "Edit Flow" } else { "Flow Details" }
            }
            
            div { class: "space-y-4",
                div {
                    label { 
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Flow Name"
                    }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500",
                        r#type: "text",
                        placeholder: "Enter flow name",
                        value: "{name.read()}",
                        oninput: move |evt| name.set(evt.value().clone()),
                    }
                }
                
                div {
                    label { 
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Description (Optional)"
                    }
                    textarea {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500",
                        rows: 3,
                        placeholder: "Enter flow description",
                        value: description.read().as_ref().map(|s| s.as_str()).unwrap_or(""),
                        oninput: move |evt| {
                            let value = evt.value();
                            description.set(if value.is_empty() { None } else { Some(value) });
                        },
                    }
                }
            }
            
            div { class: "flex justify-end gap-3 mt-6",
                button {
                    class: "px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
                button {
                    class: "px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors",
                    onclick: move |_| on_save.call(()),
                    if is_editing { "Update Flow" } else { "Create Flow" }
                }
            }
        }
    }
}

