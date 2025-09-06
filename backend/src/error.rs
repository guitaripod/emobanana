use worker::{Response, Result};
use crate::models::{ErrorResponse, ErrorDetail};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    InternalError(String),
    RateLimitExceeded(String),
}

impl AppError {
    pub fn to_response(&self) -> Result<Response> {
        let (status, error_type, message, code) = match self {
            AppError::BadRequest(msg) => (400, "invalid_request_error", msg.clone(), "bad_request"),
            AppError::InternalError(_) => (500, "internal_error", "An internal error occurred. Please try again later.".to_string(), "internal_error"),
            AppError::RateLimitExceeded(msg) => (429, "rate_limit_error", msg.clone(), "rate_limit_exceeded"),
        };

        let error_response = ErrorResponse {
            error: ErrorDetail {
                message,
                error_type: error_type.to_string(),
                param: None,
                code: Some(code.to_string()),
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

        AppError::InternalError(error_str)
    }
}

impl From<AppError> for worker::Error {
    fn from(err: AppError) -> Self {
        let encoded = match &err {
            AppError::BadRequest(msg) => format!("AppError::BadRequest::{}", msg),
            AppError::InternalError(msg) => format!("AppError::InternalError::{}", msg),
            AppError::RateLimitExceeded(msg) => format!("AppError::RateLimitExceeded::{}", msg),
        };
        worker::Error::RustError(encoded)
    }
}