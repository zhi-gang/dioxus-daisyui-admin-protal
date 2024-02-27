#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::{Link, Outlet};
use log::info;

use crate::{components::{dropdown_menu_item::DropdownMenuItem, menu_item::MenuItem, theme::Theme}, route::Route};

#[derive(Debug,PartialEq, Clone)]
pub enum MenuItemType{
    Home,
    Dashboard,
    Leads,
    Transactions,
}

#[derive(Debug,PartialEq, Clone)]
pub enum ShowNotications{
    Show,
    Hide
}

impl MenuItemType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Home=> "Home".to_string(),
            Self::Dashboard=> "Dashboard".to_string(),
            Self::Leads=> "Leads".to_string(),
            Self::Transactions=> "Transactions".to_string(),
        }
    }
}

pub fn Home(cx: Scope) ->Element {
    use_shared_state_provider::<MenuItemType>(cx, ||MenuItemType::Home);
    let menu_selection= use_shared_state::<MenuItemType>(cx).unwrap();
    use_shared_state_provider::<ShowNotications>(cx, ||ShowNotications::Hide);
    let show_notifications= use_shared_state::<ShowNotications>(cx).unwrap();
    render!{
        div{class:"drawer lg:drawer-open",
            input{id:"my-drawer-2", r#type:"checkbox", class:"drawer-toggle"}
            div{class:"drawer-content flex flex-col",
                div{class:"navbar sticky top-0 bg-base-100  z-10 shadow-md ",
                    div{class:"flex-1",
                        label{r#for:"my-drawer-2", class:"btn btn-primary drawer-button lg:hidden",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-5 inline-block w-5 stroke-2",
                                path{d:"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"}
                            }
                        }
                        h1{class:"text-2xl font-semibold ml-2","{(*menu_selection.read()).to_string()}"}
                    }
                    Theme {}
                   
                    button{class:"btn btn-ghost ml-4  btn-circle",
                        div{class:"indicator",
                            onclick: move |_|show_notifications.with_mut(|m|*m=ShowNotications::Show),
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-5 inline-block w-5 stroke-2",
                                path{d:"M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0"}
                            }
                            span{class:"indicator-item badge badge-secondary badge-sm", "15"}
                        }
                    }
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
                    MenuItem {item_type: MenuItemType::Dashboard, url: "/home/dashboard", d:"M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z"}
                    MenuItem {item_type: MenuItemType::Leads, url: "/home/leads", d:"M9 3.75H6.912a2.25 2.25 0 00-2.15 1.588L2.35 13.177a2.25 2.25 0 00-.1.661V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 00-2.15-1.588H15M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859M12 3v8.25m0 0l-3-3m3 3l3-3"}
                    MenuItem {item_type: MenuItemType::Transactions, url: "/home/transactions", d:"M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z"}
                    DropdownMenuItem {}
                }
            }
               
        }
       
        if *show_notifications.read() == ShowNotications::Show{
            render!{
                div{class:" fixed overflow-hidden z-20 bg-gray-900 bg-opacity-25 inset-0 transform ease-in-out  transition-opacity opacity-100 duration-500 translate-x-0  ",
                // class:"fixed overflow-hidden z-20 bg-gray-900 bg-opacity-25 inset-0 transform ease-in-out  transition-all delay-500 opacity-0 translate-x-full  ",
                    section{class:"w-80 md:w-96  right-0 absolute bg-base-100 h-full shadow-xl delay-400 duration-500 ease-in-out transition-all transform   translate-x-0 ",
                        div{class:"relative  pb-5 flex flex-col  h-full",
                            div{class:"navbar   flex pl-4 pr-4   shadow-md ",
                                button{
                                    onclick: move |_|show_notifications.with_mut(|m|*m=ShowNotications::Hide),
                                    svg{xmlns:"http://www.w3.org/2000/svg",  stroke:"currentColor" ,class:"h-5 inline-block w-5 stroke-1 ",
                                        path{d:"M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"}
                                    }
                                }
                                span{class:"ml-2 font-bold text-xl", "Notifications"}
                            }
                            div{class:"overflow-y-scroll pl-4 pr-4",
                                div{class:"flex flex-col w-full",
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                                    div{class:"grid mt-3 card bg-base-200 rounded-box p-3 ","Your sales has increased by 30% yesterday"}

                                }
                            }
                        }
                    }
                } 
            }
        }
            
    }
}