# Architecture

Technical overview of Saffron's architecture and design decisions.

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Core Components](#core-components)
- [Design Principles](#design-principles)
- [Data Flow](#data-flow)
- [Technology Stack](#technology-stack)

## Overview

Saffron is built as a modular Rust workspace with clear separation of concerns. The architecture follows domain-driven design principles, separating business logic from infrastructure and presentation concerns.

```
┌─────────────┐
│   CLI/UI    │  ← User Interface Layer
└──────┬──────┘
       │
┌──────▼──────┐
│  Handlers   │  ← Application Layer
└──────┬──────┘
       │
┌──────▼──────┐
│    Core     │  ← Domain Layer
└──────┬──────┘
       │
┌──────▼──────┐
│ HTTP/Data   │  ← Infrastructure Layer
└─────────────┘
```

## Project Structure

### Workspace Layout

```
saffron/
├── saffron/              # Main binary crate
├── crates/
│   ├── saffron-core/     # Domain models
│   ├── saffron-http/     # HTTP client implementation
│   ├── saffron-data/     # Data parsing (JSON)
│   ├── saffron-cli/      # Command-line interface
│   ├── saffron-ui/       # GUI (future)
│   └── saffron-utils/    # Shared utilities
├── docs/                 # Documentation
└── target/               # Build artifacts
```

### Crate Dependencies

```
saffron (main binary)
├── saffron-cli
│   ├── saffron-core
│   ├── saffron-http
│   │   └── saffron-core
│   ├── saffron-data
│   ├── clap (CLI parsing)
│   ├── colored (output)
│   └── uuid, chrono
└── clap
```

## Core Components

### 1. saffron-core

**Purpose:** Domain models and business logic

**Key Types:**
- `HttpRequest` - Request representation
- `HttpResponse` - Response representation
- `Collection` - Request organization
- `Environment` - Variable management

**Characteristics:**
- Zero external HTTP dependencies
- Pure domain logic
- Easily testable
- Framework-agnostic

**Example:**
```rust
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: RequestBody,
    pub timeout_seconds: Option<u64>,
    pub follow_redirects: bool,
}
```

### 2. saffron-http

**Purpose:** HTTP client implementation

**Technology:** `ureq` (synchronous HTTP client)

**Features:**
- Request execution
- Response handling
- Multipart form-data
- File uploads/downloads
- Custom configuration
- Error handling

**Design Decision:** We chose `ureq` over `reqwest` for:
- Simpler API
- No async runtime overhead
- Smaller binary size
- Sufficient for CLI use case

### 3. saffron-data

**Purpose:** Data parsing and manipulation

**Key Feature:** Custom JSON parser

**Why Custom Parser:**
- Learning exercise
- No external JSON dependencies in core
- Full control over error messages
- Demonstrates Rust parsing capabilities

**Components:**
- `Tokenizer` - Lexical analysis
- `TokenStream` - Token management
- `Parse` trait - Parsing interface
- `Json` type - AST representation

### 4. saffron-cli

**Purpose:** Command-line interface

**Technology:** `clap` (derive API)

**Components:**

**CLI Module:**
- Command definitions
- Argument parsing
- Subcommand structure

**Handlers Module:**
- Command execution logic
- Business logic orchestration
- Error handling

**Storage Module:**
- File system persistence
- Collection management
- Environment storage
- History tracking

**Output Module:**
- Response formatting
- Colorized output
- JSON pretty-printing

**History Module:**
- Request tracking
- Entry management
- Timestamp handling

## Design Principles

### 1. Separation of Concerns

Each crate has a single, well-defined responsibility:
- Core: Domain logic only
- HTTP: Network operations
- CLI: User interaction
- Data: Parsing operations

### 2. Dependency Inversion

The core domain doesn't depend on infrastructure:
```
✓ CLI → Core → HTTP
✗ Core → HTTP
```

### 3. Testability

- Domain logic is pure and easily testable
- Infrastructure can be mocked
- Each layer has its own test suite

### 4. Modularity

Components can be:
- Tested independently
- Replaced without affecting others
- Reused in different contexts (CLI, GUI, API)

### 5. Error Handling

Comprehensive error types using `thiserror`:

```rust
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Connection timeout")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Network error: {0}")]
    NetworkError(String),
}
```

## Data Flow

### Request Execution Flow

```
1. User Input (CLI)
         ↓
2. Parse Arguments (clap)
         ↓
3. Load Environment (Storage)
         ↓
4. Resolve Variables (Core)
         ↓
5. Build Request (Core)
         ↓
6. Execute Request (HTTP)
         ↓
7. Parse Response (Data)
         ↓
8. Save to History (Storage)
         ↓
9. Format Output (CLI)
         ↓
10. Display to User
```

### Collection Workflow

```
Create Collection
      ↓
Add Requests → Serialize (serde) → Save (JSON)
      ↓
Load Later → Deserialize → Execute
```

### Environment Resolution

```
Load Environment
      ↓
Parse Template "{{variable}}"
      ↓
Lookup Value in HashMap
      ↓
Replace in Request
```

## Technology Stack

### Core Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `ureq` | HTTP client | 2.10 |
| `serde` | Serialization | 1.0 |
| `clap` | CLI parsing | 4.5 |
| `colored` | Terminal colors | 2.1 |
| `thiserror` | Error handling | 2.0 |
| `chrono` | Date/time | 0.4 |
| `uuid` | Unique IDs | 1.0 |
| `dirs` | Home directory | 5.0 |

### Development Tools

- **Cargo** - Build system and package manager
- **rustfmt** - Code formatting
- **clippy** - Linting
- **cargo-test** - Testing framework

## Storage Format

### Collections (JSON)

```json
{
  "name": "My API",
  "description": "API collection",
  "folders": [],
  "requests": [
    {
      "id": "uuid-here",
      "name": "Get Users",
      "description": "Fetch all users",
      "method": "GET",
      "url": "https://api.example.com/users",
      "headers": [["Accept", "application/json"]],
      "body": null,
      "timeout_seconds": 30
    }
  ]
}
```

### Environments (JSON)

```json
{
  "active": "production",
  "environments": [
    {
      "name": "production",
      "variables": {
        "base_url": "https://api.prod.com",
        "api_key": "prod_key_123"
      }
    }
  ]
}
```

### History (JSON)

```json
[
  {
    "id": "uuid-here",
    "timestamp": 1733678000,
    "duration_ms": 245,
    "request": {
      "method": "GET",
      "url": "https://api.github.com",
      "headers": [],
      "body": null
    },
    "response": {
      "status": 200,
      "status_text": "OK",
      "headers": [["content-type", "application/json"]],
      "body_preview": "..."
    }
  }
]
```

## Performance Characteristics

### Binary Size
- Release build: ~5-8 MB
- Stripped: ~3-5 MB

### Memory Usage
- Baseline: ~2-5 MB
- Per request: ~1-2 MB (depends on response size)
- History cache: ~100 KB (100 entries)

### Startup Time
- Cold start: ~10-50 ms
- Warm start: ~5-10 ms

### Request Performance
- Overhead: <1 ms
- Network time: Variable (depends on server)

## Future Architecture Plans

### GUI Layer (saffron-ui)
- Technology: Tauri or egui
- Architecture: Same core, different presentation
- State management: Share with CLI

### Plugin System
- Dynamic loading of extensions
- Custom request processors
- Custom output formatters

### Team Sync
- Git-based collection sharing
- Cloud synchronization option
- Conflict resolution

### Advanced Features
- GraphQL support (new crate: saffron-graphql)
- WebSocket support (new crate: saffron-ws)
- gRPC support (new crate: saffron-grpc)

## Contributing to Architecture

When adding features:

1. **Determine the layer:**
   - Domain logic → saffron-core
   - Network operations → saffron-http
   - CLI commands → saffron-cli
   - Data parsing → saffron-data

2. **Follow patterns:**
   - Use Result types for errors
   - Builder pattern for complex objects
   - Trait-based abstractions

3. **Maintain separation:**
   - Don't leak infrastructure into core
   - Keep CLI concerns in CLI crate
   - Use dependency injection

4. **Test thoroughly:**
   - Unit tests for domain logic
   - Integration tests for workflows
   - Example-based documentation

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Domain-Driven Design](https://www.domainlanguage.com/ddd/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
