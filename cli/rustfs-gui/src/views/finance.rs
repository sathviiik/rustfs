use crate::components::Finance;
use dioxus::prelude::*;

#[component]
pub fn FinanceViews() -> Element {
    rsx! {
        Finance {}
    }
}