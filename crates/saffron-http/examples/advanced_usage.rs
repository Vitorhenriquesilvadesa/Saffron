use saffron_core::domain::request::{FormDataContent, FormDataPart, HttpRequest, RequestBody};
use saffron_http::{HttpClient, HttpClientConfig};
use std::collections::HashMap;

fn main() {
    println!("=== Saffron HTTP - Advanced Features Demo ===\n");

    demo_custom_config();
    demo_multipart_form();
    demo_form_urlencoded();
    demo_custom_headers();
    demo_helpers();
}

fn demo_custom_config() {
    println!("--- 1. Custom Client Configuration ---");

    let config = HttpClientConfig {
        timeout_seconds: 15,
        follow_redirects: true,
        max_redirects: 5,
        user_agent: Some("Saffron-Custom/1.0".to_string()),
        accept_invalid_certs: false,
        max_response_size: Some(10 * 1024 * 1024),
    };

    let client = HttpClient::with_config(config);

    match client.get("https://httpbin.org/user-agent") {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            if let Ok(body) = response.body_as_string() {
                println!("  Response: {}", body.chars().take(100).collect::<String>());
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn demo_multipart_form() {
    println!("--- 2. Multipart Form Data ---");

    let text_part = FormDataPart {
        name: "description".to_string(),
        content: FormDataContent::Text("Test file upload".to_string()),
    };

    let file_part = FormDataPart {
        name: "file".to_string(),
        content: FormDataContent::File {
            filename: "test.txt".to_string(),
            data: b"Hello from Saffron!".to_vec(),
            content_type: Some("text/plain".to_string()),
        },
    };

    let request = HttpRequest::post("https://httpbin.org/post")
        .with_body(RequestBody::FormData(vec![text_part, file_part]));

    let client = HttpClient::new();
    match client.send(&request) {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("  Content-Type: {:?}", response.content_type());
            println!("  Body size: {} bytes", response.body.len());
            println!("  Response: {:#?}", response.body_as_string().unwrap());
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn demo_form_urlencoded() {
    println!("--- 3. Form URL Encoded ---");

    let mut form_data = HashMap::new();
    form_data.insert("username".to_string(), "saffron_user".to_string());
    form_data.insert("email".to_string(), "user@saffron.dev".to_string());
    form_data.insert("age".to_string(), "25".to_string());

    let request = HttpRequest::post("https://httpbin.org/post")
        .with_body(RequestBody::FormUrlEncoded(form_data));

    let client = HttpClient::new();
    match client.send(&request) {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            if let Ok(body) = response.body_as_string() {
                println!("  Response length: {} bytes", body.len());
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn demo_custom_headers() {
    println!("--- 4. Custom Headers & Authentication ---");

    let request = HttpRequest::get("https://httpbin.org/headers")
        .with_header("Authorization", "Bearer fake-token-123")
        .with_header("X-Custom-Header", "Saffron-Value")
        .with_header("Accept", "application/json")
        .with_header("Accept-Language", "en-US,en;q=0.9");

    let client = HttpClient::new();
    match client.send(&request) {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("  Request headers sent: {}", request.headers.len());
            println!("  Response headers received: {}", response.headers.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}

fn demo_helpers() {
    println!("--- 5. Helper Functions ---");

    println!("→ Measuring request time");
    let request = HttpRequest::get("https://httpbin.org/delay/1");

    match saffron_http::helpers::measure_request_time(&request) {
        Ok((response, duration)) => {
            println!("✓ Status: {}", response.status);
            println!("  Total time: {:?}", duration);
            println!("  Server processing time: {:?}", response.elapsed);
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();
}
