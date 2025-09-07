use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRequest {
    pub image: String,
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformResponse {
    pub transformed_image: String,
    pub metadata: TransformMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformMetadata {
    pub processing_time_ms: u64,
    pub model_version: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
    pub suggestion: Option<String>,
}