# 📦 schema_to_rust.rs - Struct Generator from JSON Schema

✅ = Supported  🔜 = Partial / Planned  ❌ = Not yet

---

## 🔹 Core Struct Features
- ✅ `type: object` → generate struct
- ✅ `properties + required` → map to pub fields
- ✅ `optional fields` → `Option<T>`
- ✅ primitive types → `string`, `number`, `boolean`, `integer`

## 🔹 Composition & Recursion
- ✅ nested object → recursive struct
- ✅ array of primitives → `Vec<T>`
- ✅ array of object → `Vec<Struct>`
- ✅ `$ref` (in same file) → resolve + reuse

## 🔹 Enum & Union
- ✅ `enum` (string values) → Rust `enum` variants
- ✅ `oneOf` (object variants) → Rust `enum` with struct payloads
- ✅ `anyOf` → untagged Rust `enum` (e.g. `Variant<T1>`, `Variant<T2>`)
- ✅ `allOf` → merged struct with `#[serde(flatten)]`

## 🔹 Schema Reuse
- ✅ `$ref` (external file) → RefResolver supports cross-file
- ✅ definitions reuse → even if not used across multiple fields

## 🔹 Advanced Schema
- ✅ `additionalProperties` → `Option<HashMap<String, T>>`
- ✅ `patternProperties` → grouped by type and merged into named `HashMap<String, T>` fields using `#[serde(flatten)]`
- ✅ `const`, `default` → generates `#[serde(default = "...")]` and helper functions
- 🔜 `type: [T1, T2]` (multi-type) → not fully resolved yet
- ❌ enum fallback (`#[serde(other)]`) → not supported yet

## 🔹 Metadata Mapping
- ✅ `description` → generates `///` doc comments
- ✅ `examples` → rendered as `/// Example: ...`
- ❌ `title` → not used directly

## 🔧 Code Output
- ✅ auto import: `HashMap`
- ✅ auto import: `use serde::{Deserialize, Serialize}`
- ✅ generated filenames in `snake_case`
- ✅ struct field names in `snake_case`
- ✅ auto-generate `mod.rs` with `pub mod` declarations
- ✅ root `use` paths prefixed with `crate::generated::...`
- ✅ escape reserved words (e.g. `type`, `enum`, `const`, `$ref`, `if`, `else`, etc.) using `#[serde(rename = "...")]`
- ❌ auto import: `chrono`, `uuid`, etc.

## 🧪 Next Steps
- [x] Add `$`-prefixed keyword escaping and `serde(rename = "...")`
- [x] Test with official schemas from json-schema.org
- [x] Generate helper functions for default/const
- [ ] Handle `type: [T1, T2]` array typing safely
- [ ] Improve support for Draft 2019-09 & 2020-12 new keywords (e.g. `examples`, `unevaluatedProperties`, `dependentSchemas`, etc.)
- [ ] Add enum fallback variant (`#[serde(other)]`)
- [ ] Optional: generate impl blocks or test stubs for validation and schema examples

---

### ✅ Schema-to-Rust Generator Progress
- [x] OneOf as enum
- [x] AnyOf as untagged enum
- [x] AllOf as flatten struct
- [x] Nested struct recursion
- [x] additionalProperties as HashMap
- [x] patternProperties with type grouping and naming
- [x] `$ref` cross-file support
- [x] `$` keyword handling with renaming
- [x] definitions reuse (even single-use)
- [x] `default` and `const` mapped to helper functions
- [x] Full compile pass for draft 04/06/07/2019-09/2020-12