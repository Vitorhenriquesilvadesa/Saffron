use super::request::HttpRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub description: Option<String>,
    pub folders: Vec<Folder>,
    pub requests: Vec<SavedRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub name: String,
    pub description: Option<String>,
    pub requests: Vec<SavedRequest>,
    pub folders: Vec<Folder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(flatten)]
    pub request: SerializableRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub timeout_seconds: Option<u64>,
}

impl Collection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            folders: Vec::new(),
            requests: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_request(&mut self, request: SavedRequest) {
        self.requests.push(request);
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    pub fn find_request(&self, id: &str) -> Option<&SavedRequest> {
        self.requests
            .iter()
            .find(|r| r.id == id)
            .or_else(|| self.folders.iter().find_map(|f| f.find_request(id)))
    }
}

impl Folder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            requests: Vec::new(),
            folders: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_request(&mut self, request: SavedRequest) {
        self.requests.push(request);
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    pub fn find_request(&self, id: &str) -> Option<&SavedRequest> {
        self.requests
            .iter()
            .find(|r| r.id == id)
            .or_else(|| self.folders.iter().find_map(|f| f.find_request(id)))
    }
}

impl SavedRequest {
    pub fn new(id: impl Into<String>, name: impl Into<String>, request: &HttpRequest) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            request: SerializableRequest::from_request(request),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn to_http_request(&self) -> HttpRequest {
        self.request.to_http_request()
    }
}

impl SerializableRequest {
    pub fn from_request(request: &HttpRequest) -> Self {
        Self {
            method: request.method.as_str().to_string(),
            url: request.url.clone(),
            headers: request
                .headers
                .iter()
                .map(|h| (h.name.clone(), h.value.clone()))
                .collect(),
            body: match &request.body {
                super::request::RequestBody::None => None,
                super::request::RequestBody::Text(t) => Some(t.clone()),
                super::request::RequestBody::Json(j) => Some(j.clone()),
                _ => None,
            },
            timeout_seconds: request.timeout_seconds,
        }
    }

    pub fn to_http_request(&self) -> HttpRequest {
        let method = match self.method.to_uppercase().as_str() {
            "GET" => super::request::HttpMethod::Get,
            "POST" => super::request::HttpMethod::Post,
            "PUT" => super::request::HttpMethod::Put,
            "PATCH" => super::request::HttpMethod::Patch,
            "DELETE" => super::request::HttpMethod::Delete,
            "HEAD" => super::request::HttpMethod::Head,
            "OPTIONS" => super::request::HttpMethod::Options,
            _ => super::request::HttpMethod::Get,
        };

        let mut req = HttpRequest::new(method, self.url.clone());

        for (name, value) in &self.headers {
            req.add_header(name.clone(), value.clone());
        }

        if let Some(body) = &self.body {
            req.body = super::request::RequestBody::Text(body.clone());
        }

        if let Some(timeout) = self.timeout_seconds {
            req.timeout_seconds = Some(timeout);
        }

        req
    }
}
