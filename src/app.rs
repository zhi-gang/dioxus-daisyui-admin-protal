#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::components::Router;
use crate::route::Route;

pub fn App(cx: Scope) -> Element {
    let create_eval = use_eval(cx);
    use_shared_state_provider(cx, ||"light".to_string());

    let mode = use_shared_state::<String>(cx).unwrap();
    let _eval = create_eval(&*format!("document.documentElement.setAttribute(\"data-theme\", '{}');",*mode.read())).unwrap();
    // eval.send("light".into()).unwrap();
    render! {
        style { include_str!("../public/tailwind.css") }
        head{
            meta { charset: "UTF-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        }
        div{
            Router::<Route> {}
        }
    }
}
