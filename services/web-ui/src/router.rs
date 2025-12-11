use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{Dashboard, Flows, FlowDetail, NewFlow, EditFlow, Templates, Executions, Scheduler, Metering, Mapping, Documentation, Settings};

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/flows")]
    Flows {},
    #[route("/flows/:id")]
    FlowDetail { id: String },
    #[route("/flows/new")]
    NewFlow {},
    #[route("/flows/:id/edit")]
    EditFlow { id: String },
    #[route("/templates")]
    Templates {},
    #[route("/executions")]
    Executions {},
    #[route("/scheduler")]
    Scheduler {},
    #[route("/metering")]
    Metering {},
    #[route("/mapping")]
    Mapping {},
    #[route("/documentation")]
    Documentation {},
    #[route("/settings")]
    Settings {},
}

