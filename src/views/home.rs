#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{components::Outlet, hooks::use_navigator};

use crate::route::Route;

pub(crate) fn Home(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    render!("home"

    div{
        "router div",
        button{class:"btn btn_primary",
            onclick: move |_|{
                nav.push("/home/c2");
            },
            "c2"
        }
        Outlet::<Route> {}
    })
}