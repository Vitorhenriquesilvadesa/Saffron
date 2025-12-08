# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- GraphQL support
- WebSocket support
- Import/export (Postman, Insomnia formats)
- SSL verification control
- Proxy support
- GUI implementation

## [0.1.0] - 2025-12-08

### Added
- Initial release of Saffron HTTP client
- Custom JSON parser with zero external JSON dependencies
- Core domain models (HttpRequest, HttpResponse, Collection, Environment)
- Synchronous HTTP client using ureq
- Complete CLI interface with clap
- Command groups:
  - `send` - Execute HTTP requests
  - `collection` - Manage request collections
  - `env` - Manage environment variables
  - `history` - View and replay request history
- Template variable resolution with `{{variable}}` syntax
- Automatic request history tracking (100-entry limit)
- Colored terminal output
- Persistent storage in `~/.saffron/`
- Support for all standard HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- Custom headers and request body support
- JSON request/response formatting
- Comprehensive test suite (133 tests)
- Complete documentation:
  - Getting Started guide
  - CLI Reference
  - Examples and use cases
  - Architecture documentation
  - Contributing guidelines
  - FAQ

### Technical Details
- Built with Rust 2024 edition
- Modular workspace architecture (7 crates)
- Zero-panic error handling
- Cross-platform support (Windows, macOS, Linux)
- Single binary deployment (~5-8 MB)
- Low memory footprint (~2-5 MB runtime)

[Unreleased]: https://github.com/Vitorhenriquesilvadesa/saffron/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Vitorhenriquesilvadesa/saffron/releases/tag/v0.1.0
