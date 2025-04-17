# 🐱 Kumpun Framework

**Kumpun** is a lightweight, schema-first API framework powered by **Rust** and **JSON Schema**. Built from scratch with love and inspired by a cat named Kumpun, it's designed for developers who want performance, validation, and flexibility without losing fun.

---

## ✨ Features

- ✅ **Schema-First**: Define your input/output using JSON Schema
- ⚡ **Rust Core**: High-performance validation and dispatch
- 🔄 **Auto Docs**: Generate documentation from schema (WIP)
- 🛠️ **CLI Tools**: Fast mock, validate, and simulate APIs
- 🌐 **Cross-language Ready**: Schema can be consumed by any language

---

## 📦 Project Structure (Planned)

```
kumpun-framework/
├── @kumpun/                      # Kumpun Monorepo Crates
│   ├── core/                     # 🧠 Core routing, validation, type-safe handler mapping
│   ├── cli/                      # 🛠️ CLI: kumpun dev, check, generate, docs
│   ├── schema/                   # 📚 Rust crate: JSON Schema loader, validator, type-gen utils
│   └── Cargo.toml                # 🗂️ Workspace manifest
│
├── schemas/                      # 📄 Pure JSON Schema files (used by all languages/tools)
│   ├── user/
│   │   ├── create.json
│   │   └── login.json
│   ├── post/
│   │   └── update.json
│   └── index.json                # (optional) registry or schema manifest
│
├── examples/                     # 📦 Example apps using Kumpun CLI + core
│   ├── minimal-api/
│   └── ts-client-integration/
│
├── .gitignore
├── LICENSE
└── README.md
```

---

## 🚀 Usage (Coming Soon)

```bash
$ kumpun dev

😺  Welcome to Kumpun Framework!
📦  Schema loaded: ./schemas/user.get.json
🚀  Mock server running at http://localhost:3000
```

---

## 📖 Philosophy

- **Validate at the edge**: Keep your APIs clean, predictable, and fast
- **Make schema a first-class citizen**
- **Have fun while building tools**

> Kumpun is not just another API tool. It's your cat-powered API engine 😸

---

## 📚 License

MIT — feel free to use, modify, or even contribute!

---

## 🐾 Inspired by

> "คำปัน" — a cat who loves to nap on keyboards and chase JSONs.

---

## 🤝 Contribute

You’re welcome to submit ideas, issues, PRs, and cat photos 😹

Follow updates and roadmap here soon!

---

**Made with ☕ + 🐈 by ChilG**