use hc_zome_traits::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub large_body: Option<String>,
    pub summary: Option<String>,
    pub group: Option<String>,
    pub group_summary: bool,
    pub icon: Option<String>,
    pub large_icon: Option<String>,
    pub icon_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotificationInput {
    pub notification_id: String,
    pub locale: String,
}

#[zome_trait]
pub trait NotificationsZomeTrait {
    fn get_notification(input: GetNotificationInput) -> ExternResult<Option<Notification>>;
}
