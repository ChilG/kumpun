use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
#[allow(dead_code)]
pub struct NamedStruct {
    pub name: String,
    pub code: String,
    pub output_path: Option<String>,
}

pub struct RefResolver {
    base_path: PathBuf,
    cache: HashMap<String, Value>,
}

impl RefResolver {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            cache: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, ref_str: &str) -> Option<Value> {
        if ref_str.starts_with("#/") {
            return None;
        }

        let parts: Vec<&str> = ref_str.split('#').collect();
        let file_path = parts[0];
        let pointer = parts.get(1).map(|s| *s).unwrap_or("");

        let full_path = self.base_path.join(file_path);
        println!("📂 Looking for: {}", full_path.display());

        let content = if let Some(cached) = self.cache.get(file_path) {
            cached.clone()
        } else {
            let raw = fs::read_to_string(&full_path).ok()?;
            let parsed: Value = serde_json::from_str(&raw).ok()?;
            self.cache.insert(file_path.to_string(), parsed.clone());
            parsed
        };

        if pointer.is_empty() {
            Some(content)
        } else {
            content
                .pointer(&format!(
                    "/{}",
                    pointer.trim_start_matches('/').replace("~1", "/")
                ))
                .cloned()
        }
    }
}

pub fn write_named_structs(structs: &[NamedStruct], out_dir: &str, root_name: &str) {
    let mut root_code = vec![];
    let mut root_needs_serde = false;

    for s in structs {
        println!("🧾 writing {} → {:?}", s.name, s.output_path);
        match &s.output_path {
            Some(path_hint) => {
                let snake_case_path = to_snake_case(path_hint);
                let full_path = Path::new(out_dir).join(format!("{}.rs", snake_case_path));
                let parent = full_path.parent().unwrap();
                fs::create_dir_all(parent).expect("Failed to create output directory");

                let mut file = fs::File::create(&full_path).expect("Failed to create output file");

                let code = if s.code.contains("Serialize") || s.code.contains("Deserialize") {
                    format!("use serde::{{Deserialize, Serialize}};\n\n{}", s.code)
                } else {
                    s.code.clone()
                };

                file.write_all(code.as_bytes()).expect("Write failed");
                println!("✅ Generated: {}", full_path.display());
            }
            None => {
                if s.code.contains("Serialize") || s.code.contains("Deserialize") {
                    root_needs_serde = true;
                }
                root_code.push(s.code.as_str());
            }
        }
    }

    let full_path = Path::new(out_dir).join(format!("{}.rs", to_snake_case(root_name)));
    let parent = full_path.parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create output directory");

    let mut file = fs::File::create(&full_path).expect("Failed to write root output");

    let mut joined = String::new();
    if root_needs_serde {
        joined.push_str("use serde::{Deserialize, Serialize};\n\n");
    }
    joined.push_str(&root_code.join("\n\n"));

    file.write_all(joined.as_bytes())
        .expect("Root write failed");

    println!("✅ Stub generated: {}", full_path.display());

    // 🔧 NEW: Generate mod.rs files for all subfolders
    generate_mod_rs_recursively(Path::new(out_dir)).expect("Failed to generate mod.rs files");
}

fn generate_mod_rs_recursively(dir: &Path) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let mut mod_lines = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            generate_mod_rs_recursively(&path)?;
            if path.join("mod.rs").exists() {
                mod_lines.push(format!(
                    "pub mod {};",
                    path.file_name().unwrap().to_str().unwrap()
                ));
            }
        } else if path.is_file() {
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                if name != "mod" && path.extension().map_or(false, |e| e == "rs") {
                    mod_lines.push(format!("pub mod {};", name));
                }
            }
        }
    }

    mod_lines.sort();
    mod_lines.dedup();

    let mod_path = dir.join("mod.rs");
    let content = mod_lines.join("\n") + "\n";
    fs::write(&mod_path, content)?;
    println!("📦 mod.rs generated: {}", mod_path.display());

    Ok(())
}

pub fn generate_rust_structs_from_schema(
    root_name: &str,
    schema: &Value,
    resolver: &mut RefResolver,
    with_docs: &bool,
) -> Vec<NamedStruct> {
    let mut structs = vec![];
    let mut visited = HashSet::new();
    let definitions = schema.get("definitions").cloned().unwrap_or(Value::Null);

    extract_struct_recursive(
        root_name,
        schema,
        &mut structs,
        &mut visited,
        "#".to_string(),
        &definitions,
        resolver,
        None,
        *with_docs,
    );

    let mut use_lines = vec![];
    for s in &structs {
        let is_root_file = s.output_path.is_none() || s.output_path.as_deref() == Some(root_name);
        if (s.output_path.is_none() || is_root_file) && s.code.contains("HashMap<") {
            use_lines.push("use std::collections::HashMap;".to_string());
        }
    }

    use_lines.sort();
    use_lines.dedup();

    let mut import_uses = vec![];
    for s in &structs {
        if let Some(ref path) = s.output_path {
            if path != root_name {
                let mod_path = format!("generated::{}", path.replace('/', "::"));
                import_uses.push(format!("use crate::{}::{};", mod_path, s.name));
            }
        }
    }

    import_uses.sort();
    import_uses.dedup();

    for (i, line) in import_uses
        .into_iter()
        .chain(use_lines.into_iter())
        .rev()
        .enumerate()
    {
        structs.insert(
            0,
            NamedStruct {
                name: format!("__use_{}", i),
                code: line,
                output_path: None,
            },
        );
    }

    for s in &structs {
        println!("🧾 {} → {:?}", s.name, s.output_path);
    }

    structs
}

fn extract_struct_recursive(
    name: &str,
    schema: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    _path: String,
    definitions: &Value,
    resolver: &mut RefResolver,
    output_path: Option<String>,
    with_docs: bool,
) {
    println!("📦 Generated: {} → {:?}", name, output_path);
    if visited.contains(name) {
        return;
    }
    visited.insert(name.to_string());

    let Some(properties) = schema.get("properties") else {
        return;
    };
    let required = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|s| s.as_str())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default();

    let mut fields = vec![];

    for (key, prop) in properties.as_object().unwrap() {
        let field_name = to_snake_case(key);
        let is_required = required.contains(key.as_str());
        println!("📦 field: {}", key);
        let rust_type = infer_rust_type(
            prop,
            key,
            output,
            visited,
            definitions,
            resolver,
            output_path.clone(),
            with_docs,
        )
        .unwrap_or_else(|| "serde_json::Value".to_string());

        let final_type = if is_required {
            rust_type
        } else {
            format!("Option<{}>", rust_type)
        };

        if with_docs {
            let doc_block = doc_lines_to_string_block(prop, 4);
            if !doc_block.is_empty() {
                fields.push(doc_block);
            }
        }

        fields.push(format!("    pub {}: {},", field_name, final_type));
    }

    let mut struct_lines = vec![];
    if with_docs {
        if let Some(desc) = schema.get("description").and_then(|d| d.as_str()) {
            struct_lines.push(format!("/// {}", desc));
        } else if let Some(title) = schema.get("title").and_then(|t| t.as_str()) {
            struct_lines.push(format!("/// {}", title));
        }
    }

    struct_lines.push(format!(
        "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
        name,
        fields.join("\n")
    ));

    let struct_code = struct_lines.join("\n");

    output.push(NamedStruct {
        name: name.to_string(),
        code: struct_code,
        output_path: output_path.clone(),
    });
}

fn infer_rust_type(
    prop: &Value,
    key: &str,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
    resolver: &mut RefResolver,
    output_path: Option<String>,
    with_docs: bool,
) -> Option<String> {
    println!("🧪 infer_rust_type: key = {}, prop = {}", key, prop);
    if let Some(ref_val) = prop.get("$ref").and_then(|v| v.as_str()) {
        return if ref_val.starts_with("#/") {
            let name = ref_val.split('/').last()?.to_string();
            let def = definitions.get(&name)?;
            extract_struct_recursive(
                &name,
                def,
                output,
                visited,
                ref_val.to_string(),
                definitions,
                resolver,
                output_path.clone(),
                with_docs,
            );
            Some(name)
        } else {
            let resolved = resolver.resolve(ref_val)?;
            println!("🧩 RESOLVED: {} → {}", ref_val, resolved);
            let name = to_pascal_case(Path::new(ref_val).file_stem()?.to_str()?);

            let path_no_ext = Path::new(ref_val).with_extension("");
            let ref_output_path = Some(path_no_ext.to_string_lossy().replace('\\', "/"));

            extract_struct_recursive(
                &name,
                &resolved,
                output,
                visited,
                ref_val.to_string(),
                &Value::Null,
                resolver,
                ref_output_path.clone(),
                with_docs,
            );
            Some(name)
        };
    }

    if let Some(one_of) = prop.get("oneOf") {
        return handle_one_of(
            key,
            one_of,
            output,
            visited,
            definitions,
            resolver,
            output_path.clone(),
            with_docs,
        );
    }
    if let Some(any_of) = prop.get("anyOf") {
        return handle_any_of(
            key,
            any_of,
            output,
            visited,
            definitions,
            resolver,
            output_path.clone(),
            with_docs,
        );
    }
    if let Some(all_of) = prop.get("allOf") {
        return handle_all_of(
            key,
            all_of,
            output,
            visited,
            definitions,
            resolver,
            output_path.clone(),
            with_docs,
        );
    }

    match prop.get("type")?.as_str()? {
        "string" => {
            if let Some(enum_vals) = prop.get("enum") {
                let enum_name = to_pascal_case(key);
                let variants = enum_vals
                    .as_array()?
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|v| format!("    {},", to_pascal_case(v)))
                    .collect::<Vec<_>>()
                    .join("\n");
                let code = format!(
                    "#[derive(Debug, Serialize, Deserialize)]\npub enum {} {{\n{}\n}}",
                    enum_name, variants
                );
                output.push(NamedStruct {
                    name: enum_name.clone(),
                    code,
                    output_path,
                });
                Some(enum_name)
            } else {
                Some("String".to_string())
            }
        }
        "integer" => Some("i32".to_string()),
        "number" => Some("f64".to_string()),
        "boolean" => Some("bool".to_string()),
        "array" => {
            let items = prop.get("items")?;
            let inner = infer_rust_type(
                items,
                &format!("{}Item", key),
                output,
                visited,
                definitions,
                resolver,
                output_path.clone(),
                with_docs,
            )?;
            Some(format!("Vec<{}>", inner))
        }
        "object" => {
            if let Some(ap) = prop.get("additionalProperties") {
                let inner_type = infer_rust_type(
                    ap,
                    &format!("{}Value", key),
                    output,
                    visited,
                    definitions,
                    resolver,
                    output_path.clone(),
                    with_docs,
                )
                .unwrap_or_else(|| "serde_json::Value".to_string());
                return Some(format!("Option<HashMap<String, {}>>", inner_type));
            }
            if prop.get("properties").is_some() {
                let sub_name = to_pascal_case(key);
                extract_struct_recursive(
                    &sub_name,
                    prop,
                    output,
                    visited,
                    "#".to_string(),
                    definitions,
                    resolver,
                    output_path.clone(),
                    with_docs,
                );
                return Some(sub_name);
            }
            Some("serde_json::Value".to_string())
        }
        _ => Some("serde_json::Value".to_string()),
    }
}

fn extract_top_description(items: &Value) -> Option<String> {
    let arr = items.as_array()?;
    let top = arr.get(0)?.get("description")?.as_str()?;

    for v in arr.iter().skip(1) {
        if let Some(desc) = v.get("description").and_then(|d| d.as_str()) {
            if desc != top {
                return Some(format!("/// {}", top));
            }
        }
    }

    None
}

fn generate_doc_lines(schema: &Value) -> Vec<String> {
    let mut lines = vec![];

    if let Some(desc) = schema.get("description").and_then(|d| d.as_str()) {
        lines.push(format!("/// {}", desc));
    }

    if let Some(example) = schema
        .get("examples")
        .and_then(|e| e.as_array())
        .and_then(|arr| arr.get(0))
    {
        let rendered = match example {
            Value::String(s) => format!("\"{}\"", s),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            other => other.to_string(),
        };
        lines.push(format!("/// Example: {}", rendered));
    }

    lines
}

fn doc_lines_to_string_block(schema: &Value, indent: usize) -> String {
    let prefix = " ".repeat(indent);
    generate_doc_lines(schema)
        .into_iter()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn handle_one_of(
    key: &str,
    one_of: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
    resolver: &mut RefResolver,
    output_path: Option<String>,
    with_docs: bool,
) -> Option<String> {
    let enum_name = to_pascal_case(key);
    let mut variants = vec![];

    for variant in one_of.as_array()? {
        let title = variant
            .get("title")
            .and_then(|t| t.as_str())
            .map(|s| to_pascal_case(s))
            .unwrap_or_else(|| format!("Variant{}", variants.len() + 1));

        let struct_name = format!("{}{}", enum_name, &title);

        if variant.get("type") == Some(&Value::String("object".into())) {
            extract_struct_recursive(
                &struct_name,
                variant,
                output,
                visited,
                "#".to_string(),
                definitions,
                resolver,
                output_path.clone(),
                with_docs,
            );

            if let Some(last) = output.last_mut() {
                if last.name == struct_name {
                    last.code = format!("{}", last.code);
                }
            }

            if with_docs {
                let doc_block = doc_lines_to_string_block(variant, 4);
                if !doc_block.is_empty() {
                    variants.push(doc_block);
                }
                variants.push(format!("    {}({}),", title, struct_name));
            } else {
                variants.push(format!("    {}({}),", title, struct_name));
            }
        } else {
            let inner_type = infer_rust_type(
                variant,
                &title,
                output,
                visited,
                definitions,
                resolver,
                output_path.clone(),
                with_docs,
            )
            .unwrap_or_else(|| "serde_json::Value".to_string());
            variants.push(format!("    {}({}),", title, inner_type));
        }
    }

    let mut lines = vec![];
    lines.push(format!(
        "#[derive(Debug, Serialize, Deserialize)]\n#[serde(tag = \"type\")]\npub enum {} {{\n{}\n}}",
        enum_name,
        variants.join("\n")
    ));
    let enum_code = lines.join("\n");

    output.push(NamedStruct {
        name: enum_name.clone(),
        code: enum_code,
        output_path,
    });
    Some(enum_name)
}

fn handle_any_of(
    key: &str,
    any_of: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
    resolver: &mut RefResolver,
    output_path: Option<String>,
    with_docs: bool,
) -> Option<String> {
    let enum_name = to_pascal_case(key);
    let mut variants = vec![];

    for (i, variant) in any_of.as_array()?.iter().enumerate() {
        let var_name = format!("Variant{}", i + 1);

        let inner_type = infer_rust_type(
            variant,
            &var_name,
            output,
            visited,
            definitions,
            resolver,
            output_path.clone(),
            with_docs,
        )
        .unwrap_or_else(|| "serde_json::Value".to_string());

        if with_docs {
            let doc_block = doc_lines_to_string_block(variant, 4);
            if !doc_block.is_empty() {
                variants.push(doc_block);
            }
            variants.push(format!("    {}({}),", var_name, inner_type));
        } else {
            variants.push(format!("    {}({}),", var_name, inner_type));
        }
    }

    let mut lines = vec![];
    lines.push(format!(
        "#[derive(Debug, Serialize, Deserialize)]\n#[serde(untagged)]\npub enum {} {{\n{}\n}}",
        enum_name,
        variants.join("\n")
    ));
    let code = lines.join("\n");

    output.push(NamedStruct {
        name: enum_name.clone(),
        code,
        output_path,
    });
    Some(enum_name)
}

fn handle_all_of(
    key: &str,
    all_of: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
    resolver: &mut RefResolver,
    output_path: Option<String>,
    with_docs: bool,
) -> Option<String> {
    let main_struct_name = to_pascal_case(key);
    let mut field_lines = vec![];

    for (i, schema_part) in all_of.as_array()?.iter().enumerate() {
        let part_name = format!("{}Part{}", main_struct_name, i + 1);

        if with_docs {
            let doc_block = doc_lines_to_string_block(schema_part, 4);
            if !doc_block.is_empty() {
                field_lines.push(doc_block);
            }
        }

        extract_struct_recursive(
            &part_name,
            schema_part,
            output,
            visited,
            "#".to_string(),
            definitions,
            resolver,
            output_path.clone(),
            with_docs,
        );

        field_lines.push(format!(
            "    #[serde(flatten)]\n    pub part_{}: {},",
            i + 1,
            part_name
        ));
    }

    let mut struct_lines = vec![];
    if with_docs {
        if let Some(comment) = extract_top_description(all_of) {
            struct_lines.push(comment);
        }
    }
    struct_lines.push(format!(
        "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
        main_struct_name,
        field_lines.join("\n")
    ));

    let struct_code = struct_lines.join("\n");

    output.push(NamedStruct {
        name: main_struct_name.clone(),
        code: struct_code,
        output_path,
    });

    Some(main_struct_name)
}

pub fn to_snake_case(name: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            for c in ch.to_lowercase() {
                snake.push(c);
            }
        } else if ch == '.' || ch == '-' {
            snake.push('_');
        } else {
            snake.push(ch);
        }
    }
    snake
}

pub fn to_pascal_case(name: &str) -> String {
    name.split(|c: char| c == '_' || c == '.' || c == '-')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}
