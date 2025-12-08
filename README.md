# Saffron

**Saffron** is a lightweight, cross‑platform HTTP client written in Rust.
It aims to provide a minimal, efficient and developer‑friendly interface to send HTTP requests, inspect responses, and manage collections of requests — without the overhead of heavier GUI-based clients.

---

## 🚀 What Saffron is

- A native desktop HTTP client with a Rust core
- Supports standard HTTP methods (GET, POST, PUT, PATCH, DELETE, etc.)
- Full control over request configuration: URL, method, headers, body (JSON, raw, form-data, multipart, etc.)
- Ability to save, load and organize requests into collections
- A clean and simple GUI (planned to be implemented with a Rust-friendly UI solution such as Tauri or a native Rust GUI framework)
- Focus on performance, low memory footprint, and speed

---

## ✅ Why Saffron

Many existing HTTP clients feel heavy or rely on bulky runtimes. With Saffron, the goal is to offer:

- A **lightweight alternative** for developers who want a fast, no‑nonsense tool
- A tool that stays **close to the metal**, using Rust for core logic — ensuring efficiency and reliability
- A clean, intuitive interface for quickly testing and debugging HTTP APIs
- An open‑source solution meant to be minimal, straightforward and practical

---

## 📦 Planned Features

### MVP (first target for a working version)
- Request builder with support for URL, method, headers and body
- Execute HTTP requests and display response (raw, text, or formatted JSON)
- Save and load requests locally
- Basic GUI for constructing and executing requests

### Potential Future Additions
- Environment and variable support (e.g. placeholder variables like `{{api_key}}`)
- Organization of requests into collections or folders
- Request history or session management
- Import / export of collections (e.g. via JSON or other common formats)
- Code‑snippet generation for popular languages (cURL, Python, JavaScript, etc.)
- Hook/plugin system (e.g. pre‑request/post‑response scripts)
- Theme support (e.g. dark mode / light mode)

---

## ⚠️ Project Status

Saffron is currently in early development. Not all planned features are implemented yet, and internal APIs or UI design may change over time. The project is meant to evolve — feedback and testing will guide future improvements.
