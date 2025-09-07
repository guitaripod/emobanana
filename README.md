# Emobanana

Emoji-based facial expression transformation using Gemini 2.5 Flash Image API.

## Project Structure

```
emobanana/
├── cli/           # Command-line tool for testing the API
├── backend/       # Cloudflare Worker backend
├── web/           # Astro web application frontend
├── scripts/       # Build and deployment scripts
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

**POST** `/api/transform`

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

### Full Application (Web + Backend)

1. Install dependencies:
   ```bash
   npm install
   cd web && npm install && cd ..
   ```

2. Set Gemini API key:
   ```bash
   npx wrangler secret put GEMINI_API_KEY
   ```

3. Build and deploy:
   ```bash
   ./scripts/build-and-deploy.sh
   ```

### Manual Deployment

1. Build web app:
   ```bash
   cd web && npm run build && cd ..
   ```

2. Build backend:
   ```bash
   cd backend && cargo install -q worker-build && ~/.cargo/bin/worker-build --release --no-opt && cd ..
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

- **Web App**: `cd web && npm run dev`
- **Backend**: `cd backend && cargo build`
- **CLI**: `cd cli && cargo build`
- **All**: `cargo build`

## License

MIT
