#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::components::Router;
use fermi::{use_init_atom_root, Atom};
use crate::{route::Route, services::notification::NotificationList};

pub static USERNAME: Atom<String> = Atom(|_| "".to_string());

pub fn App(cx: Scope) -> Element {
    let create_eval = use_eval(cx);
    use_shared_state_provider(cx, ||"light".to_string());
    let mode = use_shared_state::<String>(cx).unwrap();
    let _eval = create_eval(&*format!("document.documentElement.setAttribute(\"data-theme\", '{}');",*mode.read())).unwrap();

    use_shared_state_provider::<NotificationList>(cx, ||Vec::new());
    use_init_atom_root(cx);

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
