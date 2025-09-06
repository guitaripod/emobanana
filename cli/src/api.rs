use reqwest::Client;
use tracing::{info, error};

use crate::error::{EmobananaError, Result};
use crate::models::{TransformRequest, TransformResponse, ErrorResponse};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn transform_image(&self, request: TransformRequest) -> Result<TransformResponse> {
        let url = format!("{}/transform", self.base_url);
        info!("Sending transformation request to {}", url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let transform_response: TransformResponse = response.json().await?;
            info!("Transformation successful, request ID: {}", transform_response.metadata.request_id);
            Ok(transform_response)
        } else {
            let error_response: ErrorResponse = response.json().await?;
            error!("API error: {}", error_response.error.message);
            Err(EmobananaError::Api(error_response.error.message))
        }
    }
}