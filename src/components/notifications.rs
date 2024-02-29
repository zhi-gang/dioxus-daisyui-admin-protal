#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::customized_svg::CustomizedSvg,
    services::notification::{NotificationLevels, NotificationList},
    views::home::ShowNotications,
};

#[component]
pub fn Notifications(cx: Scope) -> Element {
    let show_notifications = use_shared_state::<ShowNotications>(cx).unwrap();
    let notification_list = use_shared_state::<NotificationList>(cx).unwrap();
    let notifs = notification_list.read();

    let nodes = notifs.iter().map(|n|{
        let n2 = n.to_owned();  //clone() as well
        let bg= match n.level {
            NotificationLevels::Info=> "bg-accent",
            NotificationLevels::Warn=> "bg-primary",
            NotificationLevels::Error=> "bg-secondary",
        };
        rsx!{
            div{class:"grid mt-3 card bg-base-200 rounded-box p-3 ",
                div{class:"flex items-stretch",
                    div{class:"flex-1",
                        span{class:"absolute inset-y-0 left-0 w-1 rounded-tr-lg rounded-br-lg {bg}"}
                        label{"{n.level:?}: {n.msg}"},
                    }
                    div{class:"flex-none justify-items-end",
                        onclick:move |_|{
                            notification_list.with_mut(|list:&mut NotificationList|{
                                if let Some(index) = list.iter().position(|value| *value == n2) {
                                    list.remove(index);
                                }
                            })
                        },
                        CustomizedSvg{s:6, d:"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"}
                    }
                }
            }
        }
    });

    render! {
        div{class:" fixed overflow-hidden z-20 bg-gray-900 bg-opacity-25 inset-0 transform ease-in-out  transition-opacity opacity-100 duration-500 translate-x-0  ",
            section{class:"w-80 md:w-96  right-0 absolute bg-base-100 h-full shadow-xl delay-400 duration-500 ease-in-out transition-all transform   translate-x-0 ",
                div{class:"relative  pb-5 flex flex-col  h-full",
                    div{class:"navbar   flex pl-4 pr-4   shadow-md ",
                        button{
                            onclick: move |_|show_notifications.with_mut(|m|*m=ShowNotications::Hide),
                            CustomizedSvg{s:5, d:"M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"}
                        }
                        span{class:"ml-2 font-bold text-xl", "Notifications"}
                    }
                    div{class:"overflow-y-auto pl-4 pr-4",
                        div{class:"flex flex-col w-full",
                            nodes
                        }
                    }
                }
            }
        }
    }
}
