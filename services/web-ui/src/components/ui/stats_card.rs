use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StatsCardProps {
    pub title: String,
    pub value: String,
    pub icon: Option<Element>,
    pub trend: Option<Trend>,
}

#[derive(PartialEq, Clone)]
pub struct Trend {
    pub value: String,
    pub is_positive: bool,
}

#[component]
pub fn StatsCard(props: StatsCardProps) -> Element {
    rsx! {
        div { class: "h-full bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow p-6",
            div { class: "flex items-start justify-between",
                div { class: "flex-1",
                    p { class: "text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2", "{props.title}" }
                    p { class: "text-5xl font-bold text-gray-900", "{props.value}" }
                    if let Some(trend) = &props.trend {
                        p {
                            class: if trend.is_positive { "text-sm mt-3 font-medium text-green-600" } else { "text-sm mt-3 font-medium text-red-600" },
                            if trend.is_positive { "↑" } else { "↓" }
                            " {trend.value}"
                        }
                    }
                }
                if let Some(icon) = props.icon {
                    div { class: "p-3 rounded-lg bg-primary-50 text-primary-600 ml-4 flex-shrink-0",
                        {icon}
                    }
                }
            }
        }
    }
}

