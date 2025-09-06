# Emobanana

Emoji-based facial expression transformation using Gemini 2.5 Flash Image API.

## Overview

Emobanana allows users to upload an image containing a creature and transform its facial expression to match a selected emoji using Google's Gemini 2.5 Flash Image API.

## Features

- Transform creature facial expressions in images to match emojis
- Powered by Gemini 2.5 Flash Image API
- Simple REST API
- No authentication required
- Free tier usage (100 requests/day)

## API

### Transform Image

**POST** `/transform`

Transform the facial expression of a creature in an image to match an emoji.

**Request Body:**

```json
{
  "image": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQ...",
  "emoji": "😊"
}
```

**Response:**

```json
{
  "transformed_image": "/9j/4AAQSkZJRgABAQAAAQ..."
}
```

## Setup

### Local Development

1. Install dependencies:

   ```bash
   npm install
   ```

2. Create `.dev.vars` file with your Gemini API key:

   ```bash
   echo "GEMINI_API_KEY=your_api_key_here" > .dev.vars
   ```

3. Run locally:
   ```bash
   npx wrangler dev
   ```

### Production Deployment

1. Set the Gemini API key as a secret:

   ```bash
   npx wrangler secret put GEMINI_API_KEY
   ```

2. Deploy to Cloudflare Workers:

   ```bash
   npx wrangler deploy
   ```

3. Configure Gemini API key in `wrangler.toml`

4. Deploy to Cloudflare Workers:
   ```bash
   npx wrangler deploy
   ```

## CLI Tool

A command-line tool is available for testing the backend API locally.

### Build the CLI

```bash
cargo build --bin emobanana-cli
```

### Usage

```bash
cargo run --bin emobanana-cli -- --image path/to/image.jpg --emoji 😊
```

### CLI Options

- `-i, --image <IMAGE>`: Path to the input image file
- `-e, --emoji <EMOJI>`: Emoji to use for transformation
- `-u, --url <URL>`: Backend API URL (default: https://emobanana.guitaripod.workers.dev)
- `-o, --output <OUTPUT>`: Output file path for the transformed image (default: transformed.png)

### Example

```bash
# Transform a cat image to have a happy expression
cargo run --bin emobanana-cli -- --image cat.jpg --emoji 😺 --output happy_cat.png

# Use a different backend URL
cargo run --bin emobanana-cli -- --image dog.jpg --emoji 🐕 --url https://emobanana.guitaripod.workers.dev
```

## Development

1. Install wrangler CLI
2. Run locally:
    ```bash
    npx wrangler dev
    ```

## License

MIT
