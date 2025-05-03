mod common;
mod setup;

#[test]
fn test_snake_case_filename_and_fields() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("everything.example");
    setup::run_generate("cli_snake_case_test", schemas, &[]);

    assert!(std::path::Path::new("tests/generated/everything_example.rs").exists());

    let content = common::read("tests/generated/everything_example.rs");
    assert!(content.contains("pub first_name: String"));
    assert!(content.contains("pub last_name: String"));
    assert!(content.contains("pub part_1: SettingsPart1"));
    assert!(content.contains("pub part_2: SettingsPart2"));
    assert!(content.contains("pub is_active: Option<bool>"));
    assert!(content.contains("pub ref_example: Option<NestedRef>"));
}
