#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::{Link, Outlet};
use log::info;

use crate::{
    components::{
        bell::Bell, customized_svg::CustomizedSvg6, dropdown_menu_item::DropdownMenuItem, menu_item::MenuItem, notifications::Notifications, theme::Theme
    },
    route::Route,
    services::notification::{NotificationLevels, NotificationList, NotificationMsg},
};

#[derive(Debug, PartialEq, Clone)]
pub enum MenuItemType {
    Home,
    Dashboard,
    Leads,
    Transactions,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShowNotications {
    Show,
    Hide,
}

impl MenuItemType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Home => "Home".to_string(),
            Self::Dashboard => "Dashboard".to_string(),
            Self::Leads => "Leads".to_string(),
            Self::Transactions => "Transactions".to_string(),
        }
    }
}

pub fn Home(cx: Scope) -> Element {
    use_shared_state_provider::<MenuItemType>(cx, || MenuItemType::Home);
    let menu_selection = use_shared_state::<MenuItemType>(cx).unwrap();
    use_shared_state_provider::<ShowNotications>(cx, || ShowNotications::Hide);
    let show_notifications = use_shared_state::<ShowNotications>(cx).unwrap();

    let notification_list = use_shared_state::<NotificationList>(cx).unwrap();
    render! {
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
                    Bell {}
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
                    li{class:"",
                        a{
                            onclick: move |_|{
                                notification_list.with_mut(|m|(*m).push(
                                    NotificationMsg {
                                        level: NotificationLevels::Warn,
                                        msg: "a new notification".to_string()
                                    }
                                ))
                            },
                            CustomizedSvg6 {d:"M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"},
                            "Send Notification"
                        },
                        
                    }
                }
            }

        }
        if *show_notifications.read() == ShowNotications::Show{
            render!{Notifications {}}
        }
    }
}
