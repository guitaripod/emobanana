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
        .get("/og-banner.png", |_, _| {
            Response::from_bytes(include_bytes!("../og-banner.png").to_vec())
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "image/png").unwrap();
                    r
                })
        })
        .get("/apple-touch-icon.png", |_, _| {
            Response::from_bytes(include_bytes!("../apple-touch-icon.png").to_vec())
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "image/png").unwrap();
                    r
                })
        })
        .get("/favicon-32x32.png", |_, _| {
            Response::from_bytes(include_bytes!("../favicon-32x32.png").to_vec())
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "image/png").unwrap();
                    r
                })
        })
        .get("/favicon-16x16.png", |_, _| {
            Response::from_bytes(include_bytes!("../favicon-16x16.png").to_vec())
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "image/png").unwrap();
                    r
                })
        })
        .get("/site.webmanifest", |_, _| {
            Response::ok(include_str!("../site.webmanifest"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "application/manifest+json").unwrap();
                    r
                })
        })
        .get("/_astro/index.CWD5hzu9.css", |_, _| {
            Response::ok(include_str!("../_astro/index.CWD5hzu9.css"))
                .map(|mut r| {
                    r.headers_mut().set("Content-Type", "text/css").unwrap();
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
        .get("/_astro/EmoBananaApp.Cj1Nby82.js", |_, _| {
            Response::ok(include_str!("../_astro/EmoBananaApp.Cj1Nby82.js"))
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
        .run(req, env)
        .await;
    
    match response {
        Ok(resp) => add_cors_headers(resp),
        Err(e) => Err(e)
    }
}
