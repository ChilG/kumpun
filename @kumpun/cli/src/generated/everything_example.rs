use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Identifier {
    Variant1(String),
    Variant2(i32),
}

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
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NestedRef {
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPart1 {
    pub theme: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPart2 {
    pub notifications: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub part_1: SettingsPart1,
    #[serde(flatten)]
    pub part_2: SettingsPart2,
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
    pub identifier: Option<Identifier>,
    pub is_active: Option<bool>,
    pub meta: Option<HashMap<String, String>>,
    pub preferences: Option<Preferences>,
    pub profile: Profile,
    pub ref_example: Option<NestedRef>,
    pub settings: Option<Settings>,
    pub status: Status,
    pub tags: Vec<String>,
}