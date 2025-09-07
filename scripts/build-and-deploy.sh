#!/bin/bash

set -e

echo "🚀 Building Emobanana..."

# Build the web app
echo "📦 Building web app..."
cd web
npm run build
cd ..

# Extract actual filenames from the build output
echo "🔍 Extracting asset filenames..."
CSS_FILES=$(ls web/dist/_astro/*.css 2>/dev/null)
JS_FILES=$(ls web/dist/_astro/*.js 2>/dev/null)

# Update backend source code with correct filenames
echo "📝 Updating backend with correct asset filenames..."
cd backend/src

# Create a temporary version of lib.rs with updated asset routes
cp lib.rs lib.rs.bak

# Replace the asset routes with the correct filenames
# This is a simple approach - replace the hardcoded routes with the actual files
cat > lib.rs.new << 'EOF'
use worker::*;

mod models;
mod error;
mod handlers;
mod providers;

use handlers::handle_transform;

fn add_cors_headers(mut response: Response) -> Result<Response> {
    response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
    response.headers_mut().set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    response.headers_mut().set("Access-Control-Allow-Headers", "Content-Type")?;
    response.headers_mut().set("Access-Control-Max-Age", "86400")?;
    Ok(response)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    if req.method() == Method::Options {
        return add_cors_headers(Response::empty()?);
    }

    let router = Router::new();

    let response = router
        .get("/api/docs", |_, _| {
            Response::ok(include_str!("../swagger-ui.html"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/html").unwrap();
                    r
                })
        })
        .get("/api/privacy-policy", |_, _| {
            Response::ok(include_str!("../privacy_policy.html"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/html").unwrap();
                    r
                })
        })
        .get("/api/docs/", |req, _| {
            let url = req.url().unwrap();
            let base = format!("{}://{}", url.scheme(), url.host().unwrap());
            Response::redirect(format!("{}/api/docs", base).parse().unwrap())
        })
        .get("/api/openapi.yaml", |_, _| {
            Response::ok(include_str!("../openapi.yaml"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/yaml").unwrap();
                    r
                })
        })
        .post_async("/api/transform", handle_transform)
        .get("/", |_, _| {
            Response::ok(include_str!("../index.html"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/html").unwrap();
                    r
                })
        })
        .get("/favicon.svg", |_, _| {
            Response::ok(include_str!("../favicon.svg"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "image/svg+xml").unwrap();
                    r
                })
        })
EOF

# Add routes for each CSS file
for css_file in $CSS_FILES; do
    filename=$(basename "$css_file")
    echo "        .get(\"/_astro/$filename\", |_, _| {" >> lib.rs.new
    echo "            Response::ok(include_str!(\"../_astro/$filename\"))" >> lib.rs.new
    echo "                .map(|mut r| {" >> lib.rs.new
    echo "                    r.headers_mut().set(\"Content-Type\", \"text/css\").unwrap();" >> lib.rs.new
    echo "                    r" >> lib.rs.new
    echo "                })" >> lib.rs.new
    echo "        })" >> lib.rs.new
done

# Add routes for each JS file
for js_file in $JS_FILES; do
    filename=$(basename "$js_file")
    echo "        .get(\"/_astro/$filename\", |_, _| {" >> lib.rs.new
    echo "            Response::ok(include_str!(\"../_astro/$filename\"))" >> lib.rs.new
    echo "                .map(|mut r| {" >> lib.rs.new
    echo "                    r.headers_mut().set(\"Content-Type\", \"application/javascript\").unwrap();" >> lib.rs.new
    echo "                    r" >> lib.rs.new
    echo "                })" >> lib.rs.new
    echo "        })" >> lib.rs.new
done

# Finish the file
cat >> lib.rs.new << 'EOF'
        .run(req, env)
        .await;
    
    match response {
        Ok(resp) => add_cors_headers(resp),
        Err(e) => Err(e)
    }
}
EOF

# Replace the original file
mv lib.rs.new lib.rs

cd ../..

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