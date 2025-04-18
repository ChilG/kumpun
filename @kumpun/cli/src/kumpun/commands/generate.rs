pub mod schema_to_rust;

use schema_to_rust::generate_rust_structs_from_schema;
use schema_to_rust::to_pascal_case;
use schema_to_rust::write_named_structs;
use schema_to_rust::RefResolver;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn init() {
    println!("🛠️ [generate] stub generator module initialized");
}

pub fn run(schema: &str, target: &str, schema_dir: &str, out_dir: &str) {
    println!(
        "🛠️ Generating for schema: '{}', target: '{}'",
        schema, target
    );

    let schema_path = build_schema_path(schema_dir, schema);
    if !schema_path.exists() {
        eprintln!("❌ Schema file not found: {}", schema_path.display());
        std::process::exit(1);
    }

    let schema_str = match fs::read_to_string(&schema_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read schema: {}", e);
            std::process::exit(1);
        }
    };

    match target {
        "typescript" => generate_typescript_stub(schema, &schema_str, out_dir),
        "rust" => generate_rust_stub(schema, schema_dir, &schema_str, out_dir),
        _ => {
            eprintln!("❌ Unsupported target: {}", target);
            std::process::exit(1);
        }
    }
}

fn build_schema_path(schema_dir: &str, schema: &str) -> PathBuf {
    let root = std::env::current_dir().expect("Failed to get current dir");
    root.join(schema_dir).join(format!("{}.json", schema))
}

fn write_to_file(schema_name: &str, ext: &str, content: &str, out_dir: &str) {
    let out_path = Path::new(out_dir).join(format!("{}.{}", schema_name, ext));
    let parent = out_path.parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create output dir");

    let mut file = fs::File::create(&out_path).expect("Failed to write file");
    file.write_all(content.as_bytes()).expect("Write failed");

    println!("✅ Stub generated: {}", out_path.display());
}

fn generate_typescript_stub(schema_name: &str, _schema: &str, out_dir: &str) {
    let interface = format!(
        "// Auto-generated from schema: {}\nexport interface {} {{\n  // TODO: parse from schema\n}}\n",
        schema_name,
        to_pascal_case(schema_name)
    );
    write_to_file(schema_name, "ts", &interface, out_dir);
}

fn generate_rust_stub(schema_name: &str, schema_dir: &str, schema_str: &str, out_dir: &str) {
    // 1. Parse schema
    let schema: serde_json::Value = serde_json::from_str(schema_str).expect("Invalid JSON Schema");

    // 2. แปลงชื่อ schema เป็น struct name เช่น user.login → UserLogin
    let root_struct_name = to_pascal_case(schema_name);

    // 3. Prepare RefResolver
    let mut resolver = RefResolver::new(schema_dir);

    // 4. Generate all structs
    let structs = generate_rust_structs_from_schema(&root_struct_name, &schema, &mut resolver);

    // 5. Write all structs to files
    write_named_structs(&structs, out_dir, schema_name);
}
