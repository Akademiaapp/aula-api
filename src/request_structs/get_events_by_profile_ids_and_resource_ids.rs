use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsByProfileIdsAndResourceIdsReq {
    pub inst_profile_ids: Vec<i64>,
    pub resource_ids: Vec<Value>,
    pub start: String,
    pub end: String,
}
