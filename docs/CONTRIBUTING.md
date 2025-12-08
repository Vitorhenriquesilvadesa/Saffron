# Contributing to Saffron

Thank you for your interest in contributing to Saffron! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Submitting Changes](#submitting-changes)
- [Feature Requests](#feature-requests)
- [Bug Reports](#bug-reports)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

### Prerequisites

- Rust 2024 edition or later
- Git
- A code editor (VS Code, IntelliJ IDEA, etc.)

### First-Time Contributors

If you're new to the project:

1. **Read the documentation:**
   - [README.md](../README.md) - Project overview
   - [Architecture](./architecture.md) - Technical design
   - [Getting Started](./getting-started.md) - User guide

2. **Explore the codebase:**
   - Browse the `crates/` directory
   - Read existing tests to understand functionality
   - Try building and running the project

3. **Find a task:**
   - Look for issues labeled `good first issue`
   - Check the roadmap for planned features
   - Ask maintainers for guidance

## Development Setup

### 1. Fork and Clone

```powershell
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/saffron.git
cd saffron
```

### 2. Build the Project

```powershell
# Build all crates
cargo build

# Build in release mode
cargo build --release
```

### 3. Run Tests

```powershell
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p saffron-core

# Run tests with output
cargo test -- --nocapture
```

### 4. Run the CLI

```powershell
# Run from source
cargo run -- --help

# Run a specific command
cargo run -- send https://api.github.com
```

### 5. Install Development Tools

```powershell
# Format checker
rustup component add rustfmt

# Linter
rustup component add clippy

# Documentation generator
cargo install cargo-doc
```

## Project Structure

```
saffron/
├── saffron/              # Main binary
├── crates/
│   ├── saffron-core/     # Domain models (add business logic here)
│   ├── saffron-http/     # HTTP client (add network features here)
│   ├── saffron-data/     # JSON parser (add parsing logic here)
│   ├── saffron-cli/      # CLI interface (add commands here)
│   ├── saffron-ui/       # GUI (future)
│   └── saffron-utils/    # Shared utilities
├── docs/                 # Documentation
└── tests/                # Integration tests (future)
```

### Where to Add Code

- **New HTTP feature:** `crates/saffron-http/src/`
- **New CLI command:** `crates/saffron-cli/src/cli.rs` + `handlers.rs`
- **New domain model:** `crates/saffron-core/src/domain/`
- **New parsing logic:** `crates/saffron-data/src/`
- **Utilities:** `crates/saffron-utils/src/`

## Development Workflow

### 1. Create a Branch

```powershell
git checkout -b feature/my-feature
# or
git checkout -b fix/bug-description
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code improvements
- `test/` - Test additions

### 2. Make Changes

- Write clear, focused commits
- Follow the coding standards (see below)
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```powershell
# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Build in release mode
cargo build --release
```

### 4. Commit Your Changes

```powershell
git add .
git commit -m "Add feature: short description"
```

Commit message format:
```
Type: Short summary (50 chars or less)

Detailed explanation if needed. Explain what changed
and why, not how (the code shows how).

Fixes #123
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### 5. Push and Create Pull Request

```powershell
git push origin feature/my-feature
```

Then create a Pull Request on GitHub.

## Coding Standards

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// ✓ Good
pub fn send_request(request: &HttpRequest) -> Result<HttpResponse, HttpError> {
    // Clear, descriptive names
    // Returns Result for error handling
}

// ✗ Bad
pub fn send(r: &Req) -> Res {
    // Unclear names
    // No error handling
}
```

### Formatting

Use `rustfmt` for consistent formatting:

```powershell
cargo fmt
```

Configuration is in `rustfmt.toml` (if present).

### Linting

Use `clippy` to catch common mistakes:

```powershell
cargo clippy -- -D warnings
```

Fix all warnings before submitting.

### Documentation

- Add doc comments to public APIs:

```rust
/// Sends an HTTP request and returns the response.
///
/// # Arguments
///
/// * `request` - The HTTP request to send
///
/// # Returns
///
/// Returns `Ok(HttpResponse)` on success, or `Err(HttpError)` on failure.
///
/// # Examples
///
/// ```
/// let request = HttpRequest::builder()
///     .method(HttpMethod::GET)
///     .url("https://api.github.com")
///     .build()?;
///
/// let response = client.send(&request)?;
/// ```
pub fn send(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
    // Implementation
}
```

### Error Handling

- Use `Result` types, not panics
- Use `thiserror` for error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Parse error at line {line}: {msg}")]
    Parse { line: usize, msg: String },
}
```

### Naming Conventions

- **Types:** PascalCase - `HttpRequest`, `JsonValue`
- **Functions:** snake_case - `send_request`, `parse_json`
- **Constants:** SCREAMING_SNAKE_CASE - `MAX_TIMEOUT`, `DEFAULT_PORT`
- **Modules:** snake_case - `http_client`, `json_parser`

## Testing Guidelines

### Unit Tests

Place tests in the same file as the code:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

### Integration Tests

Create files in `tests/` directory:

```rust
// tests/http_integration_tests.rs
use saffron_http::HttpClient;

#[test]
fn test_github_api() {
    let client = HttpClient::new();
    // Test real HTTP interactions
}
```

### Test Coverage

Aim for:
- **Core logic:** >90% coverage
- **Infrastructure:** >70% coverage
- **CLI handlers:** >50% coverage

### Test Naming

```rust
#[test]
fn test_<what>_<when>_<expected>() {
    // Example:
    // test_parse_json_with_invalid_syntax_returns_error
}
```

### Test Organization

Group related tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod http_request_tests {
        use super::*;

        #[test]
        fn test_build_get_request() { }

        #[test]
        fn test_build_post_request() { }
    }

    mod http_response_tests {
        use super::*;

        #[test]
        fn test_parse_response() { }
    }
}
```

## Submitting Changes

### Pull Request Process

1. **Update documentation:**
   - Update README if needed
   - Add doc comments
   - Update CHANGELOG (if exists)

2. **Ensure quality:**
   - All tests pass
   - No clippy warnings
   - Code is formatted

3. **Create Pull Request:**
   - Clear title and description
   - Reference related issues
   - Explain the changes

4. **Address feedback:**
   - Respond to reviewer comments
   - Make requested changes
   - Re-request review

### Pull Request Template

```markdown
## Description
Brief description of the changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] No new warnings

## Related Issues
Fixes #123
```

### Review Process

- Maintainers will review within 1-3 days
- Address feedback promptly
- Be open to suggestions
- Once approved, changes will be merged

## Feature Requests

To request a new feature:

1. **Check existing issues** - Avoid duplicates
2. **Create an issue** with:
   - Clear description
   - Use case explanation
   - Example usage
   - Potential implementation approach

Example:
```markdown
### Feature Request: GraphQL Support

**Description:**
Add GraphQL query support to Saffron.

**Use Case:**
Many modern APIs use GraphQL instead of REST.

**Example:**
```powershell
saffron graphql "{ user(id: 1) { name } }" \
  --url https://api.example.com/graphql
```

**Suggested Implementation:**
- Add `saffron-graphql` crate
- Parse GraphQL queries
- Add CLI command
```

## Bug Reports

To report a bug:

1. **Check existing issues** - Avoid duplicates
2. **Create an issue** with:
   - Clear title
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Environment details
   - Error messages

Example:
```markdown
### Bug Report: JSON Parse Error on Valid Input

**Environment:**
- OS: Windows 11
- Rust version: 1.82
- Saffron version: 0.1.0

**Steps to Reproduce:**
1. Run: `saffron send https://api.github.com`
2. Observe error

**Expected Behavior:**
Should parse JSON response successfully.

**Actual Behavior:**
Error: "Invalid JSON at line 1"

**Error Output:**
```
Error: Invalid JSON at line 1
```

**Additional Context:**
The same URL works in Postman.
```

## Development Tips

### Debugging

Use `dbg!` macro for quick debugging:

```rust
dbg!(&request);
```

Or use VS Code debugger with CodeLLDB extension.

### Performance Profiling

```powershell
cargo build --release
cargo run --release -- send https://api.github.com
```

### Documentation Generation

```powershell
cargo doc --open
```

### Continuous Integration

- Tests run automatically on PR
- Must pass before merging
- Check GitHub Actions for results

## Getting Help

- **Questions:** Open a GitHub Discussion
- **Bugs:** Open a GitHub Issue
- **Chat:** Join our Discord (future)
- **Email:** Contact maintainers

## Recognition

Contributors are listed in:
- GitHub Contributors page
- CONTRIBUTORS.md file (future)
- Release notes

Thank you for contributing to Saffron! 🎉
