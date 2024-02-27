#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{components::customized_svg::CustomizedSvg5, views::home::ShowNotications};

#[component]
pub fn Bell(cx: Scope) -> Element {
    let show_notifications= use_shared_state::<ShowNotications>(cx).unwrap();
    render!{
        button{class:"btn btn-ghost ml-4  btn-circle",
            div{class:"indicator",
                onclick: move |_|show_notifications.with_mut(|m|*m=ShowNotications::Show),
                CustomizedSvg5 {d:"M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0"}
                span{class:"indicator-item badge badge-secondary badge-sm", "15"}
            }
        }
    } 
}