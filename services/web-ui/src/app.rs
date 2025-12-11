use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::router::Route;

#[allow(non_snake_case)]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

