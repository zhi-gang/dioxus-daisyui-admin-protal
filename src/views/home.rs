#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{components::Outlet, hooks::use_navigator};

use crate::route::Route;

pub(crate) fn Home(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    render!("home"

    div{
        "router div",
        img{class:"w-12 inline-block mr-2 mask mask-circle", alt:"dashwind-logo",src:"../logo192.png"}
        button{class:"btn btn_primary",
            onclick: move |_|{
                nav.push("/home/c2");
            },
            "c2"
        }
        Outlet::<Route> {}
    })
}

