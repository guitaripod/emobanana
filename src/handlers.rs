use worker::{Request, Response, RouteContext, Result};
use crate::models::{TransformRequest, TransformResponse};
use crate::error::AppError;
use crate::providers::gemini::GeminiProvider;

pub async fn handle_transform(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let env = ctx.env;

    let transform_req: TransformRequest = match req.json().await {
        Ok(req) => req,
        Err(e) => return AppError::BadRequest(format!("Invalid request body: {}", e)).to_response(),
    };

    if transform_req.image.is_empty() {
        return AppError::BadRequest("Image data is required".to_string()).to_response();
    }

    if transform_req.emoji.is_empty() {
        return AppError::BadRequest("Emoji is required".to_string()).to_response();
    }

    let provider = match GeminiProvider::new(&env) {
        Ok(p) => p,
        Err(e) => return AppError::InternalError(format!("Failed to initialize Gemini provider: {}", e)).to_response(),
    };

    let image_data = if transform_req.image.starts_with("data:") {
        let parts: Vec<&str> = transform_req.image.split(',').collect();
        if parts.len() != 2 {
            return AppError::BadRequest("Invalid image data URL".to_string()).to_response();
        }
        parts[1].to_string()
    } else {
        transform_req.image.clone()
    };

    let transformed_image = match provider.transform_image(&image_data, &transform_req.emoji).await {
        Ok(img) => img,
        Err(e) => return AppError::InternalError(format!("Transformation failed: {}", e)).to_response(),
    };

    let response = TransformResponse {
        transformed_image,
    };

    Response::from_json(&response)
}