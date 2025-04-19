use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

#[test]
fn test_generate_struct_from_everything_schema() {
    // 1. เตรียม schema ไฟล์
    let schema_dir = "tests/fixtures/schemas";
    let out_dir = "src/generated";
    let schema = include_str!("../tests/fixtures/schemas/everything.example.json");
    fs::create_dir_all(schema_dir).unwrap();
    fs::create_dir_all(out_dir).unwrap();
    fs::write("tests/fixtures/schemas/everything.example.json", schema).unwrap();
    // 2. ลบไฟล์เดิมที่ generate แล้ว (เพื่อความสะอาด)
    let _ = fs::remove_file("tests/generated/everything_example");

    // 3. เรียก CLI generate
    let mut cmd = Command::cargo_bin("kumpun-cli").unwrap();
    cmd.args([
        "generate",
        "--schema",
        "everything.example",
        "--target",
        "rust",
        "--schema-dir",
        schema_dir,
        "--out-dir",
        out_dir,
    ])
    .assert()
    .success()
    .stdout(contains("✅ Stub generated"));

    // 4. ตรวจสอบ output file ถูกสร้าง
    let output = fs::read_to_string("src/generated/everything_example.rs")
        .expect("generated file not found");

    // 5. ตรวจว่า struct สำคัญถูก generate มาครบ
    assert!(output.contains("pub struct EverythingExample"));
    assert!(output.contains("pub id: String"));
    assert!(output.contains("pub age: Option<i32>"));
    assert!(output.contains("pub status: Status"));
    assert!(output.contains("pub profile: Profile"));
    assert!(output.contains("pub tags: Vec<String>"));
    assert!(output.contains("pub meta: Option<HashMap<String, String>>")); // หากรองรับ
    assert!(output.contains("pub enum Status")); // ตรวจ enum
    assert!(output.contains("pub enum Preferences")); // หากรองรับ oneOf
    assert!(output.contains("pub struct PreferencesEmailOnly"));
    assert!(output.contains("pub struct PreferencesSMSOnly"));
    assert!(output.contains("pub identifier: Option<Identifier>"));
    assert!(output.contains("pub enum Identifier"));
    assert!(output.contains("Variant1(String)"));
    assert!(output.contains("Variant2(i32)"));
    assert!(output.contains("pub settings: Option<Settings>"));
    assert!(output.contains("#[serde(flatten)]"));
    assert!(output.contains("pub part_1: SettingsPart1"));
    assert!(output.contains("pub part_2: SettingsPart2"));
    assert!(output.contains("use std::collections::HashMap;"));
}
