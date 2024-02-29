#![allow(non_snake_case)]

use crate::services::notification::{NotificationLevels, NotificationList, NotificationMsg};
use chrono::{Duration, Utc};
use dioxus::prelude::*;
use log::info;

pub(crate) fn Toast(cx: Scope) -> Element {
    // let notificatios = use_shared_state::<NotificationList>(cx).unwrap();
    // let noti = use_state(cx, ||Vec::<NotificationMsg>::new());
    // let now = Utc::now();
    // noti.with_mut(|l|{
    //     *l = notificatios.with(|list| 
    //         list.clone().into_iter()
    //             .filter(|msg|(now - msg.create_at) < Duration::seconds(5))
    //             .collect::<NotificationList>());
    //     }); 
    // let sync_task: &Coroutine<()> = use_coroutine(cx, |_:UnboundedReceiver<_>| {
    //     let noti = noti.to_owned();
    //     async move {
    //         // loop {
    //             std::thread::sleep(std::time::Duration::from_secs(2));
    //             let now = Utc::now();
    //             // let data = notificatios.with(|list| 
    //             //             list.clone().into_iter()
    //             //                 .filter(|msg|(now - msg.create_at) < Duration::seconds(5))
    //             //                 .collect::<NotificationList>());


    //             // noti.with_mut(|l|{
    //             //     *l = notificatios.with(|list| 
    //             //         list.clone().into_iter()
    //             //             .filter(|msg|(now - msg.create_at) < Duration::seconds(5))
    //             //             .collect::<NotificationList>());
    //             //     }); 
    //         // }
            
    //     }
    // });
   
    // let noti_dom = noti.get().iter().map(|n|{
    //     let c = match n.level {
    //         NotificationLevels::Info => "alert-info",
    //         NotificationLevels::Warn => "alert-warning",
    //         NotificationLevels::Error => "alert-error",
            
    //     };
    //     rsx!{
    //         div{class:"alert {c} mb-1 ",
    //             span{"{n.msg}"}
    //         }
    //     }
    // });

    let sync_status = use_state(cx, || 0);
    let sync_task: &Coroutine<()> = use_coroutine(cx, |rx: UnboundedReceiver<_>| {
        let sync_status = sync_status.to_owned();
        async move {
            let mut i=0;
             loop {
                info!("11 {}",i);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                sync_status.set(i);
                i +=1;
            }
        }
    });
    
    render!(
        div{class:"fixed top-auto bottom-0 right-0 flex flex-col p-1  w-80  max-h-80  overflow-y-auto",
            // noti_dom
            "{sync_status.get()}"
        }
    )
}
