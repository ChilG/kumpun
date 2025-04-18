use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

#[test]
fn test_cross_file_ref_generation() {
    let schema_dir = "tests/fixtures/schemas";
    let out_dir = "tests/generated";
    let out_main_path = "tests/generated/main.schema.rs";
    let out_shared_address_path = "tests/generated/shared/address.rs";

    fs::create_dir_all("tests/generated").unwrap();

    let mut cmd = Command::cargo_bin("kumpun-cli").unwrap();
    cmd.args([
        "generate",
        "--schema",
        "main.schema",
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

    let output_main = fs::read_to_string(&out_main_path).expect("generated file main not found");
    let output_shared_address = fs::read_to_string(&out_shared_address_path)
        .expect("generated file shared/address not found");

    // ✅ ตรวจว่าทั้ง struct หลักและที่ import มาถูก generate
    assert!(output_main.contains("pub struct MainSchema")); // struct หลัก
    assert!(output_main.contains("pub address: Address")); // field ที่อ้างถึงไฟล์อื่น
    assert!(output_shared_address.contains("pub struct Address")); // จากไฟล์ shared/address.json
    assert!(output_shared_address.contains("pub city: Option<String>")); // field ภายใน Address
}
