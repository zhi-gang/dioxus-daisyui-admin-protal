#![allow(non_snake_case)]

use crate::components::{
    dashboard::Dashboard, forget_form::ForgetForm, info::Info, login_form::LoginForm,
    register_form::RegisterForm,
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
            Info {},
        #[route("/dashboard")]
            Dashboard {},
    #[end_layout]
    #[end_nest]
    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
