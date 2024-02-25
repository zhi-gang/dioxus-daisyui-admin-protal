#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Router;

use crate::route_home::RouteHome;

pub(crate) fn Home(cx: Scope) -> Element {
    render!("home"

    div{
        Router::<RouteHome> {}
    })
}