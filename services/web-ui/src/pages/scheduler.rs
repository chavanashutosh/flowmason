use dioxus::prelude::*;
use crate::api::{ApiClient, ScheduledFlow, Flow};

#[component]
pub fn Scheduler() -> Element {
    let scheduled_flows = use_signal(|| Vec::<ScheduledFlow>::new());
    let flows = use_signal(|| Vec::<Flow>::new());
    let loading = use_signal(|| true);
    let mut modal_open = use_signal(|| false);
    let mut selected_flow_id = use_signal(|| String::new());
    let mut cron_expression = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut saving = use_signal(|| false);

    let fetch_data = move || {
        let mut scheduled_flows = scheduled_flows;
        let mut flows = flows;
        let mut loading = loading;
        spawn(async move {
            match futures::join!(
                ApiClient::scheduler_list_scheduled_flows(),
                ApiClient::flows_list()
            ) {
                (Ok(scheduled), Ok(all_flows)) => {
                    scheduled_flows.set(scheduled);
                    flows.set(all_flows);
                }
                _ => {
                    log::error!("Failed to fetch scheduler data");
                }
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_data();
    });

    let handle_schedule = move |_| {
        if selected_flow_id.read().is_empty() || cron_expression.read().trim().is_empty() {
            error.set(Some("Please select a flow and enter a cron expression".to_string()));
            return;
        }

        saving.set(true);
        error.set(None);

        let flow_id = selected_flow_id.read().clone();
        let cron = cron_expression.read().clone();
        let mut scheduled_flows = scheduled_flows;
        let mut modal_open = modal_open;
        let mut saving = saving;
        let mut error = error;

        spawn(async move {
            match ApiClient::scheduler_schedule_flow(&flow_id, &cron).await {
                Ok(scheduled) => {
                    scheduled_flows.write().push(scheduled);
                    modal_open.set(false);
                    selected_flow_id.set(String::new());
                    cron_expression.set(String::new());
                }
                Err(e) => {
                    error.set(Some(format!("Failed to schedule flow: {}", e)));
                }
            }
            saving.set(false);
        });
    };

    rsx! {
        div { class: "space-y-6",
            div { class: "flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900", "Scheduler" }
                    p { class: "text-gray-600 mt-1", "Manage scheduled flows" }
                }
                button {
                    class: "px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg",
                    onclick: move |_| modal_open.set(true),
                    "Schedule Flow"
                }
            }

            if *loading.read() {
                div { class: "flex items-center justify-center h-64", "Loading..." }
            } else if scheduled_flows.read().is_empty() {
                div { class: "bg-white rounded-lg shadow p-12 text-center",
                    span { class: "text-6xl mb-6 block", "📅" }
                    p { class: "text-gray-500", "No scheduled flows" }
                }
            } else {
                div { class: "bg-white rounded-lg shadow overflow-hidden",
                    table { class: "min-w-full divide-y divide-gray-200",
                        thead { class: "bg-gray-50",
                            tr {
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Flow ID" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Cron Expression" }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase", "Actions" }
                            }
                        }
                        tbody { class: "bg-white divide-y divide-gray-200",
                            for scheduled in scheduled_flows.read().iter() {
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                        "{scheduled.flow_id}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 font-mono",
                                        "{scheduled.cron_expression}"
                                    }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                                        button {
                                            class: "text-red-600 hover:text-red-900",
                                            onclick: {
                                                let flow_id = scheduled.flow_id.clone();
                                                let scheduled_flows_clone = scheduled_flows.clone();
                                                move |_| {
                                                    let flow_id = flow_id.clone();
                                                    let mut scheduled_flows = scheduled_flows_clone.clone();
                                                    spawn(async move {
                                                        if ApiClient::scheduler_unschedule_flow(&flow_id).await.is_ok() {
                                                            scheduled_flows.write().retain(|s| s.flow_id != flow_id);
                                                        }
                                                    });
                                                }
                                            },
                                            "Unschedule"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if *modal_open.read() {
                div {
                    class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    onclick: move |_| modal_open.set(false),
                    div {
                        class: "bg-white rounded-lg p-6 max-w-md w-full",
                        onclick: |e| e.stop_propagation(),
                        h3 { class: "text-lg font-semibold text-gray-900 mb-4", "Schedule Flow" }
                        if let Some(err) = error.read().as_ref() {
                            div { class: "mb-4 p-3 bg-red-50 text-red-800 rounded", "{err}" }
                        }
                        div { class: "space-y-4",
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2", "Flow" }
                                select {
                                    class: "w-full px-3 py-2 border border-gray-300 rounded",
                                    onchange: move |e| selected_flow_id.set(e.value()),
                                    option { value: "", "Select a flow..." }
                                    for flow in flows.read().iter() {
                                        option { value: "{flow.id}", "{flow.name}" }
                                    }
                                }
                            }
                            div {
                                label { class: "block text-sm font-medium text-gray-700 mb-2", "Cron Expression" }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 rounded font-mono",
                                    r#type: "text",
                                    placeholder: "0 * * * *",
                                    value: "{cron_expression.read()}",
                                    oninput: move |e| cron_expression.set(e.value()),
                                }
                            }
                        }
                        div { class: "mt-6 flex justify-end gap-3",
                            button {
                                class: "px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200",
                                onclick: move |_| modal_open.set(false),
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700",
                                onclick: handle_schedule,
                                disabled: *saving.read(),
                                if *saving.read() { "Saving..." } else { "Schedule" }
                            }
                        }
                    }
                }
            }
        }
    }
}

