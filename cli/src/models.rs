use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TransformRequest {
    pub image: String,
    pub emoji: String,
}

#[derive(Deserialize)]
pub struct TransformResponse {
    pub transformed_image: String,
    pub metadata: TransformMetadata,
}

#[derive(Deserialize)]
pub struct TransformMetadata {
    pub processing_time_ms: u64,
    pub model_version: String,
    pub request_id: String,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_request_serialization() {
        let request = TransformRequest {
            image: "data:image/png;base64,test".to_string(),
            emoji: "😊".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("data:image/png;base64,test"));
        assert!(json.contains("😊"));
    }

    #[test]
    fn test_transform_response_deserialization() {
        let json = r#"{
            "transformed_image": "data:image/png;base64,transformed",
            "metadata": {
                "processing_time_ms": 1500,
                "model_version": "1.0.0",
                "request_id": "req-123"
            }
        }"#;

        let response: TransformResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transformed_image, "data:image/png;base64,transformed");
        assert_eq!(response.metadata.processing_time_ms, 1500);
        assert_eq!(response.metadata.model_version, "1.0.0");
        assert_eq!(response.metadata.request_id, "req-123");
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{
            "error": {
                "message": "Invalid image format",
                "type": "validation_error",
                "param": "image",
                "code": "INVALID_FORMAT"
            }
        }"#;

        let error_response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(error_response.error.message, "Invalid image format");
        assert_eq!(error_response.error.error_type, "validation_error");
        assert_eq!(error_response.error.param, Some("image".to_string()));
        assert_eq!(error_response.error.code, Some("INVALID_FORMAT".to_string()));
    }
}