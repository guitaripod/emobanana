use clap::Parser;
use config::{Config, ConfigError};

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
pub struct Args {
    /// Path to the input image file (JPEG, PNG, etc.)
    #[arg(short, long, help = "Path to the image file containing a creature to transform")]
    pub image: String,

    /// Emoji to use for facial expression transformation
    #[arg(short, long, help = "Emoji representing the desired facial expression (e.g., 😊, 😢, 😠)")]
    pub emoji: String,

    /// Backend API URL
    #[arg(
        short,
        long,
        default_value = "https://emobanana.guitaripod.workers.dev",
        help = "URL of the Emobanana backend API"
    )]
    pub url: String,

    /// Output file path for the transformed image
    #[arg(
        short,
        long,
        default_value = "transformed.png",
        help = "Path where the transformed image will be saved"
    )]
    pub output: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub default_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_url: "https://emobanana.guitaripod.workers.dev".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("emobanana").required(false))
            .add_source(config::Environment::with_prefix("EMOBANANA"))
            .build()?;

        Ok(AppConfig {
            default_url: settings.get_string("default_url")
                .unwrap_or_else(|_| "https://emobanana.guitaripod.workers.dev".to_string()),
        })
    }
}