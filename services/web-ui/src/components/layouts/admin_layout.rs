use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::layouts::sidebar::Sidebar;
use crate::components::layouts::top_nav::TopNav;
use crate::router::Route;

#[component]
pub fn AdminLayout() -> Element {
    rsx! {
        div { class: "flex h-screen bg-gray-50",
            Sidebar {}
            
            div { class: "flex-1 flex flex-col overflow-hidden lg:ml-64",
                TopNav {}
                
                main { class: "flex-1 overflow-y-auto pt-16 px-8 py-8",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

