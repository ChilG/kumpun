use assert_cmd::Command;

pub fn run_generate(schemas: Vec<&str>, extra_args: &[&str]) {
    let _ = std::fs::remove_dir_all("tests/generated");
    std::fs::create_dir_all("tests/generated").unwrap();

    for schema in schemas {
        let mut cmd = Command::cargo_bin("kumpun-cli").unwrap();

        let base_args = [
            "generate",
            "--schema",
            schema,
            "--target",
            "rust",
            "--schema-dir",
            "tests/fixtures/schemas",
            "--out-dir",
            "tests/generated",
        ];

        cmd.args(base_args.iter().chain(extra_args.iter()));

        cmd.assert().success();
    }
}
