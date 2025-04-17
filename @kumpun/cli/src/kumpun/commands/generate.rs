use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

pub fn init() {
    println!("🛠️ [generate] stub generator module initialized");
}

pub fn run(schema: &str, target: &str, schema_dir: &str, out_dir: &str) {
    println!("🛠️ Generating for schema: '{}', target: '{}'", schema, target);

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
        "rust" => generate_rust_stub(schema, &schema_str, out_dir),
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

fn generate_rust_stub(schema_name: &str, _schema: &str, out_dir: &str) {
    let rust_struct = format!(
        "// Auto-generated from schema: {}\npub struct {} {{\n    // TODO: parse from schema\n}}\n",
        schema_name,
        to_pascal_case(schema_name)
    );
    write_to_file(schema_name, "rs", &rust_struct, out_dir);
}

fn to_pascal_case(name: &str) -> String {
    name.split('.')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}
