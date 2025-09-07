#!/bin/bash

set -e

echo "🚀 Building Emobanana..."

# Build the web app
echo "📦 Building web app..."
cd web
npm run build
cd ..

# Copy web assets to backend for Worker to serve
echo "📋 Copying web assets to backend..."
cp -r web/dist/* backend/

# Build the backend
echo "🔧 Building backend..."
cd backend
cargo install -q worker-build
~/.cargo/bin/worker-build --release --no-opt
cd ..

# Deploy to Cloudflare
echo "☁️  Deploying to Cloudflare Workers..."
wrangler deploy

echo "✅ Deployment complete!"
echo "🌐 Your app is live at: https://emobanana.guitaripod.workers.dev"