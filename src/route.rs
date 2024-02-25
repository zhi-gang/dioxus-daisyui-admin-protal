#![allow(non_snake_case)]
use crate::views::{home::Home, login::Login};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
        #[redirect("/login",||Route::Login {})]
    #[route("/login")]
    Login {},
    #[route("/home")]
    Home {},
}
