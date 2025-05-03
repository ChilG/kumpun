mod common;
mod setup;

#[test]
fn test_pattern_properties_generation() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("pattern.schema");
    setup::run_generate("cli_pattern_properties", schemas, &["--with-docs"]);

    assert!(std::path::Path::new("tests/generated/mod.rs").exists());
    let root_mod = common::read("tests/generated/mod.rs");
    assert!(root_mod.contains("pub mod pattern_schema;"));

    assert!(std::path::Path::new("tests/generated/pattern_schema.rs").exists());
    let pattern_schema = common::read("tests/generated/pattern_schema.rs");
    assert!(pattern_schema.contains("use serde::{Deserialize, Serialize};"));
    assert!(pattern_schema.contains("use std::collections::HashMap;"));
    assert!(pattern_schema.contains(
        r#"
#[derive(Debug, Serialize, Deserialize)]
pub struct PatternSchema"#
    ));
    assert!(pattern_schema.contains(
        r#"
    #[serde(flatten)]
    pub pattern_mixed: Option<HashMap<String, String>>"#
    ));
    assert!(pattern_schema.contains(
        r#"
    #[serde(flatten)]
    pub pattern_int: Option<HashMap<String, i32>>"#
    ));
    assert!(pattern_schema.contains(
        r#"
    #[serde(flatten)]
    pub pattern_json: Option<HashMap<String, serde_json::Value>>"#
    ));
    assert!(pattern_schema.contains(
        r#"
    #[serde(flatten)]
    pub pattern_flag: Option<HashMap<String, bool>>"#
    ));
    assert!(pattern_schema.contains(
        r#"
    #[serde(flatten)]
    pub pattern_num1: Option<HashMap<String, f64>>"#
    ));
}
