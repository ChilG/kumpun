// =======================================================
// 📦 schema_to_rust.rs - Struct Generator from JSON Schema
//
// ✅ = Supported     🔜 = Partial / Planned     ❌ = Not yet
// =======================================================

// 🔹 Core Struct Features
// ✅ type: object               → generate struct
// ✅ properties + required      → map to pub fields
// ✅ optional fields            → Option<T>
// ✅ primitive types            → string, number, boolean, integer

// 🔹 Composition & Recursion
// ✅ nested object              → recursive struct
// ✅ array of primitives        → Vec<T>
// ✅ array of object            → Vec<Struct>
// ✅ $ref (in same file)        → resolve + reuse

// 🔹 Enum & Union
// ✅ enum (string values)       → Rust enum variants
// ✅ oneOf (object variants)    → Rust enum with struct payloads
// ✅ anyOf                      → untagged Rust enum (Variant<T1>, Variant<T2>, …)
// ✅ allOf                      → merged struct with #[serde(flatten)]

// 🔹 Schema Reuse
// 🔜 $ref (external file)       → pending RefResolver (cross-file)
// ❌ definitions reuse          → not reused across multiple fields

// 🔹 Advanced Schema
// ✅ additionalProperties       → Option<HashMap<String, T>>
// ❌ patternProperties          → not yet supported
// ❌ const / default            → not included in output
// 🔜 format, minLength, etc.    → can be added with #[validate] later

// 🔧 Code Output
// ✅ auto import: HashMap
// ❌ auto import: chrono, uuid, etc.

// 🧪 Next Steps
// - [ ] Implement `RefResolver` for cross-file $ref
// - [ ] Support patternProperties → HashMap + regex
// - [ ] Annotate doc/comments from `description`
// - [ ] Generate test stubs or `impl` blocks (future idea)

//! Schema-to-Rust Generator Progress
//! - [x] OneOf as enum
//! - [x] AnyOf as untagged enum
//! - [x] AllOf as flatten struct
//! - [x] Nested struct recursion
//! - [x] additionalProperties as HashMap
//! - [ ] $ref cross-file

use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
pub struct NamedStruct {
    name: String,
    pub(crate) code: String,
}

pub fn generate_rust_structs_from_schema(root_name: &str, schema: &Value) -> Vec<NamedStruct> {
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
    );

    let mut use_lines = vec![];
    for s in &structs {
        if s.code.contains("HashMap<") {
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
            },
        );
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
) {
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
        let rust_type = infer_rust_type(prop, field_name, output, visited, definitions)
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
    });
}

fn infer_rust_type(
    prop: &Value,
    key: &str,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
) -> Option<String> {
    if let Some(ref_val) = prop.get("$ref").and_then(|v| v.as_str()) {
        let name = ref_val.split('/').last()?.to_string();
        let def = definitions.get(&name)?;
        extract_struct_recursive(
            &name,
            def,
            output,
            visited,
            ref_val.to_string(),
            definitions,
        );
        return Some(name);
    }

    if let Some(one_of) = prop.get("oneOf") {
        return handle_one_of(key, one_of, output, visited, definitions);
    }
    if let Some(any_of) = prop.get("anyOf") {
        return handle_any_of(key, any_of, output, visited, definitions);
    }
    if let Some(all_of) = prop.get("allOf") {
        return handle_all_of(key, all_of, output, visited, definitions);
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
            let inner =
                infer_rust_type(items, &format!("{}Item", key), output, visited, definitions)?;
            Some(format!("Vec<{}>", inner))
        }
        "object" => {
            if let Some(ap) = prop.get("additionalProperties") {
                let inner_type =
                    infer_rust_type(ap, &format!("{}Value", key), output, visited, definitions)
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
            );
            variants.push(format!("    {}({}),", title, struct_name));
        } else {
            let inner_type = infer_rust_type(variant, &title, output, visited, definitions)
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
    });
    Some(enum_name)
}

fn handle_any_of(
    key: &str,
    any_of: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
) -> Option<String> {
    let enum_name = to_pascal_case(key);
    let mut variants = vec![];

    for (i, variant) in any_of.as_array()?.iter().enumerate() {
        let var_name = format!("Variant{}", i + 1);
        let inner_type = infer_rust_type(variant, &var_name, output, visited, definitions)
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
    });
    Some(enum_name)
}

fn handle_all_of(
    key: &str,
    all_of: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    definitions: &Value,
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
