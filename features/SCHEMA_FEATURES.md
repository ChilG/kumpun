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
- ❌ `const` / `default` → not included in output

## 🔹 Metadata Mapping
- ✅ `description` → generates `///` doc comments for fields, enums, oneOf/anyOf/allOf
- ✅ `examples` → rendered as inline `/// Example: ...` (with `description`)
- ❌ `title` → not used (fallback only for missing description)

## 🔧 Code Output
- ✅ auto import: `HashMap`
- ✅ auto import: `use serde::{Deserialize, Serialize}`
- ✅ generated filenames in `snake_case`
- ✅ struct field names in `snake_case`
- ✅ auto-generate `mod.rs` with `pub mod` declarations
- ✅ root `use` paths prefixed with `crate::generated::...`
- ❌ auto import: `chrono`, `uuid`, etc.

## 🧪 Next Steps
- [x] Implement `RefResolver` for cross-file `$ref`
- [x] Generate `mod.rs` recursively
- [x] Prefix `crate::generated::...` for imports
- [x] Convert filenames and fields to `snake_case`
- [x] Insert `use serde::{Deserialize, Serialize}` when required
- [x] Annotate doc/comments from `description`
- [x] Add `examples` to doc comment output
- [x] Support `definitions` reuse even if used only once
- [x] Support `patternProperties` grouping + field naming
- [ ] Generate test stubs or `impl` blocks (future idea)
- [ ] Support `const`, `default`, and enum fallback values

---

### ✅ Schema-to-Rust Generator Progress
- [x] OneOf as enum
- [x] AnyOf as untagged enum
- [x] AllOf as flatten struct
- [x] Nested struct recursion
- [x] additionalProperties as HashMap
- [x] patternProperties with intelligent grouping + field naming
- [x] `$ref` cross-file
- [x] definitions reuse (even single-use)