use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub success: bool,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub id: i64,
    #[serde(rename = "application_id")]
    pub application_id: i64,
    pub name: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub headline: String,
    pub summary: Option<String>,
    #[serde(rename = "job_title")]
    pub job_title: String,
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "job_industry")]
    pub job_industry: Value,
    pub location: String,
    #[serde(rename = "location_code")]
    pub location_code: String,
    #[serde(rename = "picture_url")]
    pub picture_url: Option<String>,
    #[serde(rename = "type_id")]
    pub type_id: i64,
    #[serde(rename = "type_category_id")]
    pub type_category_id: i64,
    #[serde(rename = "type_key")]
    pub type_key: Option<String>,
    #[serde(rename = "type_key_translation")]
    pub type_key_translation: String,
    pub active: i64,
    #[serde(rename = "needs_activation")]
    pub needs_activation: i64,
    #[serde(rename = "date_created")]
    pub date_created: i64,
    #[serde(rename = "can_meet")]
    pub can_meet: i64,
    #[serde(rename = "can_swipe")]
    pub can_swipe: i64,
    #[serde(rename = "date_updated")]
    pub date_updated: Option<i64>,
    pub uri: String,
    pub words: Value,
    #[serde(rename = "container_id")]
    pub container_id: Vec<i64>,
    #[serde(rename = "sessions_speaking")]
    pub sessions_speaking: Vec<i64>,
    #[serde(rename = "sessions_attending")]
    pub sessions_attending: Vec<i64>,
    pub types: Vec<Type>,
    #[serde(rename = "promotion_locations")]
    pub promotion_locations: Vec<Value>,
    #[serde(rename = "group_ids")]
    pub group_ids: Vec<i64>,
    pub groups: Vec<Group>,
    #[serde(rename = "type_key_translation_en-gb")]
    pub type_key_translation_en_gb: String,
    #[serde(rename = "type_key_translation_en")]
    pub type_key_translation_en: String,
    pub promoted: i64,
    #[serde(rename = "promotion_level")]
    pub promotion_level: String,
    #[serde(rename = "default_meeting_location")]
    pub default_meeting_location: Option<String>,
    #[serde(rename = "registration_id")]
    pub registration_id: Value,
    #[serde(rename = "gps_lat")]
    pub gps_lat: Value,
    #[serde(rename = "gps_lng")]
    pub gps_lng: Value,
    #[serde(rename = "location_lat")]
    pub location_lat: Value,
    #[serde(rename = "location_lng")]
    pub location_lng: Value,
    pub metadata: Value,
    pub categories: Value,
    #[serde(rename = "did_answer_yes")]
    pub did_answer_yes: i64,
    #[serde(rename = "swipe_message")]
    pub swipe_message: Value,
    #[serde(rename = "swipe_message_date_sent")]
    pub swipe_message_date_sent: Value,
    #[serde(rename = "default_access")]
    pub default_access: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    #[serde(rename = "container_id")]
    pub container_id: i64,
    #[serde(rename = "type_id")]
    pub type_id: i64,
    #[serde(rename = "can_meet")]
    pub can_meet: String,
    #[serde(rename = "can_swipe")]
    pub can_swipe: String,
    #[serde(rename = "type_key_translation_en")]
    pub type_key_translation_en: String,
    #[serde(rename = "type_key_translation")]
    pub type_key_translation: String,
    #[serde(rename = "type_key_translation_en-gb")]
    pub type_key_translation_en_gb: String,
    #[serde(rename = "type_key")]
    pub type_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "group_id")]
    pub group_id: i64,
    #[serde(rename = "group_name")]
    pub group_name: String,
    pub access: Value,
    #[serde(rename = "is_speed_networking_group")]
    pub is_speed_networking_group: i64,
}
