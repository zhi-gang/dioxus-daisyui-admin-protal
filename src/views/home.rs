#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::{Link, Outlet};
use log::info;

use crate::{components::{menu_item::MenuItem, theme::Theme}, route::Route};

#[derive(PartialEq, Clone)]
pub enum MenuItemType{
    Home,
    Dashboard
}

pub fn Home(cx: Scope) ->Element {
    use_shared_state_provider::<MenuItemType>(cx, ||MenuItemType::Home);
    let menu_selection= use_shared_state::<MenuItemType>(cx).unwrap();
    
    render!{
        div{class:"drawer lg:drawer-open",
            input{id:"my-drawer-2", r#type:"checkbox", class:"drawer-toggle"}
            div{class:"drawer-content flex flex-col",
                div{class:"navbar sticky top-0 bg-base-100  z-10 shadow-md ",
                    div{class:"flex-1",
                        label{r#for:"my-drawer-2", class:"btn btn-primary drawer-button lg:hidden",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-5 inline-block w-5 stroke-2",
                                path{d:"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",style:"stroke-linecap:round;stroke-linejoin:round"}
                            }
                        }
                    }
                    Theme {}
                }
                main{class:"flex-1 overflow-y-auto md:pt-4 pt-4 px-6  bg-base-200",
                    Outlet::<Route> {}
                }
            }  
            div{class:"drawer-side z-30",
                label{r#for:"my-drawer-2" , class:"drawer-overlay"}
                ul{class:"menu pt-2  w-80 min-h-full bg-base-100 text-base-content",
                    li{class:"mb-2 font-semibold text-xl",
                        Link{to:"/home",
                            img{class:"mask mask-squircle w-10", src:"../logo192.png",alt:"DashWind Logo"}
                            "DashWind"
                            onclick: move |_|menu_selection.with_mut(|m|*m=MenuItemType::Home),
                        }
                    }
                    MenuItem {item_type: MenuItemType::Dashboard, item_name: "Dashboard" ,url: "/home/dashboard", d:"M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z"}
                }
            }    
        }
    }
}