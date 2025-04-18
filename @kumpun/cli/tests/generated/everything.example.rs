use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferencesEmailOnly {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferencesSMSOnly {
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Preferences {
    EmailOnly(PreferencesEmailOnly),
    SMSOnly(PreferencesSMSOnly),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub zip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub address: Option<Address>,
    pub firstName: String,
    pub lastName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NestedRef {
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Inactive,
    Banned,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EverythingExample {
    pub age: Option<i32>,
    pub id: String,
    pub isActive: Option<bool>,
    pub meta: Option<HashMap<String, String>>,
    pub preferences: Option<Preferences>,
    pub profile: Profile,
    pub refExample: Option<NestedRef>,
    pub settings: Option<serde_json::Value>,
    pub status: Status,
    pub tags: Vec<String>,
}