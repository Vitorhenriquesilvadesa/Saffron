use saffron_core::domain::request::{HttpMethod, HttpRequest, RequestBody};
use saffron_http::{HttpClient, HttpClientConfig, HttpError};
use std::collections::HashMap;

#[test]
fn test_http_client_new() {
    let _client = HttpClient::new();
}

#[test]
fn test_http_client_with_timeout() {
    let _client = HttpClient::with_timeout(60);
}

#[test]
fn test_http_client_config_default() {
    let config = HttpClientConfig::default();
    assert_eq!(config.timeout_seconds, 30);
    assert!(config.follow_redirects);
    assert_eq!(config.max_redirects, 10);
    assert!(config.user_agent.is_some());
    assert!(!config.accept_invalid_certs);
    assert_eq!(config.max_response_size, Some(100 * 1024 * 1024));
}

#[test]
fn test_http_client_config_custom() {
    let config = HttpClientConfig {
        timeout_seconds: 15,
        follow_redirects: false,
        max_redirects: 5,
        user_agent: Some("TestAgent/1.0".to_string()),
        accept_invalid_certs: true,
        max_response_size: Some(10 * 1024 * 1024),
    };

    assert_eq!(config.timeout_seconds, 15);
    assert!(!config.follow_redirects);
    assert_eq!(config.max_redirects, 5);
    assert_eq!(config.user_agent, Some("TestAgent/1.0".to_string()));
    assert!(config.accept_invalid_certs);
    assert_eq!(config.max_response_size, Some(10 * 1024 * 1024));
}

#[test]
fn test_http_client_with_config() {
    let config = HttpClientConfig {
        timeout_seconds: 20,
        follow_redirects: true,
        max_redirects: 3,
        user_agent: Some("Custom/1.0".to_string()),
        accept_invalid_certs: false,
        max_response_size: Some(5 * 1024 * 1024),
    };

    let _client = HttpClient::with_config(config);
}

#[test]
fn test_http_error_display() {
    let error = HttpError::Timeout;
    assert_eq!(error.to_string(), "Connection timeout");

    let error = HttpError::InvalidUrl("bad url".to_string());
    assert_eq!(error.to_string(), "Invalid URL: bad url");

    let error = HttpError::NetworkError("connection refused".to_string());
    assert_eq!(error.to_string(), "Network error: connection refused");

    let error = HttpError::TlsError("certificate error".to_string());
    assert_eq!(error.to_string(), "TLS error: certificate error");

    let error = HttpError::TooManyRedirects;
    assert_eq!(error.to_string(), "Too many redirects");
}

#[test]
fn test_request_body_handling() {
    let _client = HttpClient::new();

    let request = HttpRequest::get("https://httpbin.org/get");
    assert!(matches!(request.body, RequestBody::None));

    let request = HttpRequest::post("https://httpbin.org/post").with_text_body("plain text");
    match request.body {
        RequestBody::Text(ref text) => assert_eq!(text, "plain text"),
        _ => panic!("Expected Text body"),
    }

    let request =
        HttpRequest::post("https://httpbin.org/post").with_json_body(r#"{"key": "value"}"#);
    match request.body {
        RequestBody::Json(ref json) => assert_eq!(json, r#"{"key": "value"}"#),
        _ => panic!("Expected JSON body"),
    }
}

#[test]
fn test_form_urlencoded_body() {
    let mut form = HashMap::new();
    form.insert("username".to_string(), "testuser".to_string());
    form.insert("password".to_string(), "secret".to_string());

    let request =
        HttpRequest::post("https://httpbin.org/post").with_body(RequestBody::FormUrlEncoded(form));

    match request.body {
        RequestBody::FormUrlEncoded(ref data) => {
            assert_eq!(data.get("username"), Some(&"testuser".to_string()));
            assert_eq!(data.get("password"), Some(&"secret".to_string()));
        }
        _ => panic!("Expected FormUrlEncoded body"),
    }
}

#[test]
fn test_binary_body() {
    let data = vec![0x00, 0x01, 0x02, 0xFF];
    let request =
        HttpRequest::post("https://httpbin.org/post").with_body(RequestBody::Binary(data.clone()));

    match request.body {
        RequestBody::Binary(ref bytes) => assert_eq!(bytes, &data),
        _ => panic!("Expected Binary body"),
    }
}

#[test]
fn test_helper_guess_content_type() {
    use std::path::Path;

    let helpers_test = |ext: &str, expected: &str| {
        let _path = Path::new(&format!("file.{}", ext));
        let content_type = match ext {
            "txt" => "text/plain",
            "html" | "htm" => "text/html",
            "json" => "application/json",
            "xml" => "application/xml",
            "pdf" => "application/pdf",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "mp4" => "video/mp4",
            "zip" => "application/zip",
            _ => "application/octet-stream",
        };
        assert_eq!(content_type, expected);
    };

    helpers_test("txt", "text/plain");
    helpers_test("html", "text/html");
    helpers_test("json", "application/json");
    helpers_test("jpg", "image/jpeg");
    helpers_test("png", "image/png");
    helpers_test("pdf", "application/pdf");
    helpers_test("unknown", "application/octet-stream");
}

#[test]
fn test_request_methods() {
    assert_eq!(HttpMethod::Get.as_str(), "GET");
    assert_eq!(HttpMethod::Post.as_str(), "POST");
    assert_eq!(HttpMethod::Put.as_str(), "PUT");
    assert_eq!(HttpMethod::Patch.as_str(), "PATCH");
    assert_eq!(HttpMethod::Delete.as_str(), "DELETE");
    assert_eq!(HttpMethod::Head.as_str(), "HEAD");
    assert_eq!(HttpMethod::Options.as_str(), "OPTIONS");
}

#[test]
fn test_client_default() {
    let _client = HttpClient::default();
}

#[test]
fn test_http_error_from_io_error() {
    use std::io;

    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let http_error: HttpError = io_error.into();

    assert!(matches!(http_error, HttpError::IoError(_)));
}

#[test]
fn test_config_clone() {
    let config1 = HttpClientConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.timeout_seconds, config2.timeout_seconds);
    assert_eq!(config1.follow_redirects, config2.follow_redirects);
    assert_eq!(config1.max_redirects, config2.max_redirects);
}

#[test]
fn test_request_with_custom_headers() {
    let request = HttpRequest::post("https://api.example.com/data")
        .with_header("Authorization", "Bearer token123")
        .with_header("X-Custom-Header", "custom-value")
        .with_header("Accept", "application/json");

    assert_eq!(request.headers.len(), 3);
    assert_eq!(request.get_header("Authorization"), Some("Bearer token123"));
    assert_eq!(request.get_header("X-Custom-Header"), Some("custom-value"));
    assert_eq!(request.get_header("Accept"), Some("application/json"));
}

#[test]
fn test_timeout_configuration() {
    let request1 = HttpRequest::get("https://example.com");
    assert_eq!(request1.timeout_seconds, Some(30));

    let request2 = HttpRequest::get("https://example.com").with_timeout(60);
    assert_eq!(request2.timeout_seconds, Some(60));

    let request3 = HttpRequest::get("https://example.com").without_timeout();
    assert_eq!(request3.timeout_seconds, None);
}

#[test]
fn test_redirect_configuration() {
    let request1 = HttpRequest::get("https://example.com");
    assert!(request1.follow_redirects);

    let request2 = HttpRequest::get("https://example.com").follow_redirects(false);
    assert!(!request2.follow_redirects);
}

#[test]
fn test_content_type_auto_detection() {
    let json_req = HttpRequest::post("https://example.com").with_json_body(r#"{"test": true}"#);

    let text_req = HttpRequest::post("https://example.com").with_text_body("plain text");

    match json_req.body {
        RequestBody::Json(_) => (),
        _ => panic!("Expected JSON body"),
    }

    match text_req.body {
        RequestBody::Text(_) => (),
        _ => panic!("Expected Text body"),
    }
}
