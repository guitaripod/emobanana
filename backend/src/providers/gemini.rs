use crate::error::AppError;
use serde::{Deserialize, Serialize};
use worker::{Env, Fetch, Headers, Method, Request as WorkerRequest, Result};

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-image-preview:generateContent";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum GeminiPart {
    Text {
        text: String,
    },
    Image {
        #[serde(rename = "inlineData")]
        inline_data: InlineData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContent>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

pub struct GeminiProvider {
    api_key: String,
}

impl GeminiProvider {
    pub fn new(env: &Env) -> Result<Self> {
        let api_key = if let Ok(secret) = env.secret("GEMINI_API_KEY") {
            secret.to_string()
        } else if let Ok(var) = env.var("GEMINI_API_KEY") {
            var.to_string()
        } else {
            return Err(worker::Error::RustError(
                "GEMINI_API_KEY not configured as secret or environment variable".to_string(),
            ));
        };

        Ok(Self { api_key })
    }

    async fn call_gemini_api(&self, request_body: GeminiRequest) -> Result<GeminiResponse> {
        let headers = Headers::new();
        headers.set("x-goog-api-key", &self.api_key)?;
        headers.set("Content-Type", "application/json")?;

        let mut init = worker::RequestInit::new();
        init.with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(worker::wasm_bindgen::JsValue::from_str(
                &serde_json::to_string(&request_body)?,
            )));

        let request = WorkerRequest::new_with_init(GEMINI_API_URL, &init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() >= 400 {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            let error = match response.status_code() {
                400 => AppError::GeminiInvalidRequest(format!("Invalid request to Gemini API: {}", error_text)),
                401 | 403 => AppError::GeminiApiError("Authentication failed with Gemini API".to_string()),
                429 => AppError::GeminiQuotaExceeded("Gemini API quota exceeded".to_string()),
                500..=599 => AppError::GeminiApiError(format!("Gemini API server error: {}", error_text)),
                _ => AppError::GeminiApiError(format!("Gemini API error: {}", error_text)),
            };

            return Err(error.into());
        }

        let response_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to get response text".to_string());

        let gemini_response: GeminiResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                AppError::InternalError(format!(
                    "Failed to parse Gemini response: {}. Response: {}",
                    e, response_text
                ))
            })?;

        Ok(gemini_response)
    }

    pub async fn transform_image(&self, image_data: &str, emoji: &str) -> Result<String> {
        const MAX_RETRIES: u32 = 3;
        let mut last_error: Option<AppError> = None;

        for attempt in 1..=MAX_RETRIES {
            let result = self.try_transform_once(image_data, emoji).await;

            match result {
                Ok(image_data) => return Ok(image_data),
                Err(e) => {
                    let error_msg = e.to_string();

                    if error_msg.contains("PROHIBITED_CONTENT") || error_msg.contains("Content flagged as inappropriate") {
                        last_error = Some(AppError::InternalError(error_msg));
                        if attempt < MAX_RETRIES {
                            continue;
                        }
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or(AppError::InternalError("Failed after retries".to_string())).into())
    }

    async fn try_transform_once(&self, image_data: &str, emoji: &str) -> Result<String> {
        let prompt = format!("Please edit this photo by changing the person's facial expression to look more like this emoji: {}. Make the facial expression match the mood of the emoji while keeping everything else the same.", emoji);

        let gemini_request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![
                    GeminiPart::Text { text: prompt },
                    GeminiPart::Image {
                        inline_data: InlineData {
                            mime_type: "image/jpeg".to_string(),
                            data: image_data.to_string(),
                        },
                    },
                ],
            }],
        };

        let response = self.call_gemini_api(gemini_request).await?;

        if response.candidates.is_empty() {
            return Err(AppError::InternalError("No response from Gemini".to_string()).into());
        }

        for candidate in response.candidates.iter() {
            if let Some(finish_reason) = &candidate.finish_reason {
                match finish_reason.as_str() {
                    "PROHIBITED_CONTENT" => {
                        return Err(AppError::GeminiContentFiltered("Content was flagged as inappropriate by Gemini".to_string()).into());
                    }
                    "SAFETY" => {
                        return Err(AppError::GeminiContentFiltered("Content violated safety guidelines".to_string()).into());
                    }
                    "RECITATION" => {
                        return Err(AppError::TransformationFailed("Gemini could not process this type of content".to_string()).into());
                    }
                    "OTHER" => {
                        return Err(AppError::TransformationFailed("Gemini encountered an unknown error".to_string()).into());
                    }
                    _ => {}
                }
            }

            if let Some(content) = &candidate.content {
                for part in content.parts.iter() {
                    match part {
                        GeminiPart::Image { inline_data } => {
                            return Ok(inline_data.data.clone());
                        }
                        GeminiPart::Text { .. } => {}
                    }
                }
            }
        }
        Err(AppError::TransformationFailed(
            "Gemini did not return an image. Try a different photo or emoji.".to_string(),
        )
        .into())
    }
}

