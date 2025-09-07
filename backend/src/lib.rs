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
        .get("/_astro/index.IQ3vksgp.css", |_, _| {
            Response::ok(include_str!("../_astro/index.IQ3vksgp.css"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/css").unwrap();
                    r
                })
        })
        .get("/_astro/EmoBananaApp.BxDkulSD.js", |_, _| {
            Response::ok(include_str!("../_astro/EmoBananaApp.BxDkulSD.js"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/javascript").unwrap();
                    r
                })
        })
        .get("/_astro/client.DVxemvf8.js", |_, _| {
            Response::ok(include_str!("../_astro/client.DVxemvf8.js"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/javascript").unwrap();
                    r
                })
        })
        .get("/_astro/index.RH_Wq4ov.js", |_, _| {
            Response::ok(include_str!("../_astro/index.RH_Wq4ov.js"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/javascript").unwrap();
                    r
                })
        })
        .get("/_astro/index.nCNIo1WA.css", |_, _| {
            Response::ok(include_str!("../_astro/index.nCNIo1WA.css"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/css").unwrap();
                    r
                })
        })
        .get("/_astro/EmoBananaApp.aJDWk7M7.js", |_, _| {
            Response::ok(include_str!("../_astro/EmoBananaApp.aJDWk7M7.js"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/javascript").unwrap();
                    r
                })
        })
        .run(req, env)
        .await;
    
    match response {
        Ok(resp) => add_cors_headers(resp),
        Err(e) => Err(e)
    }
}