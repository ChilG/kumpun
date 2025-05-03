mod common;
mod setup;

#[test]
fn test_cross_file_ref_generation() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("main.schema");
    setup::run_generate("cli_cross_file_ref", schemas, &[]);

    let content = common::read("tests/generated/main_schema.rs");
    assert!(content.contains("use serde::{Deserialize, Serialize};"));
    assert!(content.contains("pub struct MainSchema"));
    assert!(content.contains("pub address: Address"));

    let shared_mod = common::read("tests/generated/shared/address.rs");
    assert!(shared_mod.contains("use serde::{Deserialize, Serialize};"));
    assert!(shared_mod.contains("pub struct Address"));
    assert!(shared_mod.contains("pub city: Option<String>"));
}
