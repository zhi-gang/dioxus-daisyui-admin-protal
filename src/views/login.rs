#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Outlet;

use crate::{components::login_decoration::LoginDecoration, route::Route};

pub(crate) fn Login(cx: Scope) -> Element {
    render! {
        div{class:"min-h-screen bg-base-200 flex items-center",
            div{class:"card mx-auto w-full max-w-5xl  shadow-xl",
                div {class:"grid  md:grid-cols-2 grid-cols-1  bg-base-100 rounded-xl",
                    LoginDecoration {}
                    Outlet::<Route> {}
                }
            }
        }
    }
}
