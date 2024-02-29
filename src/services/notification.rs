use std::sync::{Arc, Mutex};
// use lazy_static::lazy_static;
use chrono::{DateTime, Utc};
// use log::info;
// use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq, Clone)]
pub enum NotificationLevels {
    Info,
    Warn,
    Error,
}
#[derive(Debug, PartialEq, Clone)]
pub struct NotificationMsg {
    pub level: NotificationLevels,
    pub msg: String,
    pub create_at: DateTime<Utc>
}

pub type NotificationList = Vec<NotificationMsg>;

// lazy_static!{
//     static ref QUEUE: Arc<Mutex<NotificationList>> = Arc::new(Mutex::new(Vec::new()));
// }

// pub fn push(msg: NotificationMsg) -> anyhow::Result<()> {
//     QUEUE.lock().unwrap().push(msg);
//     Ok(())
// }

// pub async fn auto_pop()-> anyhow::Result<()> {
//     let h = tokio::spawn(async move {
//         loop{
//             sleep(Duration::from_secs(5)).await;
//             let q = QUEUE.lock().unwrap();
//             // q.filter(|msg|{
//                 info!("111");

//             // })
//         }
//     });
//     Ok(())
// }
