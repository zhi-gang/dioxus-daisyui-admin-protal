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

