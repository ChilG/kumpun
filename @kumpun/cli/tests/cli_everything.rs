mod common;
mod setup;

#[test]
fn test_generate_struct_from_everything_schema() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("everything.example");
    setup::run_generate(schemas, &[]);

    assert!(std::path::Path::new("tests/generated/everything_example.rs").exists());
    let content = common::read("tests/generated/everything_example.rs");
    assert!(content.contains("pub struct EverythingExample"));
    assert!(content.contains("pub id: String"));
    assert!(content.contains("pub age: Option<i32>"));
    assert!(content.contains("pub status: Status"));
    assert!(content.contains("pub profile: Profile"));
    assert!(content.contains("pub tags: Vec<String>"));
    assert!(content.contains("pub meta: Option<HashMap<String, String>>"));
    assert!(content.contains("pub enum Status"));
    assert!(content.contains("pub enum Preferences"));
    assert!(content.contains("pub struct PreferencesEmailOnly"));
    assert!(content.contains("pub struct PreferencesSMSOnly"));
    assert!(content.contains("pub identifier: Option<Identifier>"));
    assert!(content.contains("pub enum Identifier"));
    assert!(content.contains("Variant1(String)"));
    assert!(content.contains("Variant2(i32)"));
    assert!(content.contains("pub settings: Option<Settings>"));
    assert!(content.contains("#[serde(flatten)]"));
    assert!(content.contains("pub part_1: SettingsPart1"));
    assert!(content.contains("pub part_2: SettingsPart2"));
    assert!(content.contains("use std::collections::HashMap;"));
    assert!(content.contains("use serde::{Deserialize, Serialize};"));

    assert!(std::path::Path::new("tests/generated/shared/address.rs").exists());
    let shared_mod = common::read("tests/generated/shared/address.rs");
    assert!(shared_mod.contains("use serde::{Deserialize, Serialize};"));
    assert!(shared_mod.contains("pub struct Address"));
    assert!(shared_mod.contains("pub city: Option<String>"));
    assert!(shared_mod.contains("pub zip: Option<String>"));
}
