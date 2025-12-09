pub mod insomnia;

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type ImportResult<T> = Result<T, ImportError>;

/// Generic imported collection structure (format-agnostic)
#[derive(Debug, Clone)]
pub struct ImportedCollection {
    pub name: String,
    pub description: Option<String>,
    pub requests: Vec<ImportedRequest>,
}

#[derive(Debug, Clone)]
pub struct ImportedRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

/// Trait for importing collections from external formats
pub trait ImportFormat {
    /// The source format type (e.g., InsomniaV4, PostmanV2)
    type Source;

    /// Validates if the input can be parsed by this importer
    fn can_import(content: &str) -> bool;

    /// Parses the content into the source format
    fn parse(content: &str) -> ImportResult<Self::Source>;

    /// Converts the source format into generic imported collections
    fn convert(source: Self::Source) -> ImportResult<Vec<ImportedCollection>>;

    /// Full import pipeline: parse and convert
    fn import(content: &str) -> ImportResult<Vec<ImportedCollection>> {
        let source = Self::parse(content)?;
        Self::convert(source)
    }
}

/// Auto-detect and import from multiple formats
pub fn auto_import(content: &str) -> ImportResult<Vec<ImportedCollection>> {
    // Try Insomnia first
    if insomnia::InsomniaImporter::can_import(content) {
        return insomnia::InsomniaImporter::import(content);
    }

    // Add more formats here as we implement them
    // if postman::PostmanImporter::can_import(content) {
    //     return postman::PostmanImporter::import(content);
    // }

    Err(ImportError::InvalidFormat(
        "Unknown format. Supported: Insomnia v4".into(),
    ))
}
