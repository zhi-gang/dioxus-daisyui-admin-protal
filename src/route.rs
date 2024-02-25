#![allow(non_snake_case)]

use crate::views::{home::Home, login::Login, page_not_found::PageNotFound};
use crate::components::{c1::C1, c2::C2};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
        #[redirect("/login",||Route::Login {})]
    #[route("/login")]
    Login {},

    #[nest("/home")]
    #[layout(Home)]
        #[route("/")]
            C1 {},
        #[route("/c2")]
            C2 {},
    #[end_layout]
    #[end_nest]
    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
