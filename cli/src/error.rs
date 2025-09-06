use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmobananaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("API error: {0}")]
    Api(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid image format")]
    InvalidImageFormat,
}

pub type Result<T> = std::result::Result<T, EmobananaError>;