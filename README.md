# Emobanana

Emoji-based facial expression transformation using Gemini 2.5 Flash Image API.

## Project Structure

```
emobanana/
├── cli/           # Command-line tool for testing the API
├── backend/       # Cloudflare Worker backend
└── Cargo.toml     # Workspace configuration
```

## Features

- Transform creature facial expressions in images to match emojis
- Powered by Gemini 2.5 Flash Image API
- Simple REST API
- No authentication required
- Rate limited to 5 requests per day per IP address

## API

### Transform Image

**POST** `/transform`

```json
{
  "image": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQ...",
  "emoji": "😊"
}
```

**Response:**
```json
{
  "transformed_image": "/9j/4AAQSkZJRgABAQAAAQ...",
  "metadata": {
    "processing_time_ms": 2500,
    "model_version": "gemini-2.5-flash-image-preview",
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

## Setup

### Backend (Cloudflare Worker)

1. Install dependencies:
   ```bash
   npm install
   ```

2. Set Gemini API key:
   ```bash
   npx wrangler secret put GEMINI_API_KEY
   ```

3. Deploy:
   ```bash
   npx wrangler deploy
   ```

### CLI Tool

1. Build:
   ```bash
   cargo build -p emobanana-cli
   ```

2. Run:
   ```bash
   cargo run -p emobanana-cli -- -i cat.jpg -e 😊
   ```

3. Get help:
   ```bash
   cargo run -p emobanana-cli -- --help
   ```

## Development

- **Backend**: `cd backend && cargo build`
- **CLI**: `cd cli && cargo build`
- **Both**: `cargo build`

## License

MIT
