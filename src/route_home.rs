#![allow(non_snake_case)]
use crate::components::{c1::C1, c2::C2};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum RouteHome {
    #[route("/")]
    C1 {},
    #[route("/c2")]
    C2 {}

}
