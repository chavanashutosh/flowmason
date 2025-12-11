use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Status {
    Completed,
    Failed,
    Running,
    Pending,
    Active,
    Inactive,
}

#[derive(Props, PartialEq, Clone)]
pub struct StatusBadgeProps {
    pub status: Status,
}

#[component]
pub fn StatusBadge(props: StatusBadgeProps) -> Element {
    let (color_class, label) = match props.status {
        Status::Completed | Status::Active => ("bg-green-100 text-green-800", "Completed"),
        Status::Failed => ("bg-red-100 text-red-800", "Failed"),
        Status::Running => ("bg-blue-100 text-blue-800", "Running"),
        Status::Pending => ("bg-yellow-100 text-yellow-800", "Pending"),
        Status::Inactive => ("bg-gray-100 text-gray-800", "Inactive"),
    };
    
    rsx! {
        span { class: "px-2 py-1 text-xs font-semibold rounded {color_class}",
            "{label}"
        }
    }
}

