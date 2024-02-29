#![allow(non_snake_case)]
use dioxus::prelude::*;


#[component]
pub fn CustomizedSvg<'a>(cx: Scope, s: usize, d: &'a str) -> Element {
    let c = format!("fill-none stroke-2 stroke-current inline-block h-{s}  w-{s}");
    render!(
        svg{xmlns:"http://www.w3.org/2000/svg", class:"{c}",
            path{d: *d}
        }
    )
}
