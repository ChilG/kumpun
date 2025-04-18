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
    pub output_path: Option<String>, // <- file path hint (e.g., "shared/address")
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

    for s in structs {
        println!("🧾 writing {} → {:?}", s.name, s.output_path);
        match &s.output_path {
            Some(path_hint) => {
                let full_path = Path::new(out_dir).join(format!("{}.rs", path_hint));
                let parent = full_path.parent().unwrap();
                fs::create_dir_all(parent).expect("Failed to create output directory");

                let mut file = fs::File::create(&full_path).expect("Failed to create output file");
                file.write_all(s.code.as_bytes()).expect("Write failed");

                println!("✅ Generated: {}", full_path.display());
            }
            None => {
                root_code.push(s.code.as_str());
            }
        }
    }

    let full_path = Path::new(out_dir).join(format!("{}.rs", root_name));
    let parent = full_path.parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create output directory");

    let mut file = fs::File::create(&full_path).expect("Failed to write root output");
    let joined = root_code.join("\n\n");
    file.write_all(joined.as_bytes())
        .expect("Root write failed");

    println!("✅ Stub generated: {}", full_path.display());
}

pub fn generate_rust_structs_from_schema(
    root_name: &str,
    schema: &Value,
    resolver: &mut RefResolver,
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
    );

    let mut use_lines = vec![];
    for s in &structs {
        let is_root_file = s.output_path.is_none() || s.output_path.as_deref() == Some(root_name);
        if (s.output_path.is_none() || is_root_file) && s.code.contains("HashMap<") {
            use_lines.push("use std::collections::HashMap;");
        }
    }

    use_lines.sort();
    use_lines.dedup();

    for (i, line) in use_lines.into_iter().rev().enumerate() {
        structs.insert(
            0,
            NamedStruct {
                name: format!("__use_{}", i),
                code: line.to_string(),
                output_path: None, // ✅ เขียนรวมกับ root เท่านั้น
            },
        );
    }

    // ด้านล่างสุดก่อน write_named_structs
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
        let field_name = key.as_str();
        let is_required = required.contains(field_name);
        println!("📦 field: {}", key); // ก่อนเรียก infer_rust_type
        let rust_type = infer_rust_type(
            prop,
            field_name,
            output,
            visited,
            definitions,
            resolver,
            output_path.clone(),
        )
        .unwrap_or_else(|| "serde_json::Value".to_string());

        let final_type = if is_required {
            rust_type
        } else {
            format!("Option<{}>", rust_type)
        };

        fields.push(format!("    pub {}: {},", field_name, final_type));
    }

    let struct_code = format!(
        "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
        name,
        fields.join("\n")
    );

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
                );
                return Some(sub_name);
            }
            Some("serde_json::Value".to_string())
        }
        _ => Some("serde_json::Value".to_string()),
    }
}

fn handle_one_of(
    key: &str,
    one_of: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
    resolver: &mut RefResolver,
    output_path: Option<String>,
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
            );
            variants.push(format!("    {}({}),", title, struct_name));
        } else {
            let inner_type = infer_rust_type(
                variant,
                &title,
                output,
                visited,
                definitions,
                resolver,
                output_path.clone(),
            )
            .unwrap_or_else(|| "serde_json::Value".to_string());
            variants.push(format!("    {}({}),", title, inner_type));
        }
    }

    let enum_code = format!(
        "#[derive(Debug, Serialize, Deserialize)]\n#[serde(tag = \"type\")]\npub enum {} {{\n{}\n}}",
        enum_name,
        variants.join("\n")
    );

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
        )
        .unwrap_or_else(|| "serde_json::Value".to_string());
        variants.push(format!("    {}({}),", var_name, inner_type));
    }

    let code = format!(
        "#[derive(Debug, Serialize, Deserialize)]\n#[serde(untagged)]\npub enum {} {{\n{}\n}}",
        enum_name,
        variants.join("\n")
    );

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
) -> Option<String> {
    let main_struct_name = to_pascal_case(key);
    let mut field_lines = vec![];

    for (i, schema_part) in all_of.as_array()?.iter().enumerate() {
        let part_name = format!("{}Part{}", main_struct_name, i + 1);
        extract_struct_recursive(
            &part_name,
            schema_part,
            output,
            visited,
            "#".to_string(),
            definitions,
            resolver,
            output_path.clone(),
        );
        field_lines.push(format!(
            "    #[serde(flatten)]\n    pub part_{}: {},",
            i + 1,
            part_name
        ));
    }

    let struct_code = format!(
        "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n{}\n}}",
        main_struct_name,
        field_lines.join("\n")
    );

    output.push(NamedStruct {
        name: main_struct_name.clone(),
        code: struct_code,
        output_path,
    });

    Some(main_struct_name)
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
