use worker::*;

mod models;
mod error;
mod handlers;
mod providers;

use handlers::handle_transform;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::ok(r#"Emobanana - Emoji-based facial expression transformation

API Documentation: /docs
OpenAPI Specification: /openapi.yaml
Privacy Policy: /privacy-policy"#)
        })
        .get("/docs", |_, _| {
            Response::ok(include_str!("swagger-ui.html"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/html").unwrap();
                    r
                })
        })
        .get("/privacy-policy", |_, _| {
            Response::ok(include_str!("privacy_policy.html"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/html").unwrap();
                    r
                })
        })
        .get("/docs/", |req, _| {
            let url = req.url().unwrap();
            let base = format!("{}://{}", url.scheme(), url.host().unwrap());
            Response::redirect(format!("{}/docs", base).parse().unwrap())
        })
        .get("/openapi.yaml", |_, _| {
            Response::ok(include_str!("../openapi.yaml"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/yaml").unwrap();
                    r
                })
        })
        .post_async("/transform", handle_transform)
        .run(req, env)
        .await
}