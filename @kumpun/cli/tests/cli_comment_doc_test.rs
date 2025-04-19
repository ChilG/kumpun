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
    assert!(content.contains("pub id: String"));

    assert!(content.contains("/// Age of the user in years."));
    assert!(content.contains("pub age: Option<i32>"));

    // ตรวจ comment ของ nested object
    assert!(content.contains("/// User's first name."));
    assert!(content.contains("pub first_name: String"));

    // ตรวจ comment จาก enum field
    assert!(content.contains("/// User status indicator."));
    assert!(content.contains("pub status: Status"));

    // ตรวจ comment จาก additionalProperties
    assert!(content.contains("/// Arbitrary metadata as key-value pairs."));
    assert!(content.contains("pub meta: Option<HashMap<String, String>>"));

    // ตรวจ comment ที่มาจาก oneOf
    assert!(content.contains("/// User communication preferences."));
    assert!(content.contains("/// User's email address."));
    assert!(content.contains("/// User prefers email communication only."));

    // ตรวจ comment ที่มาจาก allOf
    assert!(content.contains("/// Whether user allows notifications."));
    assert!(content.contains("pub notifications: Option<bool>"));

    // ตรวจ comment ที่มาจาก anyOf
    assert!(content.contains("/// String-based identifier"));
    assert!(content.contains("/// Integer-based identifier"));
}
