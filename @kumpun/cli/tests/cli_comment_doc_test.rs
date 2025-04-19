mod common;
mod setup;

#[test]
fn test_struct_field_doc_comments_are_generated() {
    let mut schemas: Vec<&str> = vec![];
    schemas.push("everything.example");
    setup::run_generate(schemas, &["--with-docs"]);

    let content = common::read("tests/generated/everything_example.rs");

    // ตรวจ comment ของ field ทั่วไป
    assert!(content.contains("/// Unique identifier (UUID) for the entity."));
    assert!(content.contains(r#"/// Example: "550e8400-e29b-41d4-a716-446655440000""#));
    assert!(content.contains("pub id: String"));

    assert!(content.contains("/// Age of the user in years."));
    assert!(content.contains(r#"/// Example: [{"name":"Smartphone","price":699},{"name":"Smartwatch","price":199}]"#));
    assert!(content.contains("pub age: Option<i32>"));

    // ตรวจ comment ของ nested object
    assert!(content.contains("/// User's first name."));
    assert!(content.contains(r#"/// Example: "Nantapon""#));
    assert!(content.contains("pub first_name: String"));

    // ตรวจ comment จาก enum field
    assert!(content.contains("/// User status indicator."));
    assert!(content.contains("pub status: Status"));

    // ตรวจ comment จาก additionalProperties
    assert!(content.contains("/// Arbitrary metadata as key-value pairs."));
    assert!(content.contains(r#"/// Example: {"language":"th","nickname":"nanta"}"#));
    assert!(content.contains("pub meta: Option<HashMap<String, String>>"));

    // ตรวจ comment ที่มาจาก oneOf
    assert!(content.contains("/// User communication preferences."));
    assert!(content.contains(r#"/// Example: {"email":"user@example.com"}"#));
    assert!(content.contains("pub preferences: Option<Preferences>"));
    assert!(content.contains("/// User prefers email communication only."));
    assert!(content.contains(r#"/// Example: {"email":"user@example.com"}"#));
    assert!(content.contains("/// User's email address."));
    assert!(content.contains(r#"/// Example: "user@example.com""#));

    // ตรวจ comment ที่มาจาก allOf
    assert!(content.contains("/// User configurable settings."));
    assert!(content.contains(r#"/// Example: {"notifications":true,"theme":"dark"}"#));
    assert!(content.contains("pub settings: Option<Settings>"));
    assert!(content.contains("/// UI theme setting."));
    assert!(content.contains(r#"/// Example: {"theme":"dark"}"#));
    assert!(content.contains("pub part_1: SettingsPart1"));
    assert!(content.contains("/// User's theme preference."));
    assert!(content.contains(r#"/// Example: "dark""#));
    assert!(content.contains("pub theme: Option<String>"));

    // ตรวจ comment ที่มาจาก anyOf
    assert!(content.contains("/// Flexible identifier which could be a string or number."));
    assert!(content.contains(r#"/// Example: "user_001""#));
    assert!(content.contains("pub identifier: Option<Identifier>"));
    assert!(content.contains("/// String-based identifier."));
    assert!(content.contains(r#"/// Example: "user_001""#));
    assert!(content.contains("Variant1(String)"));
}
