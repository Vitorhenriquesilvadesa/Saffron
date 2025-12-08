# 🌸 Saffron HTTP

HTTP client implementation for the Saffron project. Provides a robust, feature-rich HTTP client built on top of `ureq` with advanced capabilities for API testing and automation.

## Features

✅ **Full HTTP Methods Support**
- GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS

✅ **Multiple Body Types**
- JSON
- Plain Text
- Form URL Encoded
- Multipart Form Data (with file uploads)
- Binary data

✅ **Advanced Configuration**
- Custom timeouts
- Redirect handling
- Custom user agents
- Max response size limits
- TLS certificate validation

✅ **Rich Error Handling**
- Specific error types for different scenarios
- Timeout detection
- TLS/Certificate errors
- Network errors

✅ **Helper Functions**
- File download
- File upload
- Request time measurement

## Usage

### Basic Request

```rust
use saffron_http::HttpClient;

let client = HttpClient::new();
let response = client.get("https://api.example.com/users")?;

println!("Status: {}", response.status);
println!("Body: {}", response.body_as_string()?);
```

### POST with JSON

```rust
use saffron_core::domain::request::HttpRequest;
use saffron_http::HttpClient;

let request = HttpRequest::post("https://api.example.com/users")
    .with_json_body(r#"{"name": "Alice", "email": "alice@example.com"}"#)
    .with_header("Authorization", "Bearer token123");

let client = HttpClient::new();
let response = client.send(&request)?;
```

### Custom Configuration

```rust
use saffron_http::{HttpClient, HttpClientConfig};

let config = HttpClientConfig {
    timeout_seconds: 60,
    follow_redirects: true,
    max_redirects: 10,
    user_agent: Some("MyApp/1.0".to_string()),
    accept_invalid_certs: false,
    max_response_size: Some(50 * 1024 * 1024), // 50MB
};

let client = HttpClient::with_config(config);
```

### Multipart Form Data (File Upload)

```rust
use saffron_core::domain::request::{FormDataPart, FormDataContent, HttpRequest, RequestBody};
use saffron_http::HttpClient;

let file_data = std::fs::read("photo.jpg")?;

let part = FormDataPart {
    name: "photo".to_string(),
    content: FormDataContent::File {
        filename: "photo.jpg".to_string(),
        data: file_data,
        content_type: Some("image/jpeg".to_string()),
    },
};

let request = HttpRequest::post("https://api.example.com/upload")
    .with_body(RequestBody::FormData(vec![part]));

let client = HttpClient::new();
let response = client.send(&request)?;
```

### Form URL Encoded

```rust
use std::collections::HashMap;
use saffron_core::domain::request::{HttpRequest, RequestBody};
use saffron_http::HttpClient;

let mut form = HashMap::new();
form.insert("username".to_string(), "alice".to_string());
form.insert("password".to_string(), "secret123".to_string());

let request = HttpRequest::post("https://api.example.com/login")
    .with_body(RequestBody::FormUrlEncoded(form));

let client = HttpClient::new();
let response = client.send(&request)?;
```

### Helper Functions

```rust
use saffron_http::helpers;
use std::path::Path;

// Download a file
helpers::download_file(
    "https://example.com/file.pdf",
    Path::new("downloaded.pdf")
)?;

// Upload a file
let response = helpers::upload_file(
    "https://api.example.com/upload",
    "file",
    Path::new("document.pdf")
)?;

// Measure request time
let request = HttpRequest::get("https://api.example.com/slow-endpoint");
let (response, total_time) = helpers::measure_request_time(&request)?;
println!("Request took: {:?}", total_time);
```

## Error Handling

```rust
use saffron_http::{HttpClient, HttpError};

let client = HttpClient::new();

match client.get("https://api.example.com/data") {
    Ok(response) => {
        if response.is_success() {
            println!("Success: {}", response.status);
        } else if response.is_client_error() {
            println!("Client error: {}", response.status);
        }
    }
    Err(HttpError::Timeout) => {
        println!("Request timed out");
    }
    Err(HttpError::NetworkError(msg)) => {
        println!("Network error: {}", msg);
    }
    Err(HttpError::TlsError(msg)) => {
        println!("TLS error: {}", msg);
    }
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

## Response Inspection

```rust
let response = client.get("https://api.example.com/users")?;

// Status checking
println!("Success: {}", response.is_success());
println!("Redirect: {}", response.is_redirect());
println!("Client Error: {}", response.is_client_error());
println!("Server Error: {}", response.is_server_error());

// Content type detection
if response.is_json() {
    // Parse JSON
} else if response.is_html() {
    // Handle HTML
}

// Headers
if let Some(content_type) = response.content_type() {
    println!("Content-Type: {}", content_type);
}

// Body
let text = response.body_as_string()?;
let bytes = &response.body;
```

## Examples

Run the advanced usage example:

```bash
cargo run --example advanced_usage --package saffron-http
```

## Architecture

The `saffron-http` crate is the implementation layer that:
- Depends on `saffron-core` for domain models
- Uses `ureq` for the actual HTTP networking
- Provides a clean abstraction over the underlying HTTP library
- Can be swapped out for different implementations (async, different libraries, etc.)

## License

Part of the Saffron project.
