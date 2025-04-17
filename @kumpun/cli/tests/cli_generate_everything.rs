use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

#[test]
fn test_generate_struct_from_everything_schema() {
    // 1. เตรียม schema ไฟล์
    let schema = include_str!("../tests/fixtures/schemas/everything.example.json");
    fs::create_dir_all("tests/fixtures/schemas").unwrap();
    fs::write("tests/fixtures/schemas/everything.example.json", schema).unwrap();

    // 2. ลบไฟล์เดิมที่ generate แล้ว (เพื่อความสะอาด)
    let _ = fs::remove_file("tests/generated/everything.example.rs");

    // 3. เรียก CLI generate
    let mut cmd = Command::cargo_bin("kumpun-cli").unwrap();
    cmd.args([
        "generate",
        "--schema", "everything.example",
        "--target", "rust",
        "--schema-dir", "tests/fixtures/schemas",
        "--out-dir", "tests/generated"
    ])
        .assert()
        .success()
        .stdout(contains("✅ Stub generated"));

    // 4. ตรวจสอบ output file ถูกสร้าง
    let output = fs::read_to_string("tests/generated/everything.example.rs")
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
}
