use worker::{Request, Response, RouteContext, Result};
use crate::models::{EmojiTransformRequest, ImageResponse, ImageData};
use crate::error::AppError;
use crate::providers::{self, UnifiedEditRequest};
use serde_json::json;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;

pub async fn transform_emoji(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let env = ctx.env;

    let transform_req: EmojiTransformRequest = match req.json().await {
        Ok(req) => req,
        Err(e) => return AppError::BadRequest(format!("Invalid request body: {}", e)).to_response(),
    };

    let provider = match providers::get_provider(&env) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    if !provider.get_supported_features().supports_edit {
        return AppError::BadRequest("Provider does not support image editing".to_string()).to_response();
    }

    let emoji_description = format!("Transform the facial expression of the creature in this image to match the emoji: {}", transform_req.emoji);

    let unified_request = UnifiedEditRequest {
        image: vec![transform_req.image],
        prompt: emoji_description,
        n: Some(transform_req.n),
    };

    let provider_response = match provider.edit_image(&unified_request).await {
        Ok(resp) => resp,
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("content_policy_violation") || error_msg.contains("moderation") {
                let custom_error = crate::models::ErrorResponse {
                    error: crate::models::ErrorDetail {
                        message: "The emoji transformation may violate content policies. Try a different emoji.".to_string(),
                        error_type: "moderation_error".to_string(),
                        param: None,
                        code: Some("moderation_blocked".to_string()),
                    }
                };
                return Response::from_json(&custom_error)
                    .map(|r| r.with_status(400));
            }
            return Err(e);
        }
    };

    let mut image_data_list = Vec::new();

    for (i, image_bytes) in provider_response.images.iter().enumerate() {
        let base64_string = BASE64.encode(&image_bytes.data);

        let revised_prompt = provider_response.revised_prompts
            .get(i)
            .and_then(|p| p.clone());

        image_data_list.push(ImageData {
            b64_json: Some(base64_string),
            url: None,
            revised_prompt,
        });
    }

    let response = ImageResponse {
        created: Utc::now().timestamp() as u64,
        data: image_data_list,
    };

    Response::from_json(&response)
}