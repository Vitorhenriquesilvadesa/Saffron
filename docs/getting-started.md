# Getting Started with Saffron

This guide will help you get started with Saffron, from installation to making your first API requests.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Basic Usage](#basic-usage)
- [Next Steps](#next-steps)

## Installation

### Prerequisites

- Rust 1.70 or higher (for building from source)
- Git (for cloning the repository)

### Building from Source

1. **Clone the repository:**

```bash
git clone https://github.com/yourusername/saffron.git
cd saffron
```

2. **Build the project:**

```bash
cargo build --release
```

3. **The binary will be located at:**

```
target/release/saffron      # Linux/macOS
target/release/saffron.exe  # Windows
```

4. **Optional: Add to PATH**

**Linux/macOS:**
```bash
sudo cp target/release/saffron /usr/local/bin/
```

**Windows:**
Add the `target/release` directory to your system PATH.

### Using Cargo

```bash
cargo install saffron
```

## Configuration

Saffron stores its configuration and data in your home directory:

- **Linux/macOS:** `~/.saffron/`
- **Windows:** `C:\Users\<username>\.saffron\`

Directory structure:
```
~/.saffron/
├── collections/        # Saved request collections
├── environments/       # Environment configurations
└── history.json       # Request history
```

No additional configuration is required. Saffron will create these directories automatically on first run.

## Basic Usage

### Your First Request

Make a simple GET request:

```bash
saffron send https://api.github.com
```

This will:
1. Send a GET request to the GitHub API
2. Display the response with colorized JSON
3. Save the request to history

### Viewing Response Headers

Use the `-v` (verbose) flag to see response headers:

```bash
saffron send https://api.github.com -v
```

### POST Request with JSON

Send data to an API:

```bash
saffron send https://httpbin.org/post \
  -m POST \
  -j '{"name":"Alice","age":30}' \
  -H "Content-Type:application/json"
```

### Working with Environments

1. **Create an environment:**

```bash
saffron env set dev \
  api_url=https://api.dev.example.com \
  api_key=dev_key_123
```

2. **Use the environment:**

```bash
saffron send "{{api_url}}/users" \
  -e dev \
  -H "Authorization:Bearer {{api_key}}"
```

3. **List environments:**

```bash
saffron env list
```

### Managing Collections

1. **Create a collection:**

```bash
saffron collection new "GitHub API" \
  -d "Collection for GitHub API endpoints"
```

2. **Add a request:**

```bash
saffron collection add "GitHub API" "Get User" \
  https://api.github.com/users/octocat \
  -m GET \
  -H "Accept:application/vnd.github+json"
```

3. **View collection:**

```bash
saffron collection show "GitHub API"
```

### Using Request History

1. **List recent requests:**

```bash
saffron history list
```

2. **View request details:**

```bash
saffron history show 1
```

3. **Rerun a request:**

```bash
saffron history rerun 1
```

## Next Steps

Now that you've learned the basics, explore:

- [CLI Reference](cli-reference.md) - Complete command documentation
- [Examples](examples.md) - Real-world usage scenarios
- [Advanced Features](advanced-features.md) - File uploads, authentication, and more

## Getting Help

- Run `saffron --help` for command overview
- Run `saffron <command> --help` for command-specific help
- Check the [FAQ](faq.md) for common questions
- [Report issues](https://github.com/yourusername/saffron/issues) on GitHub
