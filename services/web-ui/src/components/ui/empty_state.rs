use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::components::ui::icons::{Clock, Play};
use crate::router::Route;

#[derive(Props, PartialEq, Clone)]
pub struct EmptyStateProps {
    pub title: String,
    pub description: String,
    pub action_label: Option<String>,
    pub action_route: Option<Route>,
    pub action_onclick: Option<EventHandler<MouseEvent>>,
    pub icon: EmptyStateIcon,
}

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum EmptyStateIcon {
    Clock,
    Play,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    let onclick_handler = props.action_onclick.clone();
    
    rsx! {
        div { class: "flex flex-col items-center justify-center py-12 px-4 text-center",
            div { class: "mb-4 text-gray-400",
                match props.icon {
                    EmptyStateIcon::Clock => rsx! { Clock { size: 48, class: "text-gray-300".to_string() } },
                    EmptyStateIcon::Play => rsx! { Play { size: 48, class: "text-gray-300".to_string() } },
                }
            }
            h3 { class: "text-lg font-semibold text-gray-900 mb-2", "{props.title}" }
            p { class: "text-sm text-gray-500 mb-6 max-w-sm", "{props.description}" }
            if let Some(label) = &props.action_label {
                if let Some(route) = &props.action_route {
                    Link {
                        to: route.clone(),
                        class: "px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors inline-block",
                        "{label}"
                    }
                } else if let Some(handler) = onclick_handler {
                    button {
                        class: "px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors",
                        onclick: move |e| handler.call(e),
                        "{label}"
                    }
                }
            }
        }
    }
}

