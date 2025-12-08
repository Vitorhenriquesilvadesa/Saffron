use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestBody {
    None,
    Text(String),
    Json(String),
    FormUrlEncoded(HashMap<String, String>),
    FormData(Vec<FormDataPart>),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormDataPart {
    pub name: String,
    pub content: FormDataContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormDataContent {
    Text(String),
    File {
        filename: String,
        data: Vec<u8>,
        content_type: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: RequestBody,
    pub timeout_seconds: Option<u64>,
    pub follow_redirects: bool,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: Vec::new(),
            body: RequestBody::None,
            timeout_seconds: Some(30),
            follow_redirects: true,
        }
    }

    pub fn get(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Get, url)
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Post, url)
    }

    pub fn put(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Put, url)
    }

    pub fn patch(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Patch, url)
    }

    pub fn delete(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Delete, url)
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(HttpHeader::new(name, value));
        self
    }

    pub fn with_headers(mut self, headers: Vec<HttpHeader>) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn with_body(mut self, body: RequestBody) -> Self {
        self.body = body;
        self
    }

    pub fn with_json_body(mut self, json: impl Into<String>) -> Self {
        self.body = RequestBody::Json(json.into());
        self
    }

    pub fn with_text_body(mut self, text: impl Into<String>) -> Self {
        self.body = RequestBody::Text(text.into());
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    pub fn without_timeout(mut self) -> Self {
        self.timeout_seconds = None;
        self
    }

    pub fn follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    pub fn add_header(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.headers.push(HttpHeader::new(name, value));
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value.as_str())
    }

    pub fn content_type(&self) -> Option<&str> {
        self.get_header("Content-Type")
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self::get("")
    }
}
