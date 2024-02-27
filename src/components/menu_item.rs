#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::components::Link;
use crate::views::home::MenuItemType;

#[component]
pub fn MenuItem<'a>(cx: Scope, item_type: MenuItemType ,url: &'a str, d:&'a str) -> Element {
    let menu_selection= use_shared_state::<MenuItemType>(cx).unwrap();
    render!{
        li{class:"",
            Link{to:*url,
                svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"h-6 w-6 stroke-2",
                    path{d:"{d}"}
                }
                onclick: move |_|menu_selection.with_mut(|m|*m = item_type.clone()),
                if *menu_selection.read() == *item_type{
                    // bolded.set("font-bold");
                    render!{
                        span{class:"absolute inset-y-0 left-0 w-1 rounded-tr-md rounded-br-md bg-primary"}
                        label{class:"font-bold","{item_type.to_string()}"}
                    }
                }else{
                    render!{label{class:"font-normal","{item_type.to_string()}"}}
                }
                
            }
        }
    }
}