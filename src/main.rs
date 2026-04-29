use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "rust-web-api is running",
        "version": "0.1.0"
    }))
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "healthy"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
