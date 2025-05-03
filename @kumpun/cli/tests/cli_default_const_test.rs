mod common;
mod setup;

#[test]
fn test_default_const() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("default_const_test.schema");
    setup::run_generate("cli_default_const", schemas, &["--with-docs"]);

    assert!(std::path::Path::new("tests/generated/mod.rs").exists());
    let root_mod = common::read("tests/generated/mod.rs");
    assert!(root_mod.contains("pub mod default_const_test_schema;"));

    assert!(std::path::Path::new("tests/generated/default_const_test_schema.rs").exists());
    let schema = common::read("tests/generated/default_const_test_schema.rs");
    assert!(schema.contains("use serde::{Deserialize, Serialize};"));
    assert!(schema.contains(
        r#"
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultConstTestSchema"#
    ));
    assert!(schema.contains(
        r#"
    #[serde(default = "default_age")]
    pub age: Option<i32>"#
    ));
    assert!(schema.contains(
        r#"
    #[serde(default = "default_is_admin")]
    pub is_admin: Option<bool>"#
    ));
    assert!(schema.contains(
        r#"
    #[serde(default = "default_role")]
    pub role: Option<String>"#
    ));
    assert!(schema.contains(
        r#"
    #[serde(default = "default_status")]
    pub status: String"#
    ));
    assert!(schema.contains(
        r#"
fn default_age() -> i32 {
    18
}"#
    ));
    assert!(schema.contains(
        r#"
fn default_is_admin() -> bool {
    false
}"#
    ));
    assert!(schema.contains(
        r#"
fn default_role() -> String {
    "guest".to_string()
}"#
    ));
    assert!(schema.contains(
        r#"
fn default_status() -> String {
    "active".to_string()
}"#
    ));
}
