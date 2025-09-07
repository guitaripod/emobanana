use worker::{Request, Response, RouteContext, Result};
use crate::models::{TransformRequest, TransformResponse, TransformMetadata};
use crate::error::AppError;
use crate::providers::gemini::GeminiProvider;
use uuid::Uuid;

pub async fn handle_transform(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let env = ctx.env;
    let start_time = worker::Date::now().as_millis() as u64;
    let request_id = Uuid::new_v4().to_string();

    if let Err(rate_limit_error) = check_rate_limit(&req, &env).await {
        return AppError::from(rate_limit_error).to_response();
    }

    let transform_req: TransformRequest = match req.json().await {
        Ok(req) => req,
        Err(e) => return AppError::BadRequest(format!("Invalid JSON in request body: {}", e)).to_response(),
    };

    if transform_req.image.is_empty() {
        return AppError::BadRequest("Please upload an image to transform".to_string()).to_response();
    }

    if transform_req.emoji.is_empty() {
        return AppError::BadRequest("Please select an emoji for the transformation".to_string()).to_response();
    }

    // Validate image format and size
    if let Err(validation_error) = validate_image_data(&transform_req.image) {
        return AppError::from(validation_error).to_response();
    }

    let provider = match GeminiProvider::new(&env) {
        Ok(p) => p,
        Err(e) => return AppError::InternalError(format!("Failed to initialize Gemini provider: {}", e)).to_response(),
    };

    let image_data = if transform_req.image.starts_with("data:") {
        let parts: Vec<&str> = transform_req.image.split(',').collect();
        if parts.len() != 2 {
            return AppError::BadRequest("Invalid image data URL format. Expected 'data:mime/type;base64,data'".to_string()).to_response();
        }
        parts[1].to_string()
    } else {
        transform_req.image.clone()
    };

    let transformed_image = match provider.transform_image(&image_data, &transform_req.emoji).await {
        Ok(img) => img,
        Err(e) => {
            // Try to extract the specific AppError from the worker::Error
            let error_str = e.to_string();
            if let Some(msg) = error_str.strip_prefix("AppError::GeminiContentFiltered::") {
                return AppError::GeminiContentFiltered(msg.to_string()).to_response();
            }
            if let Some(msg) = error_str.strip_prefix("AppError::GeminiApiError::") {
                return AppError::GeminiApiError(msg.to_string()).to_response();
            }
            if let Some(msg) = error_str.strip_prefix("AppError::GeminiQuotaExceeded::") {
                return AppError::GeminiQuotaExceeded(msg.to_string()).to_response();
            }
            if let Some(msg) = error_str.strip_prefix("AppError::GeminiInvalidRequest::") {
                return AppError::GeminiInvalidRequest(msg.to_string()).to_response();
            }
            if let Some(msg) = error_str.strip_prefix("AppError::GeminiTimeout::") {
                return AppError::GeminiTimeout(msg.to_string()).to_response();
            }
            if let Some(msg) = error_str.strip_prefix("AppError::TransformationFailed::") {
                return AppError::TransformationFailed(msg.to_string()).to_response();
            }
            // Fallback to generic error
            return AppError::InternalError(format!("Image transformation failed: {}", e)).to_response();
        }
    };

    let processing_time_ms = worker::Date::now().as_millis() as u64 - start_time;

    if let Ok(kv) = env.kv("RATE_LIMIT_KV") {
        let client_ip = req.headers().get("CF-Connecting-IP")
            .or_else(|_| req.headers().get("X-Forwarded-For"))
            .or_else(|_| req.headers().get("X-Real-IP"))
            .unwrap_or_else(|_| Some("unknown".to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        if client_ip != "unknown" {
            let date_string = worker::Date::now().to_string();
            let today = if let Some(date_part) = date_string.split('T').next() {
                if let Some(date_only) = date_part.split(' ').next() {
                    date_only.to_string()
                } else {
                    date_part.to_string()
                }
            } else {
                "unknown".to_string()
            };
            let key = format!("rate_limit:{}:{}", client_ip, today);

            let current_count: u32 = match kv.get(&key).text().await {
                Ok(Some(count_str)) => count_str.parse().unwrap_or(0),
                Ok(None) => 0,
                Err(_) => 0,
            };

            let new_count = current_count + 1;
            if let Err(_) = kv.put(&key, new_count)?.execute().await {
            }
        }
    }

    let response = TransformResponse {
        transformed_image,
        metadata: TransformMetadata {
            processing_time_ms,
            model_version: "gemini-2.5-flash-image-preview".to_string(),
            request_id,
        },
    };

    Response::from_json(&response)
}

async fn check_rate_limit(req: &Request, env: &worker::Env) -> worker::Result<()> {
    let kv = match env.kv("RATE_LIMIT_KV") {
        Ok(kv) => kv,
        Err(_) => return Ok(()),
    };

    let client_ip = req.headers().get("CF-Connecting-IP")
        .or_else(|_| req.headers().get("X-Forwarded-For"))
        .or_else(|_| req.headers().get("X-Real-IP"))
        .unwrap_or_else(|_| Some("unknown".to_string()))
        .unwrap_or_else(|| "unknown".to_string());

    if client_ip == "unknown" {
        return Err(AppError::InternalError("Unable to determine client IP for rate limiting".to_string()).into());
    }

    let date_string = worker::Date::now().to_string();
    let today = if let Some(date_part) = date_string.split('T').next() {
        if let Some(date_only) = date_part.split(' ').next() {
            date_only.to_string()
        } else {
            date_part.to_string()
        }
    } else {
        "unknown".to_string()
    };
    let key = format!("rate_limit:{}:{}", client_ip, today);

    let current_count: u32 = match kv.get(&key).text().await {
        Ok(Some(count_str)) => count_str.parse().unwrap_or(0),
        Ok(None) => 0,
        Err(_) => 0,
    };

    const MAX_REQUESTS_PER_DAY: u32 = 5;

    if current_count >= MAX_REQUESTS_PER_DAY {
        return Err(AppError::RateLimitExceeded(format!(
            "Rate limit exceeded. You can make {} requests per day. Try again tomorrow.",
            MAX_REQUESTS_PER_DAY
        )).into());
    }

    Ok(())
}

fn validate_image_data(image_data: &str) -> worker::Result<()> {
    const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB

    // Check if it's a data URL
    if !image_data.starts_with("data:") {
        return Err(AppError::InvalidImageFormat(
            "Image must be provided as a data URL (data:image/...)"
                .to_string(),
        ).into());
    }

    // Parse data URL
    let parts: Vec<&str> = image_data.split(',').collect();
    if parts.len() != 2 {
        return Err(AppError::InvalidImageFormat(
            "Invalid image data URL format".to_string(),
        ).into());
    }

    let header = parts[0];
    let data = parts[1];

    // Validate MIME type
    if !header.contains("image/") {
        return Err(AppError::UnsupportedImageType(
            "Only image files are supported".to_string(),
        ).into());
    }

    // Check for supported formats
    let supported_formats = ["image/jpeg", "image/jpg", "image/png", "image/webp"];
    let is_supported = supported_formats
        .iter()
        .any(|format| header.contains(format));

    if !is_supported {
        return Err(AppError::UnsupportedImageType(
            "Unsupported image format. Please use JPEG, PNG, or WebP".to_string(),
        ).into());
    }

    // Check approximate size (base64 is ~33% larger than binary)
    let approximate_binary_size = (data.len() * 3) / 4;
    if approximate_binary_size > MAX_IMAGE_SIZE {
        return Err(AppError::ImageTooLarge(
            format!("Image is too large (max {}MB)", MAX_IMAGE_SIZE / (1024 * 1024)),
        ).into());
    }

    // Basic base64 validation
    if !is_valid_base64(data) {
        return Err(AppError::InvalidImageFormat(
            "Invalid base64 image data".to_string(),
        ).into());
    }

    Ok(())
}

fn is_valid_base64(s: &str) -> bool {
    // Remove padding characters and check if remaining chars are valid base64
    let s = s.trim_end_matches('=');
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
}