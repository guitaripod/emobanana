use crate::error::AppError;
use serde::{Deserialize, Serialize};
use worker::{console_log, Env, Fetch, Headers, Method, Request as WorkerRequest, Result};

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
            return Err(
                AppError::InternalError(format!("Gemini API error: {}", error_text)).into(),
            );
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
        console_log!("Starting transform_image with emoji: {}", emoji);

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

        console_log!("Calling Gemini API...");
        let response = self.call_gemini_api(gemini_request).await?;
        console_log!("Gemini API call completed");

        console_log!("Processing {} candidates", response.candidates.len());

        if response.candidates.is_empty() {
            return Err(AppError::InternalError("No response from Gemini".to_string()).into());
        }

        for (i, candidate) in response.candidates.iter().enumerate() {
            console_log!("Processing candidate {}", i);

            if let Some(finish_reason) = &candidate.finish_reason {
                console_log!("Finish reason: {}", finish_reason);
                if finish_reason == "PROHIBITED_CONTENT" {
                    return Err(AppError::InternalError(
                        "Content flagged as inappropriate. Try a different image or emoji."
                            .to_string(),
                    )
                    .into());
                }
            }

            if let Some(content) = &candidate.content {
                console_log!("Candidate has {} parts", content.parts.len());
                for (j, part) in content.parts.iter().enumerate() {
                    console_log!("Processing part {}", j);
                    match part {
                        GeminiPart::Image { inline_data } => {
                            console_log!("Found image! Size: {}", inline_data.data.len());
                            return Ok(inline_data.data.clone());
                        }
                        GeminiPart::Text { text } => {
                            console_log!("Found text: {}", &text[..text.len().min(100)]);
                        }
                    }
                }
            } else {
                console_log!("Candidate has no content");
            }
        }

        console_log!("No image found in response");
        Err(AppError::InternalError(
            "Gemini did not return an image. Try a different photo or emoji.".to_string(),
        )
        .into())
    }
}

