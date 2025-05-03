mod common;
mod setup;

#[test]
fn test_generate_rust_struct_from_schema() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("user.login");
    setup::run_generate("cli_simple", schemas, &[]);

    assert!(std::path::Path::new("tests/generated/user_login.rs").exists());

    let content = common::read("tests/generated/user_login.rs");
    assert!(content.contains("pub struct UserLogin"));
    assert!(content.contains("pub email: String"));
    assert!(content.contains("pub password: Option<String>"));
}
