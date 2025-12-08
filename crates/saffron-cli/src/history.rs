use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: u64,
    pub request: HistoryRequest,
    pub response: HistoryResponse,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body_preview: String,
}

impl HistoryEntry {
    pub fn new(
        request: HistoryRequest,
        response: HistoryResponse,
        duration_ms: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            request,
            response,
            duration_ms,
        }
    }

    pub fn format_timestamp(&self) -> String {
        let datetime = chrono::DateTime::from_timestamp(self.timestamp as i64, 0)
            .unwrap_or_else(|| chrono::DateTime::UNIX_EPOCH);
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl HistoryResponse {
    pub fn from_response(response: &saffron_core::domain::response::HttpResponse) -> Self {
        let body_preview = if let Ok(body_str) = std::str::from_utf8(&response.body) {
            if body_str.len() > 500 {
                format!("{}...", &body_str[..500])
            } else {
                body_str.to_string()
            }
        } else {
            format!("<binary data, {} bytes>", response.body.len())
        };

        Self {
            status: response.status,
            status_text: response.status_text.clone(),
            headers: response.headers.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            body_preview,
        }
    }
}
