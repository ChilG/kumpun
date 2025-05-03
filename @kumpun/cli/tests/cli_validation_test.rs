mod common;
mod setup;

#[test]
fn test_generate_validation_schema() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("validation.schema");
    setup::run_generate("cli_simple", schemas, &["--with-validation", "--with-docs"]);

    assert!(std::path::Path::new("tests/generated/mod.rs").exists());
    let root_mod = common::read("tests/generated/mod.rs");
    assert!(root_mod.contains("pub mod validation_schema;"));

    assert!(std::path::Path::new("tests/generated/validation_schema.rs").exists());
    let validation_schema = common::read("tests/generated/validation_schema.rs");
    assert!(validation_schema.contains("use serde::{Deserialize, Serialize};"));
    assert!(validation_schema.contains(
        r#"
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ValidationSchema"#
    ));
    assert!(validation_schema.contains(
        r#"
    #[validate(range(min = 18, max = 99))]
    pub age: i32"#
    ));
    assert!(validation_schema.contains(
        r#"
    #[validate(email)]
    pub email: String"#
    ));
    assert!(validation_schema.contains(
        r#"
    #[validate(range(min = 0, max = 100))]
    pub score: Option<f64>"#
    ));
    assert!(validation_schema.contains(
        r#"
    #[validate(length(min = 3), length(max = 20), regex(path = r"^[a-zA-Z0-9_]+$"))]
    pub username: String"#
    ));
    assert!(validation_schema.contains(
        r#"
    #[validate(uuid)]
    pub uuid: Option<String>"#
    ));
}
