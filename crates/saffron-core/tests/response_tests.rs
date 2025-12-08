use saffron_core::domain::response::HttpResponse;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_response_new() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        b"test body".to_vec(),
        Duration::from_millis(150),
        "https://example.com".to_string(),
    );

    assert_eq!(response.status, 200);
    assert_eq!(response.status_text, "OK");
    assert_eq!(response.body, b"test body");
    assert_eq!(response.elapsed, Duration::from_millis(150));
    assert_eq!(response.url, "https://example.com");
}

#[test]
fn test_response_is_success() {
    let response = create_test_response(200);
    assert!(response.is_success());

    let response = create_test_response(299);
    assert!(response.is_success());

    let response = create_test_response(199);
    assert!(!response.is_success());

    let response = create_test_response(300);
    assert!(!response.is_success());
}

#[test]
fn test_response_is_redirect() {
    let response = create_test_response(301);
    assert!(response.is_redirect());

    let response = create_test_response(302);
    assert!(response.is_redirect());

    let response = create_test_response(399);
    assert!(response.is_redirect());

    let response = create_test_response(299);
    assert!(!response.is_redirect());

    let response = create_test_response(400);
    assert!(!response.is_redirect());
}

#[test]
fn test_response_is_client_error() {
    let response = create_test_response(400);
    assert!(response.is_client_error());

    let response = create_test_response(404);
    assert!(response.is_client_error());

    let response = create_test_response(499);
    assert!(response.is_client_error());

    let response = create_test_response(399);
    assert!(!response.is_client_error());

    let response = create_test_response(500);
    assert!(!response.is_client_error());
}

#[test]
fn test_response_is_server_error() {
    let response = create_test_response(500);
    assert!(response.is_server_error());

    let response = create_test_response(503);
    assert!(response.is_server_error());

    let response = create_test_response(599);
    assert!(response.is_server_error());

    let response = create_test_response(499);
    assert!(!response.is_server_error());

    let response = create_test_response(600);
    assert!(!response.is_server_error());
}

#[test]
fn test_response_body_as_string() {
    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        HashMap::new(),
        "Hello, World!".as_bytes().to_vec(),
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.body_as_string().unwrap(), "Hello, World!");
}

#[test]
fn test_response_body_as_string_invalid_utf8() {
    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        HashMap::new(),
        vec![0xFF, 0xFE, 0xFD],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert!(response.body_as_string().is_err());
}

#[test]
fn test_response_body_as_str() {
    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        HashMap::new(),
        "Test content".as_bytes().to_vec(),
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.body_as_str(), Some("Test content"));
}

#[test]
fn test_response_content_type() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.content_type(), Some("application/json"));
}

#[test]
fn test_response_content_type_case_insensitive() {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "text/html".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.content_type(), Some("text/html"));
}

#[test]
fn test_response_get_header() {
    let mut headers = HashMap::new();
    headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
    headers.insert("Authorization".to_string(), "Bearer token".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.get_header("X-Custom-Header"), Some("custom-value"));
    assert_eq!(response.get_header("Authorization"), Some("Bearer token"));
    assert_eq!(response.get_header("Missing"), None);
}

#[test]
fn test_response_get_header_case_insensitive() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(
        response.get_header("content-type"),
        Some("application/json")
    );
    assert_eq!(
        response.get_header("CONTENT-TYPE"),
        Some("application/json")
    );
}

#[test]
fn test_response_is_json() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert!(response.is_json());
}

#[test]
fn test_response_is_json_with_charset() {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "application/json; charset=utf-8".to_string(),
    );

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert!(response.is_json());
}

#[test]
fn test_response_is_html() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert!(response.is_html());
}

#[test]
fn test_response_is_xml() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/xml".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert!(response.is_xml());
}

#[test]
fn test_response_content_length() {
    let mut headers = HashMap::new();
    headers.insert("Content-Length".to_string(), "1234".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.content_length(), Some(1234));
}

#[test]
fn test_response_content_length_invalid() {
    let mut headers = HashMap::new();
    headers.insert("Content-Length".to_string(), "invalid".to_string());

    let response = HttpResponse::new(
        200,
        "OK".to_string(),
        headers,
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    );

    assert_eq!(response.content_length(), None);
}

fn create_test_response(status: u16) -> HttpResponse {
    HttpResponse::new(
        status,
        "Test".to_string(),
        HashMap::new(),
        vec![],
        Duration::from_millis(100),
        "https://example.com".to_string(),
    )
}
