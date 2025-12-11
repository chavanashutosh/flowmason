mod app;
mod api;
mod components;
mod pages;
mod router;
mod state;

use app::App;

fn main() {
    dioxus::launch(App);
}

