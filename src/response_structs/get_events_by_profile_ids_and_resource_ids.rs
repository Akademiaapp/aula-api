use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsByProfileIdsAndResourceIdsRes {
    pub status: Status,
    pub data: Vec<Daum>,
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
pub struct Daum {
    pub creator_inst_profile_id: Value,
    pub creator_profile_id: Value,
    pub invited_groups: Vec<InvitedGroup>,
    pub primary_resource: Option<PrimaryResource>,
    pub has_attachments: bool,
    pub created_date_time: String,
    pub lesson: Lesson,
    pub time_slot: Value,
    pub vacation_children_count_by_dates: Value,
    pub belongs_to_profiles: Vec<i64>,
    pub belongs_to_resources: Vec<Value>,
    pub old_all_day: Value,
    pub requires_new_answer: bool,
    pub directly_related: bool,
    pub response_deadline: Value,
    pub response_status: Value,
    pub id: i64,
    pub title: String,
    pub all_day: bool,
    pub start_date_time: String,
    pub end_date_time: String,
    pub old_end_date_time: Value,
    pub old_start_date_time: Value,
    pub response_required: bool,
    pub private: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub primary_resource_text: Option<String>,
    pub additional_resources: Vec<Value>,
    pub additional_resource_text: Value,
    pub repeating: Value,
    pub institution_code: Value,
    pub institution_name: Value,
    pub added_to_institution_calendar: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitedGroup {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub institution_code: String,
    pub institution_name: String,
    pub main_group: bool,
    pub uni_group_type: String,
    pub is_deactivated: bool,
    pub allow_members_to_be_shown: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryResource {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    pub lesson_id: String,
    pub lesson_status: String,
    pub participants: Vec<Participant>,
    pub has_relevant_note: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub teacher_id: i64,
    pub teacher_name: String,
    pub teacher_initials: String,
    pub participant_role: String,
}
