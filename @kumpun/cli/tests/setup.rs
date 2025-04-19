use assert_cmd::Command;

pub fn run_generate(schemas: Vec<&str>) {
    let _ = std::fs::remove_dir_all("tests/generated");
    std::fs::create_dir_all("tests/generated").unwrap();

    for schema in schemas {
        Command::cargo_bin("kumpun-cli")
            .unwrap()
            .args([
                "generate",
                "--schema",
                schema,
                "--target",
                "rust",
                "--schema-dir",
                "tests/fixtures/schemas",
                "--out-dir",
                "tests/generated",
            ])
            .assert()
            .success();
    }
}
