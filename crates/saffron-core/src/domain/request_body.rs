use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Json,
    Xml,
    Text,
    FormUrlEncoded,
    FormData,
    Binary,
    Custom(String),
}

impl ContentType {
    pub fn as_mime_type(&self) -> &str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Xml => "application/xml",
            ContentType::Text => "text/plain",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::FormData => "multipart/form-data",
            ContentType::Binary => "application/octet-stream",
            ContentType::Custom(mime) => mime.as_str(),
        }
    }

    pub fn from_mime_type(mime: &str) -> Self {
        match mime.to_lowercase().as_str() {
            s if s.contains("application/json") => ContentType::Json,
            s if s.contains("application/xml") || s.contains("text/xml") => ContentType::Xml,
            s if s.contains("text/plain") => ContentType::Text,
            s if s.contains("application/x-www-form-urlencoded") => ContentType::FormUrlEncoded,
            s if s.contains("multipart/form-data") => ContentType::FormData,
            s if s.contains("application/octet-stream") => ContentType::Binary,
            _ => ContentType::Custom(mime.to_string()),
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_mime_type())
    }
}

pub fn encode_form_urlencoded(data: &HashMap<String, String>) -> String {
    data.iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

