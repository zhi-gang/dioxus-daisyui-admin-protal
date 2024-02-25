#![allow(non_snake_case)]

use crate::components::{
    c1::C1, c2::C2, forget_form::ForgetForm, login_form::LoginForm, register_form::RegisterForm,
};
use crate::views::{home::Home, login::Login, page_not_found::PageNotFound, root::Root};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Root {},

    #[nest("/login")]
    #[layout(Login)]
        #[route("/")]
            LoginForm {},
        #[route("/forget")]
            ForgetForm {},
        #[route("/register")]
            RegisterForm {},
        
    #[end_layout]
    #[end_nest]

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
