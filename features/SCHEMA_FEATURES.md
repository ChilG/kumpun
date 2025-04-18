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
- ❌ definitions reuse → not reused across multiple fields

## 🔹 Advanced Schema
- ✅ `additionalProperties` → `Option<HashMap<String, T>>`
- ❌ `patternProperties` → not yet supported
- ❌ `const` / `default` → not included in output
- 🔜 `format`, `minLength`, etc. → can be added with `#[validate]` later

## 🔧 Code Output
- ✅ auto import: `HashMap`
- ❌ auto import: `chrono`, `uuid`, etc.

## 🧪 Next Steps
- [x] Implement `RefResolver` for cross-file `$ref`
- [ ] Support `patternProperties` → `HashMap` + regex
- [ ] Annotate doc/comments from `description`
- [ ] Generate test stubs or `impl` blocks (future idea)

---

### ✅ Schema-to-Rust Generator Progress
- [x] OneOf as enum
- [x] AnyOf as untagged enum
- [x] AllOf as flatten struct
- [x] Nested struct recursion
- [x] additionalProperties as HashMap
- [x] `$ref` cross-file

