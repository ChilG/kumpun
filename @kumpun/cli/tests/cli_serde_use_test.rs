mod common;
mod setup;

#[test]
fn test_serde_use_added() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("main.schema");
    setup::run_generate("cli_serde_use_test", schemas, &[]);

    assert!(std::path::Path::new("tests/generated/main_schema.rs").exists());
    let content = common::read("tests/generated/main_schema.rs");
    assert!(content.contains("use serde::{Deserialize, Serialize};"));

    assert!(std::path::Path::new("tests/generated/shared/address.rs").exists());
    let shared_mod = common::read("tests/generated/shared/address.rs");
    assert!(shared_mod.contains("use serde::{Deserialize, Serialize};"));
}
