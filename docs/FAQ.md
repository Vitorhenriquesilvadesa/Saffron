# Frequently Asked Questions (FAQ)

Common questions and answers about Saffron.

## Table of Contents

- [General](#general)
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Troubleshooting](#troubleshooting)
- [Development](#development)

## General

### What is Saffron?

Saffron is a command-line HTTP client for testing and debugging APIs. It's similar to Postman or Insomnia but runs entirely in the terminal. It's written in Rust for speed and reliability.

### Why use Saffron instead of curl?

Saffron offers several advantages over curl:

- **Persistent storage:** Save collections and environments
- **Request history:** Automatically tracks all requests
- **Template variables:** Use `{{variables}}` in requests
- **Organization:** Group requests into collections
- **Better output:** Colored, formatted responses
- **Ease of use:** Simpler syntax for complex requests

### Why use Saffron instead of Postman?

Saffron advantages:

- **CLI-first:** No GUI overhead, perfect for terminal workflows
- **Speed:** Fast startup and execution
- **Scriptable:** Easy to integrate into automation
- **Version control:** Collections are JSON files
- **Lightweight:** Single binary, minimal dependencies
- **Privacy:** Everything runs locally

### Is Saffron ready for production use?

Saffron is in active development. The CLI is stable for daily use, but some advanced features are still planned. Always test in non-production environments first.

### What's the license?

Saffron is open-source under the MIT License. You can use, modify, and distribute it freely.

## Installation

### How do I install Saffron?

**From crates.io (recommended):**
```powershell
cargo install saffron-http-client
```

**Verify installation:**
```powershell
saffron --version
# Output: saffron 0.1.5
```

**From source:**
```powershell
git clone https://github.com/Vitorhenriquesilvadesa/saffron.git
cd saffron
cargo build --release
```

See [Getting Started](getting-started.md#installation) for details.

### What are the system requirements?

- **OS:** Windows, macOS, or Linux
- **Rust:** 2024 edition or later (for building)
- **Disk space:** ~5-8 MB for binary
- **Memory:** ~2-5 MB at runtime

### Do I need to install Rust?

Only if you're building from source. Once built, the binary is standalone and doesn't require Rust.

### How do I update Saffron?

**From crates.io:**
```powershell
cargo install saffron-http-client --force
```

**From source:**
```powershell
cd saffron
git pull
cargo build --release
```

## Usage

### How do I make a simple GET request?

```powershell
saffron send https://api.github.com
```

### How do I send JSON data?

```powershell
saffron send https://api.example.com/users `
  --method POST `
  --header "Content-Type: application/json" `
  --body '{"name": "Alice"}'
```

### How do I save a request to a collection?

First, create a collection:
```powershell
saffron collection new my-api
```

Then add the request:
```powershell
saffron collection add my-api get-users `
  --method GET `
  --url https://api.example.com/users
```

### How do I use environment variables?

Set variables:
```powershell
saffron env set development base_url https://api.dev.com
saffron env set development api_key dev_key_123
```

Use in requests:
```powershell
saffron send {{base_url}}/users `
  --header "Authorization: Bearer {{api_key}}"
```

### How do I view request history?

```powershell
# List all requests
saffron history list

# Show details
saffron history show <id>

# Rerun a request
saffron history rerun <id>
```

### Can I pipe responses to other commands?

Yes! Saffron outputs to stdout:

```powershell
# Save response to file
saffron send https://api.github.com > response.json

# Pipe to jq
saffron send https://api.github.com | jq '.name'

# Combine with grep
saffron send https://api.github.com | grep -i "github"
```

## Features

### Does Saffron support authentication?

Yes, you can add authentication headers:

**Bearer token:**
```powershell
saffron send https://api.example.com `
  --header "Authorization: Bearer YOUR_TOKEN"
```

**Basic auth:**
```powershell
saffron send https://api.example.com `
  --header "Authorization: Basic $(echo -n 'user:pass' | base64)"
```

**API key:**
```powershell
saffron send https://api.example.com `
  --header "X-API-Key: YOUR_KEY"
```

### Can I upload files?

Yes, using the `--body` option:

```powershell
# Upload a file
saffron send https://api.example.com/upload `
  --method POST `
  --header "Content-Type: application/octet-stream" `
  --body "@path/to/file.pdf"
```

For multipart uploads, this is a planned feature.

### Does Saffron support HTTPS?

Yes, HTTPS is supported automatically. Saffron uses the system's TLS implementation.

### Can I disable SSL verification?

Not currently, but this is a planned feature for development environments.

### Does Saffron follow redirects?

Yes, by default Saffron follows up to 5 redirects. This can be customized in the request configuration.

### What HTTP methods are supported?

All standard methods:
- GET
- POST
- PUT
- DELETE
- PATCH
- HEAD
- OPTIONS

### Does Saffron support GraphQL?

Not yet, but GraphQL support is planned. For now, you can make GraphQL requests as POST requests:

```powershell
saffron send https://api.example.com/graphql `
  --method POST `
  --header "Content-Type: application/json" `
  --body '{"query": "{ user(id: 1) { name } }"}'
```

### Does Saffron support WebSockets?

Not yet, but WebSocket support is planned.

### Can I use Saffron in CI/CD?

Yes! Saffron is perfect for CI/CD:

```yaml
# GitHub Actions example
- name: Test API
  run: |
    saffron send https://api.staging.com/health
    if [ $? -ne 0 ]; then exit 1; fi
```

### Can I import collections from other tools?

**Yes! Currently supported:**
- ✅ **Insomnia v4** - Full support for workspaces and requests

```powershell
# Export from Insomnia (Application Menu → Import/Export → Export Data)
# Then import:
saffron collection import insomnia-export.json
```

**Coming soon:**
- 🔜 Postman Collection v2.1
- 🔜 Thunder Client
- 🔜 OpenAPI/Swagger specs

See [Examples - Importing Collections](examples.md#importing-collections) for details.

### Can I export my Saffron collections?

Yes! You can export collections to share with your team:

```powershell
saffron collection export "My API" my-api.json
```

The exported file uses Saffron's native format and can be imported by other Saffron users.

## Troubleshooting

### "Command not found" error

Make sure the binary is in your PATH:

**Windows:**
```powershell
$env:PATH += ";C:\path\to\saffron"
```

**macOS/Linux:**
```bash
export PATH="$PATH:/path/to/saffron"
```

### "Connection timeout" error

This usually means:
1. The server is down
2. The URL is incorrect
3. Firewall is blocking the connection

Try:
- Check the URL
- Ping the server
- Check firewall settings
- Use a longer timeout (if supported)

### "Invalid JSON" error

Common causes:
1. Response is not JSON
2. Malformed JSON from server

Try:
- Check the response with `--verbose` (future)
- Verify the API returns JSON
- Check for trailing commas or quotes

### "Permission denied" when accessing collections

This usually means:
- The `~/.saffron/` directory has wrong permissions

Fix on macOS/Linux:
```bash
chmod -R 755 ~/.saffron/
```

### Requests are slow

Saffron itself has minimal overhead (<1ms). Slow requests are usually due to:
1. Network latency
2. Slow server
3. Large response sizes

Try:
- Test with a faster server
- Check your network connection
- Use a local server for testing

### "Request failed" with no details

Enable verbose output (future feature):
```powershell
saffron send https://api.example.com --verbose
```

For now, check:
- Server logs
- Network connectivity
- Request syntax

### How do I reset Saffron?

Delete the configuration directory:

**Windows:**
```powershell
Remove-Item -Recurse -Force $env:USERPROFILE\.saffron
```

**macOS/Linux:**
```bash
rm -rf ~/.saffron/
```

This will delete all collections, environments, and history.

### Variables are not being replaced

Check:
1. Environment is active: `saffron env show`
2. Variable is set: `saffron env list`
3. Syntax is correct: `{{variable_name}}`
4. No typos in variable name

## Development

### How do I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

Quick start:
1. Fork the repository
2. Create a feature branch
3. Make changes and add tests
4. Submit a pull request

### How do I report bugs?

Open a GitHub issue with:
- Description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details

See [Bug Reports](CONTRIBUTING.md#bug-reports).

### How do I request features?

Open a GitHub issue with:
- Feature description
- Use case
- Example usage
- Suggested implementation (optional)

See [Feature Requests](CONTRIBUTING.md#feature-requests).

### How is the project structured?

Saffron is a Rust workspace with multiple crates:
- `saffron-core`: Domain models
- `saffron-http`: HTTP client
- `saffron-data`: JSON parser
- `saffron-cli`: Command-line interface
- `saffron-ui`: GUI (future)
- `saffron-utils`: Utilities

See [Architecture](architecture.md) for details.

### Can I use Saffron as a library?

Yes! You can use individual crates:

```toml
[dependencies]
saffron-core = "0.1"
saffron-http = "0.1"
```

```rust
use saffron_core::{HttpRequest, HttpMethod};
use saffron_http::HttpClient;

let request = HttpRequest::builder()
    .method(HttpMethod::GET)
    .url("https://api.github.com")
    .build()?;

let client = HttpClient::new();
let response = client.send(&request)?;
```

### Where is the GUI?

The GUI (`saffron-ui`) is planned but not yet implemented. The CLI is the primary interface for now.

### What's on the roadmap?

Planned features:
- GraphQL support
- WebSocket support
- Request chaining
- Response assertions
- Import/export (Postman, Insomnia)
- SSL verification control
- Proxy support
- GUI

See the [GitHub milestones](https://github.com/yourusername/saffron/milestones) for details.

## Still Have Questions?

- **GitHub Issues:** [Report bugs or request features](https://github.com/yourusername/saffron/issues)
- **GitHub Discussions:** [Ask questions](https://github.com/yourusername/saffron/discussions)
- **Documentation:** [Read the docs](getting-started.md)
- **Email:** Contact maintainers (future)

We're happy to help! 🎉
