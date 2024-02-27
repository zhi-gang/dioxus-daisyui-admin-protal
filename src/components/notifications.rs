#![allow(non_snake_case)]
use dioxus::prelude::*;
use log::info;

use crate::{components::customized_svg::CustomizedSvg5, services::notification::NotificationList, views::home::ShowNotications};

#[component]
pub fn Notifications(cx: Scope) -> Element {
    let show_notifications= use_shared_state::<ShowNotications>(cx).unwrap();
    let notification_list = use_shared_state::<NotificationList>(cx).unwrap();
    let notifs = notification_list.read();

    let nodes = notifs.iter().map(|n|{
        let n1 = n.clone();
        rsx!{
            div{class:"grid mt-3 card bg-base-200 rounded-box p-3 ",
                onclick:move |_|{
                    info!("c {n1:?}")
                },
                "{n.level:?}: {n1.msg}",
            }
        }
    });

    render!{
        div{class:" fixed overflow-hidden z-20 bg-gray-900 bg-opacity-25 inset-0 transform ease-in-out  transition-opacity opacity-100 duration-500 translate-x-0  ",
            section{class:"w-80 md:w-96  right-0 absolute bg-base-100 h-full shadow-xl delay-400 duration-500 ease-in-out transition-all transform   translate-x-0 ",
                div{class:"relative  pb-5 flex flex-col  h-full",
                    div{class:"navbar   flex pl-4 pr-4   shadow-md ",
                        button{
                            onclick: move |_|show_notifications.with_mut(|m|*m=ShowNotications::Hide),
                            CustomizedSvg5{d:"M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"}
                        }
                        span{class:"ml-2 font-bold text-xl", "Notifications"}
                    }
                    div{class:"overflow-y-auto pl-4 pr-4",
                        div{class:"flex flex-col w-full",
                            div{class:"grid mt-3 card bg-base-200 rounded-box p-3 bg-blue-100","Your sales has increased by 30% yesterday"}
                            div{class:"grid mt-3 card bg-base-200 rounded-box p-3 ","Your sales has increased by 30% yesterday"}
                            
                            nodes    
                                
                        }
                        
                    }
                    
                }
            }
        } 
    } 
}