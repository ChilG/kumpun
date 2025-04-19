use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

#[test]
fn test_generate_rust_struct_from_schema() {
    // 1. เตรียมไฟล์ schema fixture
    let test_schema = r#"{
  "type": "object",
  "properties": {
    "email": { "type": "string" },
    "password": { "type": "string" }
  },
  "required": ["email"]
}"#;

    let schema_dir = "tests/fixtures/schemas";
    let out_dir = "src/generated";
    let schema_path = "tests/fixtures/schemas/user.login.json";
    fs::create_dir_all("tests/fixtures/schemas").unwrap();
    fs::write(schema_path, test_schema).unwrap();

    fs::create_dir_all(out_dir).unwrap();

    // 2. รัน CLI generate command
    let mut cmd = Command::cargo_bin("kumpun-cli").unwrap();

    cmd.args([
        "generate",
        "--schema",
        "user.login",
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

    // 3. เช็กว่าไฟล์ถูกสร้าง
    let generated_file = "src/generated/user_login.rs";
    assert!(
        fs::metadata(generated_file).is_ok(),
        "Generated file not found"
    );

    let content = fs::read_to_string(generated_file).unwrap();
    assert!(content.contains("pub struct UserLogin"));
    assert!(content.contains("pub email: String"));
    assert!(content.contains("pub password: Option<String>"));
}
