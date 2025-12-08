use saffron_core::domain::request::{
    FormDataContent, FormDataPart, HttpHeader, HttpMethod, HttpRequest, RequestBody,
};
use std::collections::HashMap;

#[test]
fn test_http_method_as_str() {
    assert_eq!(HttpMethod::Get.as_str(), "GET");
    assert_eq!(HttpMethod::Post.as_str(), "POST");
    assert_eq!(HttpMethod::Put.as_str(), "PUT");
    assert_eq!(HttpMethod::Patch.as_str(), "PATCH");
    assert_eq!(HttpMethod::Delete.as_str(), "DELETE");
    assert_eq!(HttpMethod::Head.as_str(), "HEAD");
    assert_eq!(HttpMethod::Options.as_str(), "OPTIONS");
}

#[test]
fn test_http_method_display() {
    assert_eq!(format!("{}", HttpMethod::Get), "GET");
    assert_eq!(format!("{}", HttpMethod::Post), "POST");
}

#[test]
fn test_http_header_new() {
    let header = HttpHeader::new("Content-Type", "application/json");
    assert_eq!(header.name, "Content-Type");
    assert_eq!(header.value, "application/json");
}

#[test]
fn test_request_new() {
    let request = HttpRequest::new(HttpMethod::Get, "https://example.com");
    assert_eq!(request.method, HttpMethod::Get);
    assert_eq!(request.url, "https://example.com");
    assert_eq!(request.headers.len(), 0);
    assert_eq!(request.body, RequestBody::None);
    assert_eq!(request.timeout_seconds, Some(30));
    assert!(request.follow_redirects);
}

#[test]
fn test_request_builder_get() {
    let request = HttpRequest::get("https://api.example.com/users");
    assert_eq!(request.method, HttpMethod::Get);
    assert_eq!(request.url, "https://api.example.com/users");
}

#[test]
fn test_request_builder_post() {
    let request = HttpRequest::post("https://api.example.com/users");
    assert_eq!(request.method, HttpMethod::Post);
}

#[test]
fn test_request_with_header() {
    let request = HttpRequest::get("https://example.com")
        .with_header("Authorization", "Bearer token123")
        .with_header("Accept", "application/json");

    assert_eq!(request.headers.len(), 2);
    assert_eq!(request.headers[0].name, "Authorization");
    assert_eq!(request.headers[0].value, "Bearer token123");
    assert_eq!(request.headers[1].name, "Accept");
    assert_eq!(request.headers[1].value, "application/json");
}

#[test]
fn test_request_with_json_body() {
    let request = HttpRequest::post("https://example.com").with_json_body(r#"{"key": "value"}"#);

    match request.body {
        RequestBody::Json(json) => assert_eq!(json, r#"{"key": "value"}"#),
        _ => panic!("Expected JSON body"),
    }
}

#[test]
fn test_request_with_text_body() {
    let request = HttpRequest::post("https://example.com").with_text_body("plain text content");

    match request.body {
        RequestBody::Text(text) => assert_eq!(text, "plain text content"),
        _ => panic!("Expected Text body"),
    }
}

#[test]
fn test_request_with_timeout() {
    let request = HttpRequest::get("https://example.com").with_timeout(60);
    assert_eq!(request.timeout_seconds, Some(60));
}

#[test]
fn test_request_without_timeout() {
    let request = HttpRequest::get("https://example.com").without_timeout();
    assert_eq!(request.timeout_seconds, None);
}

#[test]
fn test_request_follow_redirects() {
    let request = HttpRequest::get("https://example.com").follow_redirects(false);
    assert!(!request.follow_redirects);
}

#[test]
fn test_request_add_header() {
    let mut request = HttpRequest::get("https://example.com");
    request.add_header("X-Custom", "value");

    assert_eq!(request.headers.len(), 1);
    assert_eq!(request.headers[0].name, "X-Custom");
    assert_eq!(request.headers[0].value, "value");
}

#[test]
fn test_request_get_header() {
    let request = HttpRequest::get("https://example.com")
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", "Bearer token");

    assert_eq!(request.get_header("Content-Type"), Some("application/json"));
    assert_eq!(request.get_header("Authorization"), Some("Bearer token"));
    assert_eq!(request.get_header("Missing"), None);
}

#[test]
fn test_request_get_header_case_insensitive() {
    let request =
        HttpRequest::get("https://example.com").with_header("Content-Type", "application/json");

    assert_eq!(request.get_header("content-type"), Some("application/json"));
    assert_eq!(request.get_header("CONTENT-TYPE"), Some("application/json"));
}

#[test]
fn test_request_content_type() {
    let request = HttpRequest::get("https://example.com").with_header("Content-Type", "text/html");

    assert_eq!(request.content_type(), Some("text/html"));
}

#[test]
fn test_request_body_none() {
    let body = RequestBody::None;
    assert_eq!(body, RequestBody::None);
}

#[test]
fn test_request_body_text() {
    let body = RequestBody::Text("test".to_string());
    match body {
        RequestBody::Text(text) => assert_eq!(text, "test"),
        _ => panic!("Expected Text body"),
    }
}

#[test]
fn test_request_body_json() {
    let body = RequestBody::Json(r#"{"key": "value"}"#.to_string());
    match body {
        RequestBody::Json(json) => assert_eq!(json, r#"{"key": "value"}"#),
        _ => panic!("Expected JSON body"),
    }
}

#[test]
fn test_request_body_form_urlencoded() {
    let mut data = HashMap::new();
    data.insert("key1".to_string(), "value1".to_string());
    data.insert("key2".to_string(), "value2".to_string());

    let body = RequestBody::FormUrlEncoded(data.clone());
    match body {
        RequestBody::FormUrlEncoded(form_data) => {
            assert_eq!(form_data.get("key1"), Some(&"value1".to_string()));
            assert_eq!(form_data.get("key2"), Some(&"value2".to_string()));
        }
        _ => panic!("Expected FormUrlEncoded body"),
    }
}

#[test]
fn test_request_body_binary() {
    let data = vec![0x00, 0x01, 0x02, 0x03];
    let body = RequestBody::Binary(data.clone());
    match body {
        RequestBody::Binary(bytes) => assert_eq!(bytes, data),
        _ => panic!("Expected Binary body"),
    }
}

#[test]
fn test_form_data_part_text() {
    let part = FormDataPart {
        name: "field1".to_string(),
        content: FormDataContent::Text("value1".to_string()),
    };

    assert_eq!(part.name, "field1");
    match part.content {
        FormDataContent::Text(text) => assert_eq!(text, "value1"),
        _ => panic!("Expected Text content"),
    }
}

#[test]
fn test_form_data_part_file() {
    let part = FormDataPart {
        name: "upload".to_string(),
        content: FormDataContent::File {
            filename: "test.txt".to_string(),
            data: vec![1, 2, 3],
            content_type: Some("text/plain".to_string()),
        },
    };

    assert_eq!(part.name, "upload");
    match part.content {
        FormDataContent::File {
            filename,
            data,
            content_type,
        } => {
            assert_eq!(filename, "test.txt");
            assert_eq!(data, vec![1, 2, 3]);
            assert_eq!(content_type, Some("text/plain".to_string()));
        }
        _ => panic!("Expected File content"),
    }
}

#[test]
fn test_request_builder_chaining() {
    let request = HttpRequest::post("https://api.example.com/users")
        .with_header("Authorization", "Bearer token")
        .with_header("Accept", "application/json")
        .with_json_body(r#"{"name": "Alice"}"#)
        .with_timeout(45)
        .follow_redirects(false);

    assert_eq!(request.method, HttpMethod::Post);
    assert_eq!(request.url, "https://api.example.com/users");
    assert_eq!(request.headers.len(), 2);
    assert_eq!(request.timeout_seconds, Some(45));
    assert!(!request.follow_redirects);

    match request.body {
        RequestBody::Json(_) => (),
        _ => panic!("Expected JSON body"),
    }
}

#[test]
fn test_request_default() {
    let request = HttpRequest::default();
    assert_eq!(request.method, HttpMethod::Get);
    assert_eq!(request.url, "");
}
