use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::Route;
use crate::components::ui::icons::{LayoutDashboard, Workflow, Sparkles, Clock, Calendar, TrendingUp, LinkIcon, BookOpen, Plus};

#[component]
pub fn Sidebar() -> Element {
    let menu_items = vec![
        MenuItem { href: Route::Dashboard {}, label: "Dashboard", icon: MenuIcon::Dashboard },
        MenuItem { href: Route::Flows {}, label: "Flows", icon: MenuIcon::Flows },
        MenuItem { href: Route::Templates {}, label: "Templates", icon: MenuIcon::Templates },
        MenuItem { href: Route::Executions {}, label: "Executions", icon: MenuIcon::Executions },
        MenuItem { href: Route::Scheduler {}, label: "Scheduler", icon: MenuIcon::Scheduler },
        MenuItem { href: Route::Metering {}, label: "Metering", icon: MenuIcon::Metering },
        MenuItem { href: Route::Mapping {}, label: "Mapping", icon: MenuIcon::Mapping },
        MenuItem { href: Route::Documentation {}, label: "Documentation", icon: MenuIcon::Documentation },
    ];

    rsx! {
        aside {
            class: "fixed left-0 top-0 z-40 h-screen transition-transform -translate-x-full lg:translate-x-0 bg-white border-r border-gray-200 w-64",
            "aria-label": "Sidebar navigation",
            
            div { class: "px-6 py-4 border-b border-gray-200",
                Link { to: Route::Dashboard {},
                    div { class: "flex items-center",
                        span { class: "text-lg font-semibold text-gray-900", "FlowMason" }
                    }
                }
            }
            
            nav { class: "px-4 py-4 space-y-1",
                // Quick Actions Section
                div { class: "mb-4 pb-4 border-b border-gray-200",
                    Link {
                        to: Route::NewFlow {},
                        class: "flex items-center justify-center w-full px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors mb-2",
                        Plus { size: 16, class: "mr-2".to_string() }
                        span { "Create Flow" }
                    }
                    Link {
                        to: Route::Templates {},
                        class: "flex items-center justify-center w-full px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors",
                        Sparkles { size: 16, class: "mr-2 text-gray-600".to_string() }
                        span { "Browse Templates" }
                    }
                }
                
                // Main Navigation
                for item in menu_items.iter() {
                    Link {
                        to: item.href.clone(),
                        class: "flex items-center px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-100 transition-colors",
                        div { class: "mr-3 text-gray-500",
                            match item.icon {
                                MenuIcon::Dashboard => rsx! { LayoutDashboard { size: 20, class: "".to_string() } },
                                MenuIcon::Flows => rsx! { Workflow { size: 20, class: "".to_string() } },
                                MenuIcon::Templates => rsx! { Sparkles { size: 20, class: "".to_string() } },
                                MenuIcon::Executions => rsx! { Clock { size: 20, class: "".to_string() } },
                                MenuIcon::Scheduler => rsx! { Calendar { size: 20, class: "".to_string() } },
                                MenuIcon::Metering => rsx! { TrendingUp { size: 20, class: "".to_string() } },
                                MenuIcon::Mapping => rsx! { LinkIcon { size: 20, class: "".to_string() } },
                                MenuIcon::Documentation => rsx! { BookOpen { size: 20, class: "".to_string() } },
                            }
                        }
                        span { "{item.label}" }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
struct MenuItem {
    href: Route,
    label: &'static str,
    icon: MenuIcon,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum MenuIcon {
    Dashboard,
    Flows,
    Templates,
    Executions,
    Scheduler,
    Metering,
    Mapping,
    Documentation,
}

