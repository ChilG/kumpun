mod common;
mod setup;

#[test]
fn test_with_json_schema_org() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("draft-04.schema");
    schemas.push("draft-06.schema");
    schemas.push("draft-07.schema");
    schemas.push("draft.2019-09.schema");
    schemas.push("draft.2020-12.schema");
    setup::run_generate("cli_json_schema_org", schemas, &["--with-docs"]);

    assert!(std::path::Path::new("tests/generated/mod.rs").exists());
    let root_mod = common::read("tests/generated/mod.rs");
    let expected_mods = vec![
        "draft_04_schema",
        "draft_06_schema",
        "draft_07_schema",
        "draft_2019_09_schema",
        "draft_2020_12_schema",
    ];

    for module in expected_mods {
        let line = format!("pub mod {};", module);
        assert!(
            root_mod.contains(&line),
            "❌ expected mod.rs to contain '{}'",
            line
        );
    }
}
