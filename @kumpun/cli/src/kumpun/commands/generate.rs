use std::fs;
use std::path::Path;
use std::io::Write;

pub fn init() {
    // println!("🛠️ [generate] stub generator module initialized");
}

pub fn run(schema: &str, target: &str) {
    println!("🛠️ Generating for schema: '{}', target: '{}'", schema, target);

    // 1. Build input path
    let schema_path = format!("schema/{}.json", schema);
    if !Path::new(&schema_path).exists() {
        eprintln!("❌ Schema file not found: {}", schema_path);
        std::process::exit(1);
    }

    // 2. Read schema content
    let schema_str = match fs::read_to_string(&schema_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read schema: {}", e);
            std::process::exit(1);
        }
    };

    // 3. Generate stub
    match target {
        "typescript" => generate_typescript_stub(&schema, &schema_str),
        "rust" => generate_rust_stub(&schema, &schema_str),
        _ => {
            eprintln!("❌ Unsupported target: {}", target);
            std::process::exit(1);
        }
    }
}

fn generate_typescript_stub(schema_name: &str, schema: &str) {
    let interface = format!(
        "// Auto-generated from schema: {}\nexport interface {} {{\n  // TODO: parse from schema\n}}\n",
        schema_name,
        to_pascal_case(schema_name)
    );

    write_to_file(schema_name, "ts", &interface);
}

fn generate_rust_stub(schema_name: &str, schema: &str) {
    let rust_struct = format!(
        "// Auto-generated from schema: {}\npub struct {} {{\n    // TODO: parse from schema\n}}\n",
        schema_name,
        to_pascal_case(schema_name)
    );

    write_to_file(schema_name, "rs", &rust_struct);
}

fn write_to_file(schema_name: &str, ext: &str, content: &str) {
    let out_path = format!("generated/{}.{}", schema_name, ext);
    let parent = Path::new(&out_path).parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create output dir");

    let mut file = fs::File::create(&out_path).expect("Failed to write file");
    file.write_all(content.as_bytes()).expect("Write failed");

    println!("✅ Stub generated: {}", out_path);
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
