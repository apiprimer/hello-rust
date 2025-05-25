use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct RequestData {
    name: String,
}

#[derive(Serialize)]
struct ResponseData {
    from: String,
    message: String,
    status: bool,
}

// GET handler
async fn get_hello() -> impl Responder {
    let response = ResponseData {
        from: "Rust - Actix".to_string(),
        message: "Hello, from Rust".to_string(),
        status: true,
    };

    HttpResponse::Ok().json(response)
}

// POST handler
async fn post_hello(data: web::Json<RequestData>) -> impl Responder {
    let response = ResponseData {
        from: "Rust - Actix".to_string(),
        message: format!("Hello, {}!", data.name),
        status: true,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port); // IMPORTANT for Docker!

    println!("Server running on http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_hello))
            .route("/", web::post().to(post_hello))
    })
    .bind(addr)?
    .run()
    .await
}
