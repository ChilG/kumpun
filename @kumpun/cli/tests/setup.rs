use assert_cmd::Command;
use chrono::Local;
use std::{fs, io::Write, path::PathBuf};

pub fn run_generate(name: &str, schemas: Vec<&str>, extra_args: &[&str]) {
    let _ = fs::remove_dir_all("tests/generated");
    fs::create_dir_all("tests/generated").unwrap();

    let log_dir = PathBuf::from(".tmp/test-logs");
    fs::create_dir_all(&log_dir).unwrap();

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

        let timestamp = Local::now().format("%Y%m%d-%H%M%S");
        let log_path = log_dir.join(format!("{}-{}-{}.log", name, schema, timestamp));

        let mut log_file = fs::File::create(&log_path).expect("Failed to create log file");

        let output = cmd
            .env("RUST_LOG", "debug,error")
            .args(base_args.iter().chain(extra_args.iter()))
            .output() // ✅ ใช้ get_output() จาก assert_cmd
            .expect("Failed to run command");

        log_file
            .write_all(&output.stdout)
            .expect("Failed to write stdout");
        log_file
            .write_all(&output.stderr)
            .expect("Failed to write stderr");

        assert!(
            output.status.success(),
            "Command failed. See log: {}",
            log_path.display()
        );
    }
}
