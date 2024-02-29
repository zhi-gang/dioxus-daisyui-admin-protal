#![allow(non_snake_case)]

use chrono::Utc;
use dioxus::prelude::*;

use crate::{components::customized_svg::CustomizedSvg, services::notification::{NotificationLevels, NotificationList, NotificationMsg}};

#[component]
pub fn NotificationMaker(cx: Scope) -> Element {
    let notification_list = use_shared_state::<NotificationList>(cx).unwrap();
    render!{
        li{
            details{class:"flex flex-col",
                summary{class:"w-full",
                    CustomizedSvg{s:6, d:"M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"}
                    "Actions",
                }
           
                ul{class:"w-full menu menu-compact",
                    li{class:"",
                        a{
                            onclick: move |_|{
                                notification_list.with_mut(|m|(*m).push(
                                    NotificationMsg {
                                        level: NotificationLevels::Info,
                                        msg: "a new info notification that has long long long long long long long long long long long long long long long long long long text".to_string(),
                                        create_at:Utc::now()
                                    }
                                ))
                            },
                            CustomizedSvg{s:6, d:"M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"},
                            "Send Notification(info)",
                        },
                        
                    }
                    li{class:"",
                        a{
                            onclick: move |_|{
                                notification_list.with_mut(|m|(*m).push(
                                    NotificationMsg {
                                        level: NotificationLevels::Warn,
                                        msg: "a new warn notification".to_string(),
                                        create_at:Utc::now()
                                    }
                                ))
                            },
                            CustomizedSvg{s:6, d:"M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"},
                            "Send Notification(warn)"
                        },
                        
                    }
                    li{class:"",
                        a{
                            onclick: move |_|{
                                notification_list.with_mut(|m|(*m).push(
                                    NotificationMsg {
                                        level: NotificationLevels::Error,
                                        msg: "a new error notification".to_string(),
                                        create_at:Utc::now()
                                    }
                                ))
                            },
                            CustomizedSvg{s:6, d:"M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z"},
                            "Send Notification(error)"
                        },
                        
                    }
                }
            }
        }
    }
}