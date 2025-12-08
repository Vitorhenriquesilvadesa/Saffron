use saffron_core::domain::request::{FormDataContent, FormDataPart, HttpRequest, RequestBody};
use saffron_core::domain::response::HttpResponse;
use std::collections::HashMap;
use std::io::Read;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Connection timeout")]
    Timeout,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    TlsError(String),

    #[error("Too many redirects")]
    TooManyRedirects,
}

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout_seconds: u64,
    pub follow_redirects: bool,
    pub max_redirects: usize,
    pub user_agent: Option<String>,
    pub accept_invalid_certs: bool,
    pub max_response_size: Option<usize>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            follow_redirects: true,
            max_redirects: 10,
            user_agent: Some(format!("Saffron/{}", env!("CARGO_PKG_VERSION"))),
            accept_invalid_certs: false,
            max_response_size: Some(100 * 1024 * 1024),
        }
    }
}

pub struct HttpClient {
    agent: ureq::Agent,
    config: HttpClientConfig,
}

impl HttpClient {
    pub fn new() -> Self {
        Self::with_config(HttpClientConfig::default())
    }

    pub fn with_config(config: HttpClientConfig) -> Self {
        let mut builder = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .redirects(config.max_redirects as u32);

        if let Some(ua) = &config.user_agent {
            builder = builder.user_agent(ua);
        }

        Self {
            agent: builder.build(),
            config,
        }
    }

    pub fn with_timeout(timeout_secs: u64) -> Self {
        let config = HttpClientConfig {
            timeout_seconds: timeout_secs,
            ..Default::default()
        };
        Self::with_config(config)
    }

    pub fn send(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let start = Instant::now();

        let method_str = request.method.as_str();
        let url = &request.url;

        let mut req = self.agent.request(method_str, url);

        for header in &request.headers {
            req = req.set(&header.name, &header.value);
        }

        if let Some(timeout) = request.timeout_seconds {
            req = req.timeout(Duration::from_secs(timeout));
        }

        let response = match &request.body {
            RequestBody::None => req.call(),
            RequestBody::Text(text) => {
                if request.get_header("Content-Type").is_none() {
                    req = req.set("Content-Type", "text/plain; charset=utf-8");
                }
                req.send_string(text)
            }
            RequestBody::Json(json) => {
                if request.get_header("Content-Type").is_none() {
                    req = req.set("Content-Type", "application/json; charset=utf-8");
                }
                req.send_string(json)
            }
            RequestBody::FormUrlEncoded(data) => {
                if request.get_header("Content-Type").is_none() {
                    req = req.set("Content-Type", "application/x-www-form-urlencoded");
                }
                let encoded = saffron_core::domain::request_body::encode_form_urlencoded(data);
                req.send_string(&encoded)
            }
            RequestBody::Binary(bytes) => {
                if request.get_header("Content-Type").is_none() {
                    req = req.set("Content-Type", "application/octet-stream");
                }
                req.send_bytes(bytes)
            }
            RequestBody::FormData(parts) => {
                return self.send_multipart(req, parts, start);
            }
        };

        self.process_response(response, start)
    }

    fn send_multipart(
        &self,
        mut req: ureq::Request,
        parts: &[FormDataPart],
        start: Instant,
    ) -> Result<HttpResponse, HttpError> {
        let boundary = format!(
            "----SaffronBoundary{}",
            chrono::Utc::now().timestamp_millis()
        );
        req = req.set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", boundary),
        );

        let mut body = Vec::new();

        for part in parts {
            body.extend_from_slice(b"--");
            body.extend_from_slice(boundary.as_bytes());
            body.extend_from_slice(b"\r\n");

            match &part.content {
                FormDataContent::Text(text) => {
                    body.extend_from_slice(
                        format!(
                            "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                            part.name
                        )
                        .as_bytes(),
                    );
                    body.extend_from_slice(text.as_bytes());
                }
                FormDataContent::File {
                    filename,
                    data,
                    content_type,
                } => {
                    body.extend_from_slice(
                        format!(
                            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                            part.name, filename
                        )
                        .as_bytes(),
                    );

                    if let Some(ct) = content_type {
                        body.extend_from_slice(format!("Content-Type: {}\r\n", ct).as_bytes());
                    }

                    body.extend_from_slice(b"\r\n");
                    body.extend_from_slice(data);
                }
            }
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"--\r\n");

        let response = req.send_bytes(&body);
        self.process_response(response, start)
    }

    fn process_response(
        &self,
        response: Result<ureq::Response, ureq::Error>,
        start: Instant,
    ) -> Result<HttpResponse, HttpError> {
        let elapsed = start.elapsed();

        match response {
            Ok(resp) => self.extract_response(resp, elapsed),
            Err(ureq::Error::Status(code, resp)) => {
                self.extract_response_with_code(resp, code, elapsed)
            }
            Err(ureq::Error::Transport(transport)) => {
                let error_msg = transport.to_string();
                if error_msg.contains("timeout") || error_msg.contains("timed out") {
                    Err(HttpError::Timeout)
                } else if error_msg.contains("certificate") || error_msg.contains("tls") {
                    Err(HttpError::TlsError(error_msg))
                } else {
                    Err(HttpError::NetworkError(error_msg))
                }
            }
        }
    }

    fn extract_response(
        &self,
        resp: ureq::Response,
        elapsed: Duration,
    ) -> Result<HttpResponse, HttpError> {
        let status = resp.status();
        let status_text = resp.status_text().to_string();
        let url = resp.get_url().to_string();

        let mut headers = HashMap::new();
        for name in resp.headers_names() {
            if let Some(value) = resp.header(&name) {
                headers.insert(name.clone(), value.to_string());
            }
        }

        let body = self.read_response_body(resp)?;

        Ok(HttpResponse::new(
            status,
            status_text,
            headers,
            body,
            elapsed,
            url,
        ))
    }

    fn extract_response_with_code(
        &self,
        resp: ureq::Response,
        code: u16,
        elapsed: Duration,
    ) -> Result<HttpResponse, HttpError> {
        let status_text = resp.status_text().to_string();
        let url = resp.get_url().to_string();

        let mut headers = HashMap::new();
        for name in resp.headers_names() {
            if let Some(value) = resp.header(&name) {
                headers.insert(name.clone(), value.to_string());
            }
        }

        let body = self.read_response_body(resp)?;

        Ok(HttpResponse::new(
            code,
            status_text,
            headers,
            body,
            elapsed,
            url,
        ))
    }

    fn read_response_body(&self, resp: ureq::Response) -> Result<Vec<u8>, HttpError> {
        let mut reader = resp.into_reader();
        let mut body = Vec::new();

        if let Some(max_size) = self.config.max_response_size {
            let mut limited_reader = reader.take(max_size as u64);
            limited_reader.read_to_end(&mut body)?;
        } else {
            reader.read_to_end(&mut body)?;
        }

        Ok(body)
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::get(url))
    }

    pub fn post(&self, url: &str, body: RequestBody) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::post(url).with_body(body))
    }

    pub fn put(&self, url: &str, body: RequestBody) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::put(url).with_body(body))
    }

    pub fn patch(&self, url: &str, body: RequestBody) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::patch(url).with_body(body))
    }

    pub fn delete(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::delete(url))
    }

    pub fn head(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::new(
            saffron_core::domain::request::HttpMethod::Head,
            url,
        ))
    }

    pub fn options(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.send(&HttpRequest::new(
            saffron_core::domain::request::HttpMethod::Options,
            url,
        ))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

pub mod helpers {
    use super::*;

    pub fn download_file(url: &str, path: &std::path::Path) -> Result<(), HttpError> {
        let client = HttpClient::new();
        let response = client.get(url)?;

        std::fs::write(path, response.body).map_err(HttpError::IoError)?;

        Ok(())
    }

    pub fn upload_file(
        url: &str,
        field_name: &str,
        file_path: &std::path::Path,
    ) -> Result<HttpResponse, HttpError> {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        let data = std::fs::read(file_path)?;

        let part = FormDataPart {
            name: field_name.to_string(),
            content: FormDataContent::File {
                filename,
                data,
                content_type: Some(guess_content_type(file_path)),
            },
        };

        let request = HttpRequest::post(url).with_body(RequestBody::FormData(vec![part]));

        let client = HttpClient::new();
        client.send(&request)
    }

    fn guess_content_type(path: &std::path::Path) -> String {
        match path.extension().and_then(|e| e.to_str()) {
            Some("txt") => "text/plain",
            Some("html") | Some("htm") => "text/html",
            Some("json") => "application/json",
            Some("xml") => "application/xml",
            Some("pdf") => "application/pdf",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("mp4") => "video/mp4",
            Some("zip") => "application/zip",
            _ => "application/octet-stream",
        }
        .to_string()
    }

    pub fn measure_request_time(
        request: &HttpRequest,
    ) -> Result<(HttpResponse, Duration), HttpError> {
        let start = Instant::now();
        let client = HttpClient::new();
        let response = client.send(request)?;
        let total_time = start.elapsed();
        Ok((response, total_time))
    }
}
