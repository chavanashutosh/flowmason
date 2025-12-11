use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DataTableProps {
    pub headers: Vec<String>,
    pub children: Element,
}

#[component]
pub fn DataTable(props: DataTableProps) -> Element {
    rsx! {
        div { class: "overflow-x-auto",
            table { class: "min-w-full divide-y divide-gray-200",
                thead { class: "bg-gray-50",
                    tr {
                        for header in props.headers.iter() {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "{header}"
                            }
                        }
                    }
                }
                tbody { class: "bg-white divide-y divide-gray-200",
                    {props.children}
                }
            }
        }
    }
}

