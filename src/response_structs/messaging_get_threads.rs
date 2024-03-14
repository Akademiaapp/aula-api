use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingGetThreadsRes {
    pub status: Status,
    pub data: Data,
    pub version: i64,
    pub module: String,
    pub method: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub code: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub more_messages_exist: bool,
    pub page: i64,
    pub threads: Vec<Thread>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub leave_time: Value,
    pub latest_message: LatestMessage,
    pub regarding_children: Vec<Value>,
    pub creator: Creator,
    pub started_time: String,
    pub read: bool,
    pub is_thread_or_subscription_deleted: bool,
    pub subscription_id: i64,
    pub subscription_type: String,
    pub number_of_bundle_items: Value,
    pub thread_entity_link_dto: ThreadEntityLinkDto,
    pub id: i64,
    pub subject: String,
    pub recipients: Vec<Recipient>,
    pub extra_recipients_count: i64,
    pub muted: bool,
    pub marked: bool,
    pub sensitive: bool,
    pub last_read_message_id: Option<String>,
    pub is_archived: bool,
    pub mail_box_owner: MailBoxOwner3,
    pub institution_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestMessage {
    pub id: String,
    pub send_date_time: String,
    pub text: Text,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub html: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub mail_box_owner: MailBoxOwner,
    pub full_name: String,
    pub metadata: String,
    pub answer_directly_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailBoxOwner {
    pub profile_id: i64,
    pub portal_role: String,
    pub is_deactivated: bool,
    pub mail_box_owner_type: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadEntityLinkDto {
    pub entity_id: Value,
    pub thread_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    pub last_read_message_id: Option<String>,
    pub last_read_timestamp: Option<String>,
    pub leave_time: Value,
    pub deleted_at: Value,
    pub short_name: String,
    pub profile_picture_url: Value,
    pub mail_box_owner: MailBoxOwner2,
    pub full_name: String,
    pub metadata: String,
    pub answer_directly_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailBoxOwner2 {
    pub profile_id: i64,
    pub portal_role: String,
    pub is_deactivated: bool,
    pub mail_box_owner_type: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailBoxOwner3 {
    pub profile_id: i64,
    pub portal_role: String,
    pub is_deactivated: bool,
    pub mail_box_owner_type: String,
    pub id: i64,
}
