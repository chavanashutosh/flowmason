pub mod components;

use askama::Template;

#[derive(Template)]
#[template(path = "base.html", escape = "none")]
pub struct BaseTemplate {
    pub title: String,
    pub content: String,
    pub current_path: String,
}

pub use components::*;
