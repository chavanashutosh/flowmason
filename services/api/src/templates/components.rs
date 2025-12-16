// HTML component generation functions

pub fn status_badge(status: &str) -> String {
    let (color_class, label) = match status {
        "completed" | "active" => ("bg-green-100 text-green-800", "Completed"),
        "failed" => ("bg-red-100 text-red-800", "Failed"),
        "running" => ("bg-blue-100 text-blue-800", "Running"),
        "pending" => ("bg-yellow-100 text-yellow-800", "Pending"),
        "inactive" => ("bg-gray-100 text-gray-800", "Inactive"),
        _ => ("bg-gray-100 text-gray-800", status),
    };
    
    format!(
        r#"<span class="px-2 py-1 text-xs font-semibold rounded {}">{}</span>"#,
        color_class, label
    )
}

pub fn stats_card(title: &str, value: &str, icon: Option<&str>) -> String {
    let icon_html = if let Some(icon_svg) = icon {
        format!(
            r#"<div class="p-3 rounded-lg bg-primary-50 text-primary-600 ml-4 flex-shrink-0">{}</div>"#,
            icon_svg
        )
    } else {
        String::new()
    };
    
    format!(
        r#"<div class="h-full bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow p-6">
            <div class="flex items-start justify-between">
                <div class="flex-1">
                    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">{}</p>
                    <p class="text-5xl font-bold text-gray-900">{}</p>
                </div>
                {}
            </div>
        </div>"#,
        title, value, icon_html
    )
}

pub fn empty_state(title: &str, description: &str, action_label: Option<&str>, action_url: Option<&str>) -> String {
    let action_html = if let Some(label) = action_label {
        if let Some(url) = action_url {
            format!(
                r#"<a href="{}" class="px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors inline-block">{}</a>"#,
                url, label
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    format!(
        r#"<div class="flex flex-col items-center justify-center py-12 px-4 text-center">
            <div class="mb-4 text-gray-400">
                <svg class="w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">{}</h3>
            <p class="text-sm text-gray-500 mb-6 max-w-sm">{}</p>
            {}
        </div>"#,
        title, description, action_html
    )
}

pub fn data_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let headers_html: String = headers
        .iter()
        .map(|h| format!(r#"<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{}</th>"#, h))
        .collect::<Vec<_>>()
        .join("\n");
    
    let rows_html: String = rows
        .iter()
        .map(|row| {
            let cells: String = row
                .iter()
                .map(|cell| format!(r#"<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{}</td>"#, cell))
                .collect::<Vec<_>>()
                .join("\n");
            format!(r#"<tr class="hover:bg-gray-50">{}</tr>"#, cells)
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(
        r#"<div class="overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                    <tr>
                        {}
                    </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                    {}
                </tbody>
            </table>
        </div>"#,
        headers_html, rows_html
    )
}

#[allow(dead_code)]
pub fn confirm_modal(id: &str, title: &str, message: &str, confirm_text: &str, cancel_text: &str, action_url: &str) -> String {
    format!(
        r#"<div id="{}" class="hidden fixed inset-0 z-50 overflow-y-auto">
            <div class="flex items-center justify-center min-h-screen px-4">
                <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" onclick="document.getElementById('{}').classList.add('hidden')"></div>
                <div class="relative bg-white rounded-lg shadow-xl max-w-md w-full p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">{}</h3>
                    <p class="text-sm text-gray-600 mb-6">{}</p>
                    <div class="flex justify-end space-x-3">
                        <button onclick="document.getElementById('{}').classList.add('hidden')" class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors">
                            {}
                        </button>
                        <a href="{}" class="px-4 py-2 text-sm font-medium text-white bg-red-600 hover:bg-red-700 rounded-lg transition-colors">
                            {}
                        </a>
                    </div>
                </div>
            </div>
        </div>"#,
        id, id, title, message, id, cancel_text, action_url, confirm_text
    )
}

#[allow(dead_code)]
pub fn flow_card(title: &str, content: &str, actions: Option<&str>) -> String {
    let actions_html = if let Some(actions) = actions {
        format!(r#"<div class="flex items-center space-x-2">{}</div>"#, actions)
    } else {
        String::new()
    };
    
    format!(
        r#"<div class="bg-white rounded-lg shadow p-6">
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-semibold text-gray-900">{}</h3>
                {}
            </div>
            {}
        </div>"#,
        title, actions_html, content
    )
}
