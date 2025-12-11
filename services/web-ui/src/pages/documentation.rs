use dioxus::prelude::*;

#[component]
pub fn Documentation() -> Element {
    rsx! {
        div { class: "space-y-6",
            div {
                h1 { class: "text-3xl font-bold text-gray-900 flex items-center gap-3",
                    span { class: "text-3xl", "📚" }
                    "Documentation"
                }
                p { class: "text-gray-600 mt-2", "Learn how to use FlowMason to build automation workflows" }
            }

            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                DocumentationSection {
                    id: "getting-started",
                    title: "Getting Started",
                    icon: "▶️",
                    content: vec![
                        ("What is FlowMason?", "FlowMason is a visual automation platform that allows you to build powerful workflows by connecting different services and APIs together."),
                        ("Your First Flow", "1. Navigate to Flows and click \"Create Flow\"\n2. Give your flow a name and description\n3. Add bricks to define your workflow steps\n4. Configure each brick with the required parameters\n5. Save and test your flow"),
                    ]
                }
                DocumentationSection {
                    id: "flows",
                    title: "Creating Flows",
                    icon: "🔀",
                    content: vec![
                        ("Flow Builder", "The visual flow builder lets you drag and connect nodes to create your automation workflow."),
                        ("Adding Bricks", "Click \"Add Brick\" to see available integrations. Select a brick type and configure it."),
                        ("Testing Flows", "Use the \"Run Flow\" button to test your flow immediately."),
                    ]
                }
                DocumentationSection {
                    id: "templates",
                    title: "Using Templates",
                    icon: "✨",
                    content: vec![
                        ("Template Library", "Browse pre-built templates to quickly get started with common automation patterns."),
                        ("Using a Template", "1. Go to Templates page\n2. Browse available templates\n3. Click \"Use Template\"\n4. Customize the flow"),
                    ]
                }
                DocumentationSection {
                    id: "executions",
                    title: "Monitoring Executions",
                    icon: "⏱️",
                    content: vec![
                        ("Execution History", "View all flow executions in the Executions page."),
                        ("Execution Status", "Executions can have the following statuses:\n• Pending\n• Running\n• Completed\n• Failed"),
                    ]
                }
                DocumentationSection {
                    id: "scheduler",
                    title: "Scheduling Flows",
                    icon: "📅",
                    content: vec![
                        ("Cron Expressions", "Schedule flows to run automatically using cron expressions."),
                        ("Creating a Schedule", "1. Go to Scheduler page\n2. Click \"Schedule Flow\"\n3. Select a flow\n4. Enter a cron expression"),
                    ]
                }
                DocumentationSection {
                    id: "metering",
                    title: "Usage & Metering",
                    icon: "📈",
                    content: vec![
                        ("Usage Tracking", "Monitor usage and quotas for each brick type."),
                        ("Quotas", "Each brick type has daily and monthly usage limits."),
                    ]
                }
            }

            div { class: "bg-primary-50 border border-primary-200 rounded-lg p-6",
                h2 { class: "text-xl font-bold text-gray-900 mb-4 flex items-center gap-2",
                    span { "⚙️" }
                    "Quick Tips"
                }
                ul { class: "space-y-2 text-gray-700",
                    li { class: "flex items-start gap-2",
                        span { class: "text-primary-600 font-bold", "•" }
                        span { "Start with templates to learn common patterns" }
                    }
                    li { class: "flex items-start gap-2",
                        span { class: "text-primary-600 font-bold", "•" }
                        span { "Test flows before scheduling them" }
                    }
                    li { class: "flex items-start gap-2",
                        span { class: "text-primary-600 font-bold", "•" }
                        span { "Monitor execution history to debug issues" }
                    }
                    li { class: "flex items-start gap-2",
                        span { class: "text-primary-600 font-bold", "•" }
                        span { "Check usage limits in Metering before production" }
                    }
                }
            }
        }
    }
}

#[component]
fn DocumentationSection(id: &'static str, title: &'static str, icon: &'static str, content: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow p-6 hover:shadow-lg transition-shadow",
            div { class: "flex items-center gap-3 mb-4",
                div { class: "p-2 bg-primary-50 rounded-lg",
                    span { class: "text-2xl", "{icon}" }
                }
                h2 { class: "text-xl font-bold text-gray-900", "{title}" }
            }
            div { class: "space-y-4",
                for (heading, text) in content.iter() {
                    div {
                        h3 { class: "font-semibold text-gray-900 mb-2", "{heading}" }
                        p { class: "text-gray-600 text-sm whitespace-pre-line", "{text}" }
                    }
                }
            }
        }
    }
}

