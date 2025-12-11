use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ConfirmModalProps {
    pub show: bool,
    pub title: String,
    pub message: String,
    pub on_confirm: EventHandler,
    pub on_cancel: EventHandler,
    #[props(default = "Confirm".to_string())]
    pub confirm_text: String,
    #[props(default = "Cancel".to_string())]
    pub cancel_text: String,
}

#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    
    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            onclick: move |_| props.on_cancel.call(()),
            div {
                class: "bg-white rounded-lg p-6 max-w-md w-full",
                onclick: |e| e.stop_propagation(),
                h3 { class: "text-lg font-semibold text-gray-900 mb-4", "{props.title}" }
                p { class: "text-gray-600 mb-6", "{props.message}" }
                div { class: "flex justify-end gap-3",
                    button {
                        class: "px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200",
                        onclick: move |_| props.on_cancel.call(()),
                        "{props.cancel_text}"
                    }
                    button {
                        class: "px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-lg hover:bg-red-700",
                        onclick: move |_| props.on_confirm.call(()),
                        "{props.confirm_text}"
                    }
                }
            }
        }
    }
}

