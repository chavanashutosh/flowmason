use dioxus::prelude::*;
use crate::components::ui::icons::X;

const ONBOARDING_DISMISSED_KEY: &str = "flowmason_onboarding_dismissed";

#[component]
pub fn OnboardingPanel() -> Element {
    let mut is_dismissed = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().ok().flatten();
            if let Some(storage) = storage {
                storage.get_item(ONBOARDING_DISMISSED_KEY).ok().flatten()
                    .map(|v| v == "true")
                    .unwrap_or(false)
            } else {
                false
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    });

    let dismiss = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(ONBOARDING_DISMISSED_KEY, "true");
            }
        }
        is_dismissed.set(true);
    };

    if *is_dismissed.read() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "bg-gray-50 border border-gray-200 rounded-lg p-4 mb-6 flex items-start justify-between",
            div { class: "flex-1",
                p { class: "text-sm text-gray-700", "Create a flow or browse templates to get started." }
            }
            button {
                class: "ml-4 text-gray-400 hover:text-gray-600 transition-colors flex-shrink-0",
                onclick: dismiss,
                X { size: 20, class: "".to_string() }
            }
        }
    }
}

