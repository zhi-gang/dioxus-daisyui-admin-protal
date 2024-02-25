#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render!("404")
}