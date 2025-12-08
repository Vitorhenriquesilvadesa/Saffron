use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub elapsed: Duration,
    pub url: String,
}

impl HttpResponse {
    pub fn new(
        status: u16,
        status_text: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
        elapsed: Duration,
        url: String,
    ) -> Self {
        Self {
            status,
            status_text,
            headers,
            body,
            elapsed,
            url,
        }
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_redirect(&self) -> bool {
        self.status >= 300 && self.status < 400
    }

    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }

    pub fn body_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    pub fn body_as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.body).ok()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
            .map(|(_, v)| v.as_str())
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    pub fn is_json(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }

    pub fn is_html(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("text/html"))
            .unwrap_or(false)
    }

    pub fn is_xml(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("xml"))
            .unwrap_or(false)
    }

    pub fn content_length(&self) -> Option<usize> {
        self.get_header("content-length")
            .and_then(|v| v.parse().ok())
    }
}
