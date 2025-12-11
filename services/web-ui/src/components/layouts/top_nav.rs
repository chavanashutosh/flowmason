use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::Route;
use crate::components::ui::icons::{User, Settings, LogOut, Plus};

#[component]
pub fn TopNav() -> Element {
    let mut menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        let current = *menu_open.read();
        menu_open.set(!current);
    };

    let close_menu = move |_| {
        menu_open.set(false);
    };

    rsx! {
        nav {
            class: "fixed top-0 right-0 left-64 z-30 bg-white border-b border-gray-200 h-16 flex items-center px-8",
            div { class: "flex-1 flex items-center justify-between",
                div { class: "flex items-center",
                    h2 { class: "text-xl font-semibold text-gray-900", "FlowMason" }
                }
                div { class: "flex items-center space-x-4",
                    Link {
                        to: Route::NewFlow {},
                        class: "hidden md:flex items-center px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors",
                        Plus { size: 16, class: "mr-2".to_string() }
                        span { "Create Flow" }
                    }
                    div { class: "relative",
                        button {
                            class: "flex items-center justify-center w-10 h-10 rounded-full bg-gray-100 hover:bg-gray-200 transition-colors text-gray-700",
                            onclick: toggle_menu,
                            User { size: 20, class: "".to_string() }
                        }
                        if *menu_open.read() {
                            div {
                                id: "user-menu-dropdown",
                                class: "absolute right-0 top-12 mt-2 w-48 bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-50",
                                onclick: move |e: MouseEvent| { e.stop_propagation(); },
                                Link {
                                    to: Route::Dashboard {},
                                    class: "flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                    onclick: close_menu,
                                    User { size: 16, class: "mr-3".to_string() }
                                    span { "Profile" }
                                }
                                Link {
                                    to: Route::Settings {},
                                    class: "flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                    onclick: close_menu,
                                    Settings { size: 16, class: "mr-3".to_string() }
                                    span { "Settings" }
                                }
                                div { class: "border-t border-gray-200 my-1" }
                                button {
                                    class: "w-full flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 text-left",
                                    onclick: close_menu,
                                    LogOut { size: 16, class: "mr-3".to_string() }
                                    span { "Logout" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

