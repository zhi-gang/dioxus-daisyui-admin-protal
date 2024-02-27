#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::components::Link;
use crate::{components::customized_svg::CustomizedSvg6, views::home::MenuItemType};

#[component]
pub fn DropdownMenuItem<'a>(cx: Scope) -> Element {
    let menu_selection= use_shared_state::<MenuItemType>(cx).unwrap();
    render!{
        li{
            details{class:"flex flex-col",
                summary{class:"w-full",
                    CustomizedSvg6{d:"M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75"}
                    "Pages",
                }
           
                ul{class:"w-full menu menu-compact",
                    li{
                        Link{to:"/login",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"h-6 w-6 inline stroke-2",
                                path{style:"stroke-linecap:round;stroke-linejoin:round",d:"M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9"}
                            },
                            "Logout"
                        }
                    }
                    li{
                        Link{to:"/home/leads",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"h-6 w-6 inline stroke-2",
                                path{style:"stroke-linecap:round;stroke-linejoin:round",d:"M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"}
                            }
                            onclick: move |_|menu_selection.with_mut(|m|*m = MenuItemType::Leads),
                            "leads"
                        }
                    }
                }
            }
        }
    }
}