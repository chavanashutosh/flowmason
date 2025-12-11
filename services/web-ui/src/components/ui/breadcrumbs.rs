use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::router::Route;
use crate::components::ui::icons::ChevronRight;

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbItem {
    pub label: String,
    pub route: Option<Route>,
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbsProps {
    pub items: Vec<BreadcrumbItem>,
}

#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
    rsx! {
        nav { class: "flex items-center space-x-2 text-sm text-gray-500 mb-6",
            for (index, item) in props.items.iter().enumerate() {
                if index > 0 {
                    ChevronRight { size: 16, class: "text-gray-400".to_string() }
                }
                if let Some(route) = &item.route {
                    Link {
                        to: route.clone(),
                        class: "hover:text-gray-700 transition-colors",
                        span { "{item.label}" }
                    }
                } else {
                    span { class: "text-gray-900 font-medium", "{item.label}" }
                }
            }
        }
    }
}

