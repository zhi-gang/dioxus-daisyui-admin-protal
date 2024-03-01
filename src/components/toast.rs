#![allow(non_snake_case, unused)]

use crate::services::notification::{NotificationLevels, NotificationList, NotificationMsg};
use chrono::{Duration, Utc};
use dioxus::prelude::*;

pub(crate) fn Toast(cx: Scope) -> Element {
    let notifications = use_shared_state::<NotificationList>(cx).unwrap();
    let noti = use_state(cx, ||Vec::<NotificationMsg>::new());
   
    let noti_dom = noti.get().iter().map(|n|{
        let c = match n.level {
            NotificationLevels::Info => "alert-info",
            NotificationLevels::Warn => "alert-warning",
            NotificationLevels::Error => "alert-error",
        };
        rsx!{
            div{class:"alert {c} mb-1 ",
                span{"{n.msg}"}
            }
        }
    });

    #[cfg(feature = "desktop")]
    let _sync_task: &Coroutine<()> = use_coroutine(cx, |_: UnboundedReceiver<_>| {
        let notifications = notifications.to_owned();
        let noti = noti.to_owned();
        
        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let now = Utc::now();
                noti.with_mut(|l|{
                        *l = notifications.with(|list| 
                        list.clone().into_iter()
                                .filter(|msg|(now - msg.create_at) < Duration::seconds(5))
                                .collect::<NotificationList>());
                    }); 
            }
        }
    });
    
    render!(
        div{class:"fixed top-auto bottom-0 right-0 flex flex-col p-1  w-80  max-h-80  overflow-y-auto",
            noti_dom
        }
    )
}
