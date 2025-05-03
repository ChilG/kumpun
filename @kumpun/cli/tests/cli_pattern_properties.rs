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
/// Schema with patternProperties for dynamic keys.
#[derive(Debug, Serialize, Deserialize)]
pub struct PatternSchema {
    /// Keys matching pattern: `^num_`
    #[serde(flatten)]
    pub pattern_1: Option<HashMap<String, f64>>,
    /// Keys matching pattern: `^str_`
    #[serde(flatten)]
    pub pattern_2: Option<HashMap<String, String>>,
}"#
    ));
}
