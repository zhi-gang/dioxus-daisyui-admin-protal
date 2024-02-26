#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::warn;
#[component]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    warn!("visit unsupported address: {route:?}");
    render!("Page Not Found")
}