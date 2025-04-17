use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug)]
struct NamedStruct {
    name: String,
    code: String,
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

    structs
}

fn extract_struct_recursive(
    name: &str,
    schema: &Value,
    output: &mut Vec<NamedStruct>,
    visited: &mut HashSet<String>,
    path: String,
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
        let field_name: &str = key;
        let is_required = required.contains(field_name);
        let rust_type = match infer_rust_type(prop, field_name, output, visited, definitions) {
            Some(t) => t,
            None => "serde_json::Value".to_string(),
        };

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
    // $ref handling
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

    match prop.get("type")?.as_str()? {
        "string" => {
            if let Some(enum_vals) = prop.get("enum") {
                // Generate enum
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
            let sub_name = to_pascal_case(key);
            extract_struct_recursive(
                &sub_name,
                prop,
                output,
                visited,
                "#".to_string(),
                definitions,
            );
            Some(sub_name)
        }
        _ => Some("serde_json::Value".to_string()),
    }
}

fn to_pascal_case(name: &str) -> String {
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
