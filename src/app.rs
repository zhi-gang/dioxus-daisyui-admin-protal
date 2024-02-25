#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::components::Router;
use crate::route::Route;

pub fn App(cx: Scope) -> Element {
    render! {
        style { include_str!("../public/tailwind.css") }

        head{
            meta { charset: "UTF-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        }
        div{
            "xxxxxxxx"
        }
        div{
            Router::<Route> {}
        }
    }
}
