use hc_zome_traits::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon_src: String,
    pub group: Option<String>,
    pub url_path_to_navigate_to_on_click: Option<String>,
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
