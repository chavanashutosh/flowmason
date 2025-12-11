use dioxus::prelude::*;
use crate::api::ApiClient;

#[component]
pub fn Metering() -> Element {
    let stats = use_signal(|| Vec::<UsageStat>::new());
    let loading = use_signal(|| true);

    use_future(move || {
        let mut stats = stats;
        let mut loading = loading;
        async move {
            match ApiClient::usage_get_stats().await {
                Ok(data) => {
                    if let Some(array) = data.as_array() {
                        let parsed: Vec<UsageStat> = array
                            .iter()
                            .filter_map(|v| serde_json::from_value(v.clone()).ok())
                            .collect();
                        stats.set(parsed);
                    }
                }
                Err(e) => {
                    log::error!("Failed to fetch usage stats: {}", e);
                }
            }
            loading.set(false);
        }
    });

    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center gap-3",
                span { class: "text-3xl", "📈" }
                div {
                    h1 { class: "text-3xl font-bold text-gray-900", "Usage & Metering" }
                    p { class: "text-gray-600 mt-1", "Monitor usage and quotas for each brick type" }
                }
            }

            if *loading.read() {
                div { class: "flex items-center justify-center h-64",
                    "Loading..."
                }
            } else if stats.read().is_empty() {
                div { class: "bg-white rounded-lg shadow p-12 text-center",
                    span { class: "text-6xl mb-6 block", "📈" }
                    p { class: "text-gray-500", "No usage data available" }
                }
            } else {
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    for stat in stats.read().iter() {
                        UsageCard { stat: stat.clone() }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
struct UsageStat {
    brick_type: String,
    daily_usage: u32,
    daily_limit: u32,
    monthly_usage: Option<u32>,
    monthly_limit: Option<u32>,
}

#[component]
fn UsageCard(stat: UsageStat) -> Element {
    let daily_percentage = ((stat.daily_usage as f64 / stat.daily_limit as f64) * 100.0).min(100.0);
    let monthly_percentage = if let (Some(usage), Some(limit)) = (stat.monthly_usage, stat.monthly_limit) {
        ((usage as f64 / limit as f64) * 100.0).min(100.0)
    } else {
        0.0
    };

    let progress_color = if daily_percentage >= 90.0 {
        "bg-red-600"
    } else if daily_percentage >= 70.0 {
        "bg-yellow-600"
    } else {
        "bg-green-600"
    };

    rsx! {
        div { class: "bg-white rounded-lg shadow p-6",
            div { class: "flex items-center justify-between mb-6",
                h3 { class: "text-lg font-semibold text-gray-900 capitalize",
                    "{stat.brick_type.replace('_', \" \")}"
                }
                span { class: "px-2 py-1 text-xs font-semibold rounded bg-gray-100 text-gray-800",
                    "{stat.daily_usage} / {stat.daily_limit}"
                }
            }

            div { class: "space-y-6",
                div {
                    div { class: "flex justify-between text-sm mb-3",
                        span { class: "text-gray-600", "Daily Usage" }
                        span { class: "font-medium text-gray-900",
                            "{stat.daily_usage} / {stat.daily_limit}"
                        }
                    }
                    div { class: "w-full bg-gray-200 rounded-full h-2.5",
                        div {
                            class: "{progress_color} h-2.5 rounded-full",
                            style: "width: {daily_percentage}%"
                        }
                    }
                }

                if stat.monthly_limit.is_some() {
                    div {
                        div { class: "flex justify-between text-sm mb-3",
                            span { class: "text-gray-600", "Monthly Usage" }
                            span { class: "font-medium text-gray-900",
                                "{stat.monthly_usage.unwrap_or(0)} / {stat.monthly_limit.unwrap_or(0)}"
                            }
                        }
                        div { class: "w-full bg-gray-200 rounded-full h-2.5",
                            div {
                                class: "{progress_color} h-2.5 rounded-full",
                                style: "width: {monthly_percentage}%"
                            }
                        }
                    }
                }
            }
        }
    }
}

