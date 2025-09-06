mod cli;
mod api;
mod models;
mod utils;
mod error;

use clap::Parser;
use tracing::info;
use tracing_subscriber;

use crate::cli::Args;
use crate::api::ApiClient;
use crate::models::TransformRequest;
use crate::utils::{load_image_as_base64, save_base64_image};
use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    info!("Starting image transformation");
    info!("Image: {}", args.image);
    info!("Emoji: {}", args.emoji);
    info!("Backend URL: {}", args.url);

    let image_data = load_image_as_base64(&args.image)?;

    let request = TransformRequest {
        image: image_data,
        emoji: args.emoji.clone(),
    };

    let api_client = ApiClient::new(args.url.clone());

    let response = api_client.transform_image(request).await?;

    save_base64_image(&response.transformed_image, &args.output)?;

    info!("Transformation completed successfully!");
    info!("Request ID: {}", response.metadata.request_id);
    info!("Processing time: {}ms", response.metadata.processing_time_ms);
    info!("Model version: {}", response.metadata.model_version);
    info!("Transformed image saved to: {}", args.output);

    Ok(())
}