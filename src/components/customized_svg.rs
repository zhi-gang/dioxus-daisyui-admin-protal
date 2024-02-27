#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn CustomizedSvg4<'a>(cx: Scope, d: &'a str) -> Element {
    render!(
        svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-4 inline-block w-4 stroke-1",
            path{d: *d}
        }
    )
}

#[component]
pub fn CustomizedSvg5<'a>(cx: Scope, d: &'a str) -> Element {
    render!(
        svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-5 inline-block w-5 stroke-2",
            path{d: *d}
        }
    )
}

#[component]
pub fn CustomizedSvg6<'a>(cx: Scope, d: &'a str) -> Element {
    render!(
        svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-6 inline-block w-6 stroke-2",
            path{d: *d}
        }
    )
}