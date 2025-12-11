use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "space-y-6",
            div {
                h1 { class: "text-3xl font-bold text-gray-900", "Settings" }
                p { class: "text-gray-600 mt-1", "Manage your account and preferences" }
            }

            div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6",
                p { class: "text-gray-500", "Settings page coming soon." }
            }
        }
    }
}

