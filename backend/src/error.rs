use worker::{Response, Result};
use crate::models::{ErrorResponse, ErrorDetail};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    InternalError(String),
    RateLimitExceeded(String),
    // Image processing specific errors
    InvalidImageFormat(String),
    ImageTooLarge(String),
    UnsupportedImageType(String),
    // Gemini API specific errors
    GeminiApiError(String),
    GeminiQuotaExceeded(String),
    GeminiContentFiltered(String),
    GeminiInvalidRequest(String),
    GeminiTimeout(String),
    // Processing errors
    ProcessingFailed(String),
    NoFacesDetected(String),
    TransformationFailed(String),
}

impl AppError {
    pub fn to_response(&self) -> Result<Response> {
        let (status, error_type, message, code, suggestion) = match self {
            AppError::BadRequest(msg) => (
                400,
                "invalid_request_error",
                msg.clone(),
                "bad_request",
                Some("Please check your input and try again.".to_string())
            ),
            AppError::InternalError(_) => (
                500,
                "internal_error",
                "An internal error occurred. Please try again later.".to_string(),
                "internal_error",
                Some("If the problem persists, please contact support.".to_string())
            ),
            AppError::RateLimitExceeded(msg) => (
                429,
                "rate_limit_error",
                msg.clone(),
                "rate_limit_exceeded",
                Some("Please wait until tomorrow to make more requests.".to_string())
            ),
            AppError::InvalidImageFormat(msg) => (
                400,
                "invalid_image_format",
                msg.clone(),
                "invalid_image_format",
                Some("Please upload a valid image file (JPEG, PNG, or WebP).".to_string())
            ),
            AppError::ImageTooLarge(msg) => (
                413,
                "image_too_large",
                msg.clone(),
                "image_too_large",
                Some("Please upload a smaller image (max 10MB).".to_string())
            ),
            AppError::UnsupportedImageType(msg) => (
                415,
                "unsupported_image_type",
                msg.clone(),
                "unsupported_image_type",
                Some("Please upload a JPEG, PNG, or WebP image.".to_string())
            ),
            AppError::GeminiApiError(_msg) => (
                502,
                "ai_service_error",
                "AI service temporarily unavailable. Please try again.".to_string(),
                "gemini_api_error",
                Some("The AI service is experiencing issues. Please try again in a few minutes.".to_string())
            ),
            AppError::GeminiQuotaExceeded(_msg) => (
                429,
                "ai_quota_exceeded",
                "AI service quota exceeded. Please try again later.".to_string(),
                "gemini_quota_exceeded",
                Some("The AI service is at capacity. Please try again in a few hours.".to_string())
            ),
            AppError::GeminiContentFiltered(_msg) => (
                451,
                "content_filtered",
                "The AI service flagged this content as inappropriate.".to_string(),
                "gemini_content_filtered",
                Some("Try using a different image or emoji that follows our content guidelines.".to_string())
            ),
            AppError::GeminiInvalidRequest(_msg) => (
                400,
                "ai_invalid_request",
                "Invalid request to AI service.".to_string(),
                "gemini_invalid_request",
                Some("Please check your image and emoji selection.".to_string())
            ),
            AppError::GeminiTimeout(_msg) => (
                504,
                "ai_timeout",
                "AI service took too long to respond.".to_string(),
                "gemini_timeout",
                Some("Please try again with a simpler image.".to_string())
            ),
            AppError::ProcessingFailed(_msg) => (
                422,
                "processing_failed",
                "Failed to process the image.".to_string(),
                "processing_failed",
                Some("Please try with a different image.".to_string())
            ),
            AppError::NoFacesDetected(_msg) => (
                422,
                "no_faces_detected",
                "No faces detected in the image.".to_string(),
                "no_faces_detected",
                Some("Please upload an image with a clear face.".to_string())
            ),
            AppError::TransformationFailed(_msg) => (
                422,
                "transformation_failed",
                "Failed to transform the facial expression.".to_string(),
                "transformation_failed",
                Some("Please try with a different emoji or image.".to_string())
            ),
        };

        let error_response = ErrorResponse {
            error: ErrorDetail {
                message,
                error_type: error_type.to_string(),
                param: None,
                code: Some(code.to_string()),
                suggestion,
            },
        };

        Response::from_json(&error_response)
            .map(|r| r.with_status(status))
    }
}

impl From<worker::Error> for AppError {
    fn from(err: worker::Error) -> Self {
        let error_str = err.to_string();

        if let Some(msg) = error_str.strip_prefix("AppError::BadRequest::") {
            return AppError::BadRequest(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::InternalError::") {
            return AppError::InternalError(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::RateLimitExceeded::") {
            return AppError::RateLimitExceeded(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::InvalidImageFormat::") {
            return AppError::InvalidImageFormat(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::ImageTooLarge::") {
            return AppError::ImageTooLarge(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::UnsupportedImageType::") {
            return AppError::UnsupportedImageType(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiApiError::") {
            return AppError::GeminiApiError(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiQuotaExceeded::") {
            return AppError::GeminiQuotaExceeded(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiApiError::") {
            return AppError::GeminiApiError(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiQuotaExceeded::") {
            return AppError::GeminiQuotaExceeded(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiContentFiltered::") {
            return AppError::GeminiContentFiltered(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiInvalidRequest::") {
            return AppError::GeminiInvalidRequest(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::GeminiTimeout::") {
            return AppError::GeminiTimeout(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::ProcessingFailed::") {
            return AppError::ProcessingFailed(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::NoFacesDetected::") {
            return AppError::NoFacesDetected(msg.to_string());
        }
        if let Some(msg) = error_str.strip_prefix("AppError::TransformationFailed::") {
            return AppError::TransformationFailed(msg.to_string());
        }

        AppError::InternalError(error_str)
    }
}

impl From<AppError> for worker::Error {
    fn from(err: AppError) -> Self {
        let encoded = match &err {
            AppError::BadRequest(msg) => format!("AppError::BadRequest::{}", msg),
            AppError::InternalError(msg) => format!("AppError::InternalError::{}", msg),
            AppError::RateLimitExceeded(msg) => format!("AppError::RateLimitExceeded::{}", msg),
            AppError::InvalidImageFormat(msg) => format!("AppError::InvalidImageFormat::{}", msg),
            AppError::ImageTooLarge(msg) => format!("AppError::ImageTooLarge::{}", msg),
            AppError::UnsupportedImageType(msg) => format!("AppError::UnsupportedImageType::{}", msg),
            AppError::GeminiApiError(msg) => format!("AppError::GeminiApiError::{}", msg),
            AppError::GeminiQuotaExceeded(msg) => format!("AppError::GeminiQuotaExceeded::{}", msg),
            AppError::GeminiContentFiltered(msg) => format!("AppError::GeminiContentFiltered::{}", msg),
            AppError::GeminiInvalidRequest(msg) => format!("AppError::GeminiInvalidRequest::{}", msg),
            AppError::GeminiTimeout(msg) => format!("AppError::GeminiTimeout::{}", msg),
            AppError::ProcessingFailed(msg) => format!("AppError::ProcessingFailed::{}", msg),
            AppError::NoFacesDetected(msg) => format!("AppError::NoFacesDetected::{}", msg),
            AppError::TransformationFailed(msg) => format!("AppError::TransformationFailed::{}", msg),
        };
        worker::Error::RustError(encoded)
    }
}