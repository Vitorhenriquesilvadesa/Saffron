# Saffron 🦀

**Saffron** is a fast, lightweight command-line HTTP client written in Rust. Test APIs, debug endpoints, and manage request collections — all from your terminal.

[![Rust](https://img.shields.io/badge/rust-2024%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

---

## ✨ Features

- **🚀 Fast & Lightweight:** Single binary, minimal dependencies, <5MB
- **📦 Collections:** Organize requests into reusable collections
- **🔄 Import from External Tools:** Import collections from Insomnia (Postman coming soon)
- **🌍 Environments:** Manage variables across dev/staging/production
- **📜 Request History:** Automatic tracking with 100-entry rolling cache
- **🔧 Template Variables:** Use `{{variables}}` in URLs, headers, and body
- **🎨 Colored Output:** Beautiful, readable terminal responses
- **💾 Persistent Storage:** All data stored locally in `~/.saffron/`
- **🔒 Privacy-First:** No cloud sync, everything runs locally
- **⚡ Zero Configuration:** Works out of the box

---

## 🚀 Quick Start

### Installation

**From source:**
```powershell
git clone https://github.com/yourusername/saffron.git
cd saffron
cargo build --release
```

The binary will be in `target/release/saffron.exe`.

**From crates.io (coming soon):**
```powershell
cargo install saffron
```

### Your First Request

```powershell
# Simple GET request
saffron send https://api.github.com

# POST with JSON
saffron send https://api.example.com/users `
  --method POST `
  --header "Content-Type: application/json" `
  --body '{"name": "Alice", "email": "alice@example.com"}'

# Using environment variables
saffron env set production api_url https://api.prod.com
saffron send {{api_url}}/users
```

---

## 📚 Documentation

- **[Getting Started](docs/getting-started.md)** - Installation, configuration, basic usage
- **[CLI Reference](docs/cli-reference.md)** - Complete command reference
- **[Examples](docs/examples.md)** - Real-world usage patterns
- **[Architecture](docs/architecture.md)** - Technical design and structure
- **[Contributing](docs/CONTRIBUTING.md)** - How to contribute
- **[FAQ](docs/FAQ.md)** - Frequently asked questions

---

## 💡 Core Concepts

### Collections

Organize related requests:

```powershell
# Create collection
saffron collection new github-api

# Add request
saffron collection add github-api get-user `
  --method GET `
  --url https://api.github.com/users/octocat

# Run request from collection
saffron send --from-collection "github-api/get-user"

# Import from Insomnia
saffron collection import insomnia-export.json

# List all collections
saffron collection list
```

### Environments

Manage variables across environments:

```powershell
# Set variables
saffron env set development base_url https://api.dev.com
saffron env set production base_url https://api.prod.com

# Switch environments
saffron env use production

# Use in requests
saffron send {{base_url}}/users
```

### Request History

Automatic tracking of all requests:

```powershell
# List history
saffron history list

# Show details
saffron history show abc123

# Rerun a request
saffron history rerun abc123

# Clear history
saffron history clear
```

---

## 🎯 Why Saffron?

| Feature | Saffron | curl | Postman |
|---------|---------|------|---------|
| CLI-first | ✅ | ✅ | ❌ |
| Collections | ✅ | ❌ | ✅ |
| Environments | ✅ | ❌ | ✅ |
| Request History | ✅ | ❌ | ✅ |
| Template Variables | ✅ | ❌ | ✅ |
| Lightweight | ✅ | ✅ | ❌ |
| Scriptable | ✅ | ✅ | ⚠️ |
| Privacy | ✅ | ✅ | ⚠️ |
| Open Source | ✅ | ✅ | ❌ |

---

## 🏗️ Architecture

Saffron is built as a modular Rust workspace:

```
saffron/
├── saffron-core     # Domain models (requests, responses, collections)
├── saffron-http     # HTTP client implementation (ureq-based)
├── saffron-data     # Custom JSON parser (no external deps)
├── saffron-cli      # Command-line interface (clap-based)
├── saffron-ui       # GUI (planned)
└── saffron-utils    # Shared utilities
```

See [Architecture](docs/architecture.md) for details.

---

## 🧪 Testing

Saffron has comprehensive test coverage:

```powershell
# Run all tests
cargo test

# Run specific crate tests
cargo test -p saffron-core
cargo test -p saffron-http
cargo test -p saffron-data
```

**Current test counts:**
- `saffron-core`: 82 tests
- `saffron-data`: 33 tests
- `saffron-http`: 18 tests
- **Total: 133 tests**

---

## 🗺️ Roadmap

### ✅ Completed (v0.1.0)
- [x] Core HTTP client
- [x] CLI interface
- [x] Collections management
- [x] Environment variables
- [x] Request history
- [x] Template variable resolution
- [x] Colored terminal output
- [x] Comprehensive documentation

### 🚧 In Progress
- [ ] GUI implementation (saffron-ui)
- [ ] Import/export (Postman, Insomnia)
- [ ] SSL verification control
- [ ] Proxy support

### 📋 Planned
- [ ] GraphQL support
- [ ] WebSocket support
- [ ] Request chaining
- [ ] Response assertions
- [ ] Pre/post-request scripts
- [ ] Code generation (curl, Python, etc.)
- [ ] Team sync (Git-based)

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

Quick checklist:
- Fork the repository
- Create a feature branch
- Write tests
- Follow coding standards
- Submit a pull request

---

## 📝 License

Saffron is licensed under the [MIT License](LICENSE).

---

## 🙏 Acknowledgments

Built with these excellent Rust crates:
- [ureq](https://github.com/algesten/ureq) - HTTP client
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [serde](https://github.com/serde-rs/serde) - Serialization
- [colored](https://github.com/colored-rs/colored) - Terminal colors
- [thiserror](https://github.com/dtolnay/thiserror) - Error handling

---

## 📧 Contact

- **Issues:** [GitHub Issues](https://github.com/yourusername/saffron/issues)
- **Discussions:** [GitHub Discussions](https://github.com/yourusername/saffron/discussions)

---

Made with ❤️ and Rust
