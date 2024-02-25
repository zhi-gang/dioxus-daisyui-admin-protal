#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
pub(crate) fn Login(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    render!{
        button{class:"btn btn_primary",
            onclick: move |_|{
                nav.push("/home");
            },
            "login"
        }
    }

   
}
