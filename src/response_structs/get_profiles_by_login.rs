use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProfilesByLoginRes {
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
    pub profiles: Vec<Profile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub institution_profiles: Vec<InstitutionProfile>,
    pub children: Vec<Value>,
    #[serde(rename = "age18AndOlder")]
    pub age18and_older: bool,
    pub over_consent_age: bool,
    pub contact_info_editable: bool,
    pub portal_role: String,
    pub is_latest_data_policy_accepted: bool,
    pub support_role: bool,
    pub profile_id: i64,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionProfile {
    pub id: i64,
    pub profile_id: i64,
    pub institution_code: String,
    pub institution_name: String,
    pub municipality_code: String,
    pub municipality_name: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub gender: String,
    pub role: String,
    pub institution_role: Value,
    pub institution_type: Value,
    pub aula_email: String,
    pub address: Address,
    pub email: String,
    pub home_phone_number: String,
    pub mobile_phone_number: Value,
    pub work_phone_number: Value,
    pub main_group: Value,
    pub short_name: String,
    pub profile_picture_url: Value,
    pub profile_picture: ProfilePicture,
    pub new_institution_profile: bool,
    pub communication_blocked: Value,
    pub is_primary: bool,
    pub birthday: Value,
    pub institution_profile_descriptions: Value,
    pub last_activity: Value,
    pub has_custody: Value,
    pub alias: bool,
    pub groups: Value,
    pub relation: Value,
    pub is_internal_profile_picture: Value,
    pub access_level: Value,
    pub current_user_can_view_contact_information: bool,
    pub user_has_given_consent_to_show_contact_information: bool,
    pub deactivated: Value,
    pub profile_status: String,
    pub current_user_can_see_profile_description: bool,
    pub current_user_can_edit_profile_description: bool,
    pub current_user_can_edit_contact_information: bool,
    pub current_user_can_edit_profile_picture: bool,
    pub current_user_can_delete_profile_picture: bool,
    pub should_show_decline_consent_two_warning: Value,
    pub contact_type: String,
    pub has_blocked_communication_channels: bool,
    pub metadata: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub id: i64,
    pub street: String,
    pub postal_code: i64,
    pub postal_district: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePicture {
    pub id: i64,
    pub key: String,
    pub bucket: String,
    pub is_image_scaling_pending: bool,
    pub url: String,
}
