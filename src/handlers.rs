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
        return AppError::BadRequest("Image data is required and cannot be empty".to_string()).to_response();
    }

    if transform_req.emoji.is_empty() {
        return AppError::BadRequest("Emoji is required and cannot be empty".to_string()).to_response();
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
        Err(e) => return AppError::InternalError(format!("Image transformation failed: {}", e)).to_response(),
    };

    let processing_time_ms = worker::Date::now().as_millis() as u64 - start_time;

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

    let new_count = current_count + 1;
    if let Err(_) = kv.put(&key, new_count)?.execute().await {
        // KV update failed, but we don't want to block the user
    }

    Ok(())
}