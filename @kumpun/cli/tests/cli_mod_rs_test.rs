mod common;
mod setup;

#[test]
fn test_mod_rs_generated() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("everything.example");
    schemas.push("main.schema");
    schemas.push("user.login");
    setup::run_generate("cli_mod_rs_test", schemas, &[]);

    assert!(std::path::Path::new("tests/generated/mod.rs").exists());
    let root_mod = common::read("tests/generated/mod.rs");
    assert!(root_mod.contains("pub mod everything_example;"));
    assert!(root_mod.contains("pub mod main_schema;"));
    assert!(root_mod.contains("pub mod shared;"));
    assert!(root_mod.contains("pub mod user_login;"));

    assert!(std::path::Path::new("tests/generated/shared/mod.rs").exists());
    let shared_mod = common::read("tests/generated/shared/mod.rs");
    assert!(shared_mod.contains("pub mod address;"));
}
