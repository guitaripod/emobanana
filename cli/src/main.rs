use clap::Parser;
use std::fs;
use std::path::Path;
use reqwest::Client;
use base64::Engine;

#[derive(Parser)]
#[command(
    name = "emobanana-cli",
    version,
    about = "Transform creature facial expressions using emoji prompts",
    long_about = "A command-line tool to test the Emobanana API by transforming creature facial expressions in images to match emoji emotions.

EXAMPLES:
    # Transform a cat to look happy
    emobanana-cli -i cat.jpg -e 😊

    # Use custom output filename
    emobanana-cli --image dog.png --emoji 😢 --output sad_dog.png

    # Test against local development server
    emobanana-cli -i bird.jpg -e 😠 -u http://localhost:8787"
)]
struct Args {
    /// Path to the input image file (JPEG, PNG, etc.)
    #[arg(short, long, help = "Path to the image file containing a creature to transform")]
    image: String,

    /// Emoji to use for facial expression transformation
    #[arg(short, long, help = "Emoji representing the desired facial expression (e.g., 😊, 😢, 😠)")]
    emoji: String,

    /// Backend API URL
    #[arg(
        short,
        long,
        default_value = "https://emobanana.guitaripod.workers.dev",
        help = "URL of the Emobanana backend API"
    )]
    url: String,

    /// Output file path for the transformed image
    #[arg(
        short,
        long,
        default_value = "transformed.png",
        help = "Path where the transformed image will be saved"
    )]
    output: String,
}

#[derive(serde::Serialize)]
struct TransformRequest {
    image: String,
    emoji: String,
}

#[derive(serde::Deserialize)]
struct TransformResponse {
    transformed_image: String,
    metadata: TransformMetadata,
}

#[derive(serde::Deserialize)]
struct TransformMetadata {
    processing_time_ms: u64,
    model_version: String,
    request_id: String,
}

#[derive(serde::Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(serde::Deserialize)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    error_type: String,
    #[allow(dead_code)]
    param: Option<String>,
    #[allow(dead_code)]
    code: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !Path::new(&args.image).exists() {
        eprintln!("Error: Image file '{}' does not exist", args.image);
        std::process::exit(1);
    }

    let image_data = fs::read(&args.image)?;
    let base64_image = base64::engine::general_purpose::STANDARD.encode(image_data);
    let data_url = format!("data:image/png;base64,{}", base64_image);

    let request = TransformRequest {
        image: data_url,
        emoji: args.emoji.clone(),
    };

    println!("Sending transformation request...");
    println!("Image: {}", args.image);
    println!("Emoji: {}", args.emoji);
    println!("Backend URL: {}", args.url);

    let client = Client::new();
    let response = client
        .post(format!("{}/transform", args.url))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let transform_response: TransformResponse = response.json().await?;
        println!("Transformation successful!");
        println!("Request ID: {}", transform_response.metadata.request_id);
        println!("Processing time: {}ms", transform_response.metadata.processing_time_ms);
        println!("Model version: {}", transform_response.metadata.model_version);

        let base64_data = if transform_response.transformed_image.starts_with("data:") {
            let parts: Vec<&str> = transform_response.transformed_image.split(',').collect();
            if parts.len() == 2 {
                parts[1]
            } else {
                &transform_response.transformed_image
            }
        } else {
            &transform_response.transformed_image
        };

        let image_data = base64::engine::general_purpose::STANDARD.decode(base64_data)?;
        fs::write(&args.output, image_data)?;
        println!("Transformed image saved to: {}", args.output);
    } else {
        let error_response: ErrorResponse = response.json().await?;
        eprintln!("Error: {}", error_response.error.message);
        std::process::exit(1);
    }

    Ok(())
}