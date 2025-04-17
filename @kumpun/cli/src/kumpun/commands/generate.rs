pub mod schema_to_rust;

use serde_json::Value;
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

fn generate_rust_stub(schema_name: &str, schema_json: &str, out_dir: &str) {
    let schema: Value = serde_json::from_str(schema_json).expect("Invalid JSON schema");

    let struct_code = extract_struct(&schema, &to_pascal_case(schema_name))
        .unwrap_or_else(|| "// Failed to generate struct".to_string());

    write_to_file(schema_name, "rs", &struct_code, out_dir);
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

fn extract_struct(schema: &Value, struct_name: &str) -> Option<String> {
    let body = schema.get("properties")?.get("body")?.get("properties")?;
    let required_fields = schema
        .get("properties")?
        .get("body")?
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let mut fields = vec![];
    for (key, value) in body.as_object()? {
        let rust_type = match value.get("type")?.as_str()? {
            "string" => "String",
            "integer" => "i32",
            "number" => "f64",
            "boolean" => "bool",
            _ => "serde_json::Value",
        };

        let is_required = required_fields.contains(&key.as_str());
        let field_type = if is_required {
            rust_type.to_string()
        } else {
            format!("Option<{}>", rust_type)
        };

        fields.push(format!("    pub {}: {},", key, field_type));
    }

    Some(format!(
        "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
        struct_name,
        fields.join("\n")
    ))
}
