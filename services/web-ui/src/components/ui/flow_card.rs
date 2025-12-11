use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FlowCardProps {
    pub title: String,
    pub children: Element,
    #[props(default = "".to_string())]
    pub class: String,
    pub actions: Option<Element>,
}

#[component]
pub fn FlowCard(props: FlowCardProps) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow p-6 {props.class}",
            div { class: "flex items-center justify-between mb-4",
                h3 { class: "text-lg font-semibold text-gray-900", "{props.title}" }
                if let Some(actions) = &props.actions {
                    {actions}
                }
            }
            {props.children}
        }
    }
}

