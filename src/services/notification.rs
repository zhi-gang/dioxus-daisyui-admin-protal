#[derive(Debug,PartialEq,Clone)]
pub enum NotificationLevels {
    Info,
    Warn,
    Error
}
#[derive(Debug,PartialEq,Clone)]
pub struct NotificationMsg {
    pub level: NotificationLevels,
    pub msg: String
}

pub type NotificationList = Vec<NotificationMsg>;