#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

pub(crate) fn Root(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    nav.push("/login");
    render!{div{class:"h-5 w-5",  //here is to fore import class h-5 w-5
        // div {class:"h-4 w-4"}
    }}
}

